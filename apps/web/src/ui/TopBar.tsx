import { formatEarth, formatMet, missionDay } from "../clock";
import type { Environment } from "../contract/Environment";
import type { CrewStore } from "../store";
import { StatusChip } from "./StatusChip";

const environments: Record<Environment, string> = {
  station: "Station", transit: "Mars transit", lunar_base: "Lunar base",
  mars_base: "Mars base", earth_clinic: "Earth clinic",
};

export function TopBar({ model }: { model: ReturnType<CrewStore["getSnapshot"]> }) {
  const { assessment, earthMs, nowMet, settings } = model;
  return (
    <header className="top-bar">
      <div className="top-inner">
        <a className="brand" href="#/crew/body">
          <span className="brand-mark" aria-hidden="true">✦</span>
          <span>Astronaut Health Companion</span>
        </a>
        <div className="top-status" role="status">Overall <StatusChip status={assessment.overall} /></div>
      </div>
      <dl className="mission-data">
        <div><dt>Mission day</dt><dd>{missionDay(nowMet)}</dd></div>
        <div><dt>MET</dt><dd>{formatMet(nowMet)}</dd></div>
        <div><dt>Earth time</dt><dd>{formatEarth(earthMs)}</dd></div>
        <div><dt>Environment</dt><dd>{environments[settings.environment]}</dd></div>
        <div><dt>One-way delay</dt><dd>{settings.one_way_delay_s} s <small>demo</small></dd></div>
      </dl>
    </header>
  );
}
