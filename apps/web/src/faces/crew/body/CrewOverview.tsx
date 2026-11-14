import { lazy, Suspense, useState } from "react";
import type { CSSProperties } from "react";
import type { Hazard } from "../../../contract/Hazard";
import type { Region } from "../../../contract/Region";
import type { Status } from "../../../contract/Status";
import { Ladder } from "../../../ui/Ladder";
import { RegionIcon } from "../../../ui/icons";
import { SourceChip } from "../../../ui/SourceChip";
import { StatusChip } from "../../../ui/StatusChip";
import "./body.css";
import { ErrorBoundary } from "./ErrorBoundary";
import { HAZARDS, REGIONS, regionName } from "./regions";
import type { Model } from "./regions";

// three.js stays out of the first paint: the legend renders before the canvas chunk arrives.
const BodyCanvas = lazy(() => import("./BodyCanvas"));

const countWord = (n: number) => (n === 1 ? "One thing" : n === 2 ? "Two things" : `${n} things`);

export function CrewOverview({ model }: { model: Model }) {
  const { assessment, indicators } = model;
  const [lens, setLens] = useState<Hazard | null>(null);
  const [picked, setPicked] = useState<Region | null>(null);
  const [focus, setFocus] = useState<{ region: Region; n: number } | null>(null);
  const [ready, setReady] = useState(false);
  const [failed, setFailed] = useState(false);

  const regionOf = (indicatorId: string) => indicators.find(({ id }) => id === indicatorId)?.region;
  const status = Object.fromEntries(assessment.regions.map((r) => [r.region, r.status])) as Record<Region, Status>;
  const flags = assessment.flags;
  const selected = picked ?? (flags[0] ? regionOf(flags[0].indicator_id) ?? null : null);

  // Hazard lens membership comes from the registry: a region is lit when any of its indicators carries the hazard.
  const lit = lens ? new Set(indicators.filter((i) => i.hazards.includes(lens)).map((i) => i.region)) : null;

  // "What is off" per region: the engine's ranked flag first, else the reason of an unknown indicator.
  const meta = (region: Region): string | null => {
    const flag = flags.find((f) => regionOf(f.indicator_id) === region);
    if (flag) return flag.summary;
    const unknown = assessment.indicators.find((i) => i.status === "unknown" && regionOf(i.indicator_id) === region);
    return unknown?.reason ?? null;
  };

  const select = (region: Region) => {
    setPicked(region);
    setFocus((f) => ({ region, n: (f?.n ?? 0) + 1 })); // selecting again re-zooms
  };

  // Presentation of the engine's state: stale = unknown but it has been read before; cold start = never read.
  const unknown = assessment.overall === "unknown";
  const stale = assessment.indicators.some((i) => i.status === "unknown" && i.last_met_s !== null);
  const headline = flags.length > 0 ? `${countWord(flags.length)} worth a look`
    : unknown ? (stale ? "Some readings are old" : "Learning your normal")
    : "All clear";
  const sub = flags.length > 0 ? "Tap a system to see what is behind its status."
    : unknown ? (stale
      ? "Some readings are old, so those systems are shown as unknown."
      : "Building your baseline. Ranges are general guidance until then.")
    : "All systems within your normal. Nothing needs you right now.";

  const panelIndicators = selected ? indicators.filter((i) => i.region === selected) : [];

  return (
    <div className="body-home">
      <section className="card body-stage" aria-labelledby="body-h">
        <div className="body-head">
          <div>
            <h1 id="body-h">{headline}</h1>
            <p>{sub}</p>
          </div>
          <span className="body-sim" title="Ranges are simulated, not NASA data">Simulated data</span>
        </div>

        <div className="body-hero" data-model={failed ? "off" : "on"} role="group"
          aria-label="Body model. Same information as the Body systems list.">
          <aside className="body-legend" aria-label="Body systems">
            <h2>Body systems · tap one</h2>
            <div className="body-rlist">
              {REGIONS.map(({ id, name }) => {
                const s = status[id] ?? "unknown";
                const note = meta(id);
                return (
                  <button key={id} className="body-rrow" data-s={s} aria-pressed={selected === id}
                    data-dim={lit && !lit.has(id) ? "" : undefined} onClick={() => select(id)}>
                    <span className="body-rico"><RegionIcon region={id} /></span>
                    <span className="body-rname">{name}</span>
                    <StatusChip status={s} />
                    {note && <span className="body-rmeta">{note}</span>}
                  </button>
                );
              })}
            </div>
          </aside>

          {!failed && (
            <div className="body-viewer">
              <ErrorBoundary onError={() => setFailed(true)}>
                <Suspense fallback={null}>
                  <BodyCanvas status={status} lit={lit} selected={selected} focus={focus}
                    onPick={select} onEmpty={() => setFocus(null)} onReady={() => setReady(true)} />
                </Suspense>
              </ErrorBoundary>
              {!ready && <div className="body-load" role="status">Loading body model</div>}
              {ready && <div className="body-hint">Drag to turn. Tap an organ to look closer.</div>}
              {focus && <button className="body-reset" onClick={() => setFocus(null)}>Whole body</button>}
            </div>
          )}
        </div>

        <div className="body-lens" role="group" aria-label="Hazard lens">
          {HAZARDS.map(({ id, label }) => (
            <button key={id} aria-pressed={lens === id} onClick={() => setLens(lens === id ? null : id)}
              style={{ "--h": `var(--hazard-${id.replace("_", "-")})` } as CSSProperties}>{label}</button>
          ))}
        </div>
      </section>

      <aside className="body-side">
        <section className="card" aria-labelledby="body-flags-h">
          <h2 id="body-flags-h">What needs you now <span className="flag-count">{flags.length}</span></h2>
          {flags.length === 0 ? <p className="body-calm">Calm. Nothing needs attention.</p> : (
            <ul className="body-flags">
              {flags.slice(0, 2).map((flag) => {
                const indicator = assessment.indicators.find(({ indicator_id }) => indicator_id === flag.indicator_id);
                const region = regionOf(flag.indicator_id);
                return (
                  <li key={flag.indicator_id} data-s={flag.status}>
                    <div className="body-flag-meta">
                      <StatusChip status={flag.status} />
                      {region && <span className="body-muted">{regionName(region)}</span>}
                      {indicator && <SourceChip status={indicator.source_status} />}
                    </div>
                    <p className="body-flag-summary">{flag.summary}</p>
                    <p className="next-step">{flag.next_step}</p>
                    <div className="body-flag-links">
                      {region && <button onClick={() => select(region)}>Show on body</button>}
                      <a href={`#/crew/guidance/${flag.indicator_id}`}>Open guidance</a>
                    </div>
                  </li>
                );
              })}
            </ul>
          )}
        </section>

        <section className="card" aria-live="polite" aria-labelledby="body-sel-h">
          <h2 id="body-sel-h">Selected system</h2>
          {selected ? (
            <>
              <h3>{regionName(selected)} <StatusChip status={status[selected] ?? "unknown"} /></h3>
              {panelIndicators.length === 0 ? <p className="body-muted">No indicators are tracked for this system yet.</p> : (
                <ul className="body-inds">
                  {panelIndicators.map((ind) => {
                    const a = assessment.indicators.find(({ indicator_id }) => indicator_id === ind.id);
                    const flagged = flags.some((f) => f.indicator_id === ind.id);
                    return (
                      <li key={ind.id}>
                        <div className="body-ind-head">
                          <b>{ind.name}</b>
                          {a && <StatusChip status={a.status} />}
                        </div>
                        {a && <p className="body-muted">{a.reason}</p>}
                        <div className="body-flag-meta">
                          {a && <SourceChip status={a.source_status} />}
                          {flagged && <a href={`#/crew/guidance/${ind.id}`}>What should I do?</a>}
                        </div>
                      </li>
                    );
                  })}
                </ul>
              )}
              <a className="body-open" href={`#/crew/region/${selected}`}>Open {regionName(selected).toLowerCase()} details</a>
            </>
          ) : <p className="body-muted">Pick a system on the body or in the list.</p>}
        </section>

        <section className="card" aria-labelledby="body-esc-h">
          <h2 id="body-esc-h">Escalation</h2>
          <Ladder current={assessment.escalation} />
        </section>
      </aside>
    </div>
  );
}
