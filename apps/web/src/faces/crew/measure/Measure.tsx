import { useState } from "react";
import { formatMet } from "../../../clock";
import type { Confidence } from "../../../contract/Confidence";
import type { CrewStore } from "../../../store";
import { Card } from "../../../ui/Card";
import { Ladder } from "../../../ui/Ladder";
import { SourceChip } from "../../../ui/SourceChip";
import { StatusChip } from "../../../ui/StatusChip";
import "./measure.css";

type Model = ReturnType<CrewStore["getSnapshot"]>;

export function Measure({ model, store, indicatorId }: { model: Model; store: CrewStore; indicatorId: string | null }) {
  if (indicatorId === null) return <Picker model={model} />;
  const indicator = model.indicators.find(({ id }) => id === indicatorId);
  if (!indicator) {
    return (
      <div className="page-heading">
        <h1>Unknown indicator</h1>
        <a href="#/crew/measure">Back to measurements</a>
      </div>
    );
  }
  return <Guide key={indicator.id} model={model} store={store} indicator={indicator} />;
}

function Picker({ model }: { model: Model }) {
  return (
    <>
      <div className="page-heading">
        <h1>Measure</h1>
        <span className="data-label">Choose an indicator</span>
      </div>
      <ul className="measure-list">
        {model.indicators.map((ind) => {
          const status = model.assessment.indicators.find(({ indicator_id }) => indicator_id === ind.id);
          return (
            <li key={ind.id}>
              <a className="measure-row" href={`#/crew/measure/${ind.id}`}>
                <span className="measure-name">{ind.name}</span>
                <span className="measure-last">
                  {status?.last_met_s != null ? `Last ${formatMet(status.last_met_s)} MET` : "Not measured"}
                </span>
                <StatusChip status={status?.status ?? "unknown"} />
              </a>
            </li>
          );
        })}
      </ul>
    </>
  );
}

// Systolic and diastolic come off the cuff together; the contract gives them one prompt.
const PAIRED: Record<string, string> = { bp_systolic: "bp_diastolic", bp_diastolic: "bp_systolic" };

const KEYS = ["7", "8", "9", "4", "5", "6", "1", "2", "3", "-", "0", "."];

