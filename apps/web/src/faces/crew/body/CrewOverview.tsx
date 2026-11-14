import type { Assessment } from "../../../contract/Assessment";
import { Card } from "../../../ui/Card";
import { Ladder } from "../../../ui/Ladder";
import { SourceChip } from "../../../ui/SourceChip";
import { StatusChip } from "../../../ui/StatusChip";

export function CrewOverview({ assessment }: { assessment: Assessment }) {
  return (
    <>
      <div className="page-heading">
        <h1>Crew health</h1>
        <span className="data-label">Simulated mission data</span>
      </div>
      <div className="overview-grid">
        <Card className="overall-card">
          <h2>Mission assessment</h2>
          <div className="overall-value"><StatusChip status={assessment.overall} /></div>
          <h3>Escalation</h3>
          <Ladder current={assessment.escalation} />
        </Card>
        <Card className="flags-card">
          <h2>Active flags <span className="flag-count">{assessment.flags.length}</span></h2>
          {assessment.flags.length === 0 ? <p className="next-step">No active flags.</p> : (
            <ul className="flag-list">
              {assessment.flags.slice(0, 2).map((flag) => {
                const indicator = assessment.indicators.find(({ indicator_id }) => indicator_id === flag.indicator_id);
                return (
                  <li key={flag.indicator_id}>
                    <div className="flag-meta">
                      <StatusChip status={flag.status} />
                      {indicator && <SourceChip status={indicator.source_status} />}
                    </div>
                    <p>{flag.summary}</p>
                    <p className="next-step">{flag.next_step}</p>
                  </li>
                );
              })}
            </ul>
          )}
        </Card>
      </div>
    </>
  );
}
