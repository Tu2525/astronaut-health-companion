import { useEffect, useState } from "react";
import { formatEarth, wallTime } from "../../../clock";
import type { Event } from "../../../contract/Event";
import type { CrewStore } from "../../../store";
import { Card } from "../../../ui/Card";
import { API_URL, delayLabel, flushOutbox, readOutbox } from "./outbox";
import "./comms.css";

type Model = ReturnType<CrewStore["getSnapshot"]>;
const POLL_MS = 4000;
const SKIPS: [number, string][] = [[300, "5 min"], [3600, "1 h"], [21600, "6 h"]];

function countdown(ms: number) {
  const s = Math.ceil(ms / 1000);
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
}

export function CommsPage({ model, store }: { model: Model; store: CrewStore }) {
  const { settings, events } = model;
  const [link, setLink] = useState<"checking" | "up" | "down">(API_URL ? "checking" : "down");
  const [outbox, setOutbox] = useState(readOutbox);

  useEffect(() => {
    if (!API_URL) return;
    let live = true;
    async function sync() {
      await flushOutbox();
      try {
        const res = await fetch(`${API_URL}/api/notes?crew_id=${encodeURIComponent(settings.crew_id)}`, { signal: AbortSignal.timeout(4000) });
        if (!res.ok) throw new Error(String(res.status));
        const notes: Event[] = await res.json();
        if (!live) return;
        notes.filter((n) => n.kind.type === "ground_note").forEach((n) => store.append(n)); // append dedupes by id
        setLink("up");
      } catch { if (live) setLink("down"); }
      if (live) setOutbox(readOutbox());
    }
    void sync();
    const timer = window.setInterval(sync, POLL_MS);
    window.addEventListener("online", sync);
    return () => { live = false; window.clearInterval(timer); window.removeEventListener("online", sync); };
  }, [settings.crew_id, store]);

  // Reports are timed in real UTC ms; time-skip moves the countdown forward too.
  const now = wallTime() + settings.met_offset_s * 1000;
  const notes = events.filter((e) => e.kind.type === "ground_note").sort((a, b) => b.met_s - a.met_s);
  const label = { up: "Link up", down: API_URL ? "Link down" : "Link down (no ground station configured)", checking: "Checking link" }[link];
  const icon = { up: "●", down: "▲", checking: "○" }[link];

  return (
    <>
      <div className="page-heading">
        <h1>Ground comms</h1>
        <span className="data-label">Simulated mission data</span>
      </div>
      <div className="comms-grid">
        <Card className="comms-card">
          <h2>Link</h2>
          <p role="status" className="comms-state" data-state={link}><span aria-hidden="true">{icon}</span> {label}</p>
          {link === "down" && <p className="next-step">Reports stay queued on board and send automatically. Onboard guidance keeps working.</p>}
          <p className="next-step">One-way delay: {delayLabel(settings.one_way_delay_s)}.</p>
          <div className="skip" role="group" aria-label="Demo time skip">
            <span className="data-label">Demo: skip time</span>
            {SKIPS.map(([seconds, text]) => <button key={text} onClick={() => store.skipTime(seconds)}>+{text}</button>)}
          </div>
        </Card>
        <Card className="comms-card">
          <h2>Outbox <span className="flag-count">{outbox.length}</span></h2>
          {outbox.length === 0 ? <p className="next-step">No reports sent. <a href="#/crew/comms/send">Write a report</a></p> : (
            <ul className="comms-list">
              {[...outbox].reverse().map(({ report, posted }) => {
                const left = report.deliver_at_earth_ms - now;
                const [state, text, glyph] = !posted ? ["queued", "Queued, waiting for link", "◔"]
                  : left > 0 ? ["transit", `Arrives in ${countdown(left)}`, "◐"] : ["arrived", "Arrived at ground", "●"];
                return (
                  <li key={report.id}>
                    <p role="status" className="comms-state" data-state={state}><span aria-hidden="true">{glyph}</span> {text}</p>
                    <p className="next-step">Sent {formatEarth(report.sent_earth_ms)}. {report.sbar.situation}</p>
                  </li>
                );
              })}
            </ul>
          )}
        </Card>
        <Card className="comms-card inbox">
          <h2>Inbox <span className="flag-count">{notes.length}</span></h2>
          {notes.length === 0 ? <p className="next-step">No notes from ground yet.</p> : (
            <ul className="comms-list">
              {notes.map((n) => n.kind.type === "ground_note" && (
                <li key={n.id}>
                  <p className="next-step">{n.author.replace("ground:", "")} · arrived {formatEarth(n.earth_ms)}</p>
                  <p>{n.kind.text}</p>
                </li>
              ))}
            </ul>
          )}
        </Card>
      </div>
    </>
  );
}
