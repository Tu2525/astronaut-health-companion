import type { Action } from "../../../contract/Action";
import type { Event } from "../../../contract/Event";
import type { EventKind } from "../../../contract/EventKind";
import type { CrewStore } from "../../../store";
import { Card } from "../../../ui/Card";
import { Ladder } from "../../../ui/Ladder";
import { SourceChip } from "../../../ui/SourceChip";
import { StatusChip } from "../../../ui/StatusChip";
import "./Guidance.css";

type Model = ReturnType<CrewStore["getSnapshot"]>;

export function Guidance({ model, store, indicatorId }: { model: Model; store: CrewStore; indicatorId: string }) {
  const { assessment, events, settings, nowMet, earthMs, indicators } = model;
  const flag = assessment.flags.find((f) => f.indicator_id === indicatorId);
  const status = assessment.indicators.find((i) => i.indicator_id === indicatorId);
  const name = indicators.find((i) => i.id === indicatorId)?.name ?? indicatorId;

  const write = (kind: EventKind) => store.append({
    id: crypto.randomUUID(), crew_id: settings.crew_id, met_s: nowMet, earth_ms: earthMs,
    environment: settings.environment, author: settings.crew_id, kind,
  } satisfies Event);
  const started = (action: Action) => events.some(({ kind }) =>
    kind.type === "action_taken" && kind.action_id === action.id && kind.indicator_id === indicatorId);
  const start = (action: Action) => {
    write({ type: "action_taken", action_id: action.id, indicator_id: indicatorId });
    if (action.kit_item) write({ type: "kit_use", item: action.kit_item, qty: 1 });
  };

  return (
    <div className="guidance">
      <div className="page-heading">
        <h1>Guidance: {name}</h1>
        <a href="#/crew/body">Back to crew health</a>
      </div>
      {!flag ? (
        <Card>
          <h2>Nothing to act on</h2>
          {status && <p className="flag-meta"><StatusChip status={status.status} /><SourceChip status={status.source_status} /></p>}
          <p className="next-step">{status?.reason ?? "No assessment for this indicator."}</p>
        </Card>
      ) : (
        <>
          <Card>
            <h2>Situation</h2>
            <div className="flag-meta">
              <StatusChip status={flag.status} />
              {status && <SourceChip status={status.source_status} />}
            </div>
            <p>{flag.summary}</p>
            <p className="next-step">{flag.next_step}</p>
          </Card>
          <Card>
            <h2>Escalation</h2>
            <Ladder current={flag.escalation} />
          </Card>
          <Card>
            <h2>Actions</h2>
            <ul className="action-list">
              {flag.actions.map(({ action, available }) => {
                const done = started(action);
                return (
                  <li key={action.id} data-available={available}>
                    <span className="action-text">
                      {action.text}
                      {!available && <span className="action-note"> · not in kit</span>}
                      {done && <span className="action-note"> · started</span>}
                    </span>
                    <button className="btn" data-primary="true" disabled={!available || done} onClick={() => start(action)}>
                      {done ? "Started" : "Start"}
                    </button>
                  </li>
                );
              })}
            </ul>
          </Card>
        </>
      )}
      <div className="link-row">
        <a className="btn" href={`#/crew/measure/${indicatorId}`}>Measure again</a>
        <a className="btn" href={`#/crew/comms/send?indicator=${indicatorId}`}>Send to ground</a>
      </div>
    </div>
  );
}
