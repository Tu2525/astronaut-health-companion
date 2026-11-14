import { useMemo, useState } from "react";
import { wallTime } from "../../../clock";
import type { Event } from "../../../contract/Event";
import type { Report } from "../../../contract/Report";
import { draftSbar } from "../../../engine";
import type { CrewStore } from "../../../store";
import { Card } from "../../../ui/Card";
import { StatusChip } from "../../../ui/StatusChip";
import { delayLabel, flushOutbox, readOutbox, writeOutbox } from "./outbox";
import "./comms.css";

type Model = ReturnType<CrewStore["getSnapshot"]>;
const WINDOW_S = 48 * 3600;

export function SendReport({ model, store, indicatorId }: { model: Model; store: CrewStore; indicatorId: string | null }) {
  const { assessment, indicators, settings, nowMet, earthMs, events } = model;
  const sbar = useMemo(() => draftSbar(assessment, indicators), [assessment, indicators]);
  const recent = events.filter((e) => e.crew_id === settings.crew_id && e.met_s <= nowMet && nowMet - e.met_s <= WINDOW_S);
  const indicator = indicators.find(({ id }) => id === indicatorId);
  const [sent, setSent] = useState(false);

  function send() {
    const now = wallTime();
    const report: Report = {
      id: crypto.randomUUID(), crew_id: settings.crew_id, sent_met_s: nowMet, sent_earth_ms: now,
      deliver_at_earth_ms: now + settings.one_way_delay_s * 1000, sbar, assessment, events: recent,
    };
    writeOutbox([...readOutbox(), { report, posted: false }]);
    const logged: Event = {
      id: crypto.randomUUID(), crew_id: settings.crew_id, met_s: nowMet, earth_ms: earthMs,
      environment: settings.environment, author: settings.crew_id,
      kind: { type: "report_sent", report_id: report.id, deliver_at_earth_ms: report.deliver_at_earth_ms },
    };
    store.append(logged);
    void flushOutbox();
    setSent(true);
  }

  const parts: [string, string][] = [
    ["Situation", sbar.situation], ["Background", sbar.background],
    ["Assessment", sbar.assessment], ["Recommendation", sbar.recommendation],
  ];
  return (
    <>
      <div className="page-heading">
        <h1>Report to ground</h1>
      </div>
      <Card className="comms-card">
        <h2>SBAR draft{indicator ? <> for {indicator.name}</> : null}</h2>
        <p className="next-step">Drafted on board by the engine. One-way delay: {delayLabel(settings.one_way_delay_s)}.</p>
        <dl className="sbar">
          {parts.map(([label, text]) => <div key={label}><dt>{label}</dt><dd>{text}</dd></div>)}
        </dl>
        <p>Current assessment: <StatusChip status={assessment.overall} /></p>
        <p className="next-step">Attached: {recent.length} events from the last 48 hours.</p>
        {sent ? (
          <p role="status" className="comms-state" data-state="queued">
            <span aria-hidden="true">◔</span> Queued in your outbox; it sends when the link is up.{" "}
            <a href="#/crew/comms">Open outbox</a>
          </p>
        ) : <button onClick={send}>Send to ground</button>}
      </Card>
    </>
  );
}
