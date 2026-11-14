import { useState } from "react";
import type { FormEvent } from "react";
import type { CrewStore } from "../../../store";
import { Card } from "../../../ui/Card";
import "./checkin.css";

const FIELDS = [
  // Contract: every scale runs 1 (bad) to 5 (good), so for stress 5 means calm.
  ["mood", "Mood", "1 low", "5 good"],
  ["stress", "Stress", "1 very stressed", "5 calm"],
  ["energy", "Energy", "1 drained", "5 full of energy"],
  ["sleep_quality", "Sleep quality", "1 poor", "5 good"],
] as const;
type Field = (typeof FIELDS)[number][0];

export function CheckIn({ store }: { store: CrewStore }) {
  const [values, setValues] = useState<Partial<Record<Field, number>>>({});
  const [note, setNote] = useState("");
  const [sent, setSent] = useState(false);
  const complete = FIELDS.every(([key]) => values[key] !== undefined);

  function submit(e: FormEvent) {
    e.preventDefault();
    const { mood, stress, energy, sleep_quality } = values;
    if (mood === undefined || stress === undefined || energy === undefined || sleep_quality === undefined) return;
    const { settings, nowMet, earthMs } = store.getSnapshot();
    store.append({
      id: crypto.randomUUID(), crew_id: settings.crew_id, met_s: nowMet, earth_ms: earthMs,
      environment: settings.environment, author: settings.crew_id,
      kind: { type: "check_in", mood, stress, energy, sleep_quality, note: note.trim() || null },
    });
    setSent(true);
    window.setTimeout(() => { location.hash = "#/crew/today"; }, 1500);
  }

  if (sent) {
    return (
      <Card className="checkin-card">
        <h1 role="status">Check-in saved</h1>
        <p className="next-step">Thanks. Returning to Today…</p>
        <a href="#/crew/today">Back to Today</a>
      </Card>
    );
  }

  return (
    <form onSubmit={submit}>
      <div className="page-heading"><h1>Check-in</h1></div>
      <Card className="checkin-card">
        {FIELDS.map(([key, label, low, high]) => (
          <fieldset key={key} className="scale" role="radiogroup">
            <legend>{label}</legend>
            <div className="scale-options">
              {[1, 2, 3, 4, 5].map((n) => (
                <label key={n}>
                  <input type="radio" name={key} value={n} checked={values[key] === n}
                    onChange={() => setValues({ ...values, [key]: n })} />
                  <span>{n}</span>
                </label>
              ))}
            </div>
            <div className="scale-ends"><span>{low}</span><span>{high}</span></div>
          </fieldset>
        ))}
        <label className="note">
          <span>Note (optional)</span>
          <textarea rows={3} value={note} onChange={(e) => setNote(e.target.value)} />
        </label>
        <button type="submit" className="submit" disabled={!complete}>Save check-in</button>
      </Card>
    </form>
  );
}