function Guide({ model, store, indicator }: { model: Model; store: CrewStore; indicator: Model["indicators"][number] }) {
  const units = [{ unit: indicator.unit, factor: 1, offset: 0 }, ...indicator.alt_units];
  const [unitIdx, setUnitIdx] = useState(0);
  const [text, setText] = useState("");
  const partner = model.indicators.find(({ id }) => id === PAIRED[indicator.id]);
  const [text2, setText2] = useState("");
  const [active, setActive] = useState<1 | 2>(1);
  const typed = active === 1 ? text : text2;
  const setTyped = active === 1 ? setText : setText2;
  const [device, setDevice] = useState("manual entry");
  const [confidence, setConfidence] = useState<Confidence>("high");
  const [error, setError] = useState("");
  const [done, setDone] = useState(false);
  const u = units[unitIdx];

  function submit(e: React.FormEvent) {
    e.preventDefault();
    const shown = text.trim() === "" ? NaN : Number(text);
    const value = (shown - u.offset) / u.factor;
    const value2 = text2.trim() === "" ? NaN : Number(text2);
    if (!Number.isFinite(value) || (partner && !Number.isFinite(value2))) {
      setError(partner ? "Enter both numbers." : "Enter a number."); return;
    }
    const log = (indicator_id: string, v: number) => store.append({
      id: crypto.randomUUID(), crew_id: model.settings.crew_id, met_s: model.nowMet,
      earth_ms: model.earthMs, environment: model.settings.environment, author: model.settings.crew_id,
      kind: { type: "reading", indicator_id, value: v, device: device.trim() || "manual entry", confidence },
    });
    log(indicator.id, value);
    if (partner) log(partner.id, value2);
    setError("");
    setDone(true);
  }

  if (done) {
    return <Result model={model} indicatorId={indicator.id} name={indicator.name} onAgain={() => { setText(""); setText2(""); setActive(1); setDone(false); }} />;
  }

  return (
    <>
      <div className="page-heading">
        <h1>{indicator.name}</h1>
      </div>
      <div className="measure-grid">
        <Card>
          <h2>How to measure</h2>
          <p className="next-step">{indicator.what}</p>
          <ol className="measure-steps">
            {indicator.how_to_measure.map((step, i) => <li key={i}>{step}</li>)}
          </ol>
        </Card>
        <Card>
          <form onSubmit={submit} noValidate>
            <h2><label htmlFor="measure-value">{partner ? indicator.name : "Value"}</label></h2>
            {units.length > 1 && (
              <div className="unit-toggle" role="group" aria-label="Unit">
                {units.map((x, i) => (
                  <button key={x.unit} type="button" aria-pressed={i === unitIdx}
                    onClick={() => { setUnitIdx(i); setText(""); setError(""); }}>{x.unit}</button>
                ))}
              </div>
            )}
            <div className="measure-display">
              <input id="measure-value" inputMode="decimal" autoComplete="off" value={text}
                aria-describedby={error ? "measure-error" : undefined} aria-invalid={error ? true : undefined}
                onFocus={() => setActive(1)} data-active={partner && active === 1 ? "" : undefined}
                onChange={(e) => { setText(e.target.value); setError(""); }} />
              <span>{u.unit}</span>
            </div>
            {partner && (
              <>
                <h2><label htmlFor="measure-value-2">{partner.name}</label></h2>
                <div className="measure-display">
                  <input id="measure-value-2" inputMode="decimal" autoComplete="off" value={text2}
                    onFocus={() => setActive(2)} data-active={active === 2 ? "" : undefined}
                    onChange={(e) => { setText2(e.target.value); setError(""); }} />
                  <span>{partner.unit}</span>
                </div>
              </>
            )}
            {error && <p id="measure-error" className="measure-error" role="alert">{error}</p>}
            <div className="keypad">
              {KEYS.map((k) => (
                <button key={k} type="button" onClick={() => { setTyped(typed + k); setError(""); }}>{k}</button>
              ))}
              <button type="button" className="keypad-wide" onClick={() => setTyped(typed.slice(0, -1))}>Delete</button>
              <button type="button" className="keypad-wide" onClick={() => setTyped("")}>Clear</button>
            </div>
            <div className="measure-meta">
              <label>Device
                <input value={device} onChange={(e) => setDevice(e.target.value)} />
              </label>
              <label>Confidence
                <select value={confidence} onChange={(e) => setConfidence(e.target.value as Confidence)}>
                  <option value="high">High</option>
                  <option value="medium">Medium</option>
                  <option value="low">Low</option>
                </select>
              </label>
            </div>
            <button type="submit" className="measure-submit">Log reading</button>
          </form>
        </Card>
      </div>
    </>
  );
}

function Result({ model, indicatorId, name, onAgain }: { model: Model; indicatorId: string; name: string; onAgain: () => void }) {
  const status = model.assessment.indicators.find(({ indicator_id }) => indicator_id === indicatorId);
  const flag = model.assessment.flags.find(({ indicator_id }) => indicator_id === indicatorId);
  return (
    <>
      <div className="page-heading">
        <h1>{name}</h1>
      </div>
      <Card>
        <h2>Result</h2>
        {status && (
          <div className="result-status" role="status">
            <StatusChip status={status.status} />
            <SourceChip status={status.source_status} />
          </div>
        )}
        <p>{status?.reason}</p>
        {flag && <p className="next-step">{flag.next_step}</p>}
        <h3>Escalation</h3>
        <Ladder current={flag?.escalation ?? "monitor"} />
        <div className="result-actions">
          {flag && <a className="button-link primary" href={`#/crew/guidance/${indicatorId}`}>What to do next</a>}
          <button type="button" onClick={onAgain}>Measure again</button>
        </div>
      </Card>
    </>
  );
}
