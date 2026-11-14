import { formatMet } from "../../../clock";
import type { Assessment } from "../../../contract/Assessment";
import type { Indicator } from "../../../contract/Indicator";
import type { IndicatorStatus } from "../../../contract/IndicatorStatus";
import { Card } from "../../../ui/Card";
import { SourceChip } from "../../../ui/SourceChip";
import { StatusChip } from "../../../ui/StatusChip";
import "./region.css";

const num = (n: number) => String(Number(n.toPrecision(4)));
const label = (s: string) => s.replace(/_/g, " ");

function Row({ ind, st, flagged }: { ind: Indicator; st: IndicatorStatus | undefined; flagged: boolean }) {
  const b = st?.bands;
  return (
    <Card className="region-row">
      <div className="region-row-head">
        <h2><a href={`#/crew/measure/${ind.id}`}>{ind.name}</a></h2>
        <StatusChip status={st?.status ?? "unknown"} />
      </div>
      <p className="region-value">
        {st?.value == null ? "No reading" : `${num(st.value)} ${ind.unit}`}
        {st?.last_met_s != null && <span className="next-step"> · measured MET {formatMet(st.last_met_s)}</span>}
      </p>
      {st && <p>{st.reason}</p>}
      {st && (st.baseline != null || st.preflight != null || st.drift_pct != null) && (
        <dl className="region-facts">
          {st.baseline != null && <div><dt>Baseline</dt><dd>{num(st.baseline)} {ind.unit}</dd></div>}
          {st.preflight != null && <div><dt>Preflight</dt><dd>{num(st.preflight)} {ind.unit}</dd></div>}
          {st.drift_pct != null && <div><dt>Drift</dt><dd>{num(st.drift_pct)}%</dd></div>}
        </dl>
      )}
      <ul className="hazard-list" aria-label="Hazards">
        {ind.hazards.map((h) => <li key={h} data-hazard={h}>{label(h)}</li>)}
      </ul>
      <p className="region-links">
        <a href={`#/crew/measure/${ind.id}`}>Measure</a>
        {flagged && <a href={`#/crew/guidance/${ind.id}`}>Guidance</a>}
      </p>
      <details className="why">
        <summary>Why?</summary>
        <h3>What it is</h3><p>{ind.what}</p>
        <h3>Why it matters in space</h3><p>{ind.why_in_space}</p>
        {ind.expected_change && <><h3>Expected change in flight</h3><p>{ind.expected_change}</p></>}
        {b && (
          <>
            <h3>Ranges used <SourceChip status={st.source_status} /></h3>
            <dl className="region-facts">
              <div><dt>Nominal</dt><dd>{num(b.nominal_low)} to {num(b.nominal_high)} {ind.unit}</dd></div>
              <div><dt>Caution limits</dt><dd>{num(b.caution_low)} to {num(b.caution_high)} {ind.unit}</dd></div>
            </dl>
          </>
        )}
        <h3>Source <SourceChip status={ind.source.status} /></h3>
        <p>{ind.source.url ? <a href={ind.source.url} target="_blank" rel="noreferrer">{ind.source.citation}</a> : ind.source.citation}</p>
      </details>
    </Card>
  );
}

export function RegionDetail({ region, assessment, indicators }: { region: string; assessment: Assessment; indicators: Indicator[] }) {
  const list = indicators.filter((i) => i.region === region);
  const rs = assessment.regions.find((r) => r.region === region);
  return (
    <>
      <div className="page-heading">
        <h1>{region}</h1>
        <StatusChip status={rs?.status ?? "unknown"} />
      </div>
      <p><a href="#/crew/body">Back to crew health</a></p>
      {list.length === 0 ? <p className="next-step">No indicators are tracked for this region.</p> : (
        <div className="region-list">
          {list.map((ind) => <Row key={ind.id} ind={ind} st={assessment.indicators.find((s) => s.indicator_id === ind.id)} flagged={assessment.flags.some((f) => f.indicator_id === ind.id)} />)}
        </div>
      )}
    </>
  );
}
