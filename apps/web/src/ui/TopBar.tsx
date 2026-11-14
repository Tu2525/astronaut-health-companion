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
      <a className="brand" href="#/crew/body">ZeroG Pulse</a>
      <div className="tb-item"><small>Mission day</small><b>{missionDay(nowMet)}<span className="tb-hide"> · MET {formatMet(nowMet)}</span></b></div>
      <div className="tb-item tb-hide"><small>Earth</small><b>{formatEarth(earthMs)}</b></div>
      <div className="tb-item tb-hide"><small>Environment</small><b>{environments[settings.environment]}</b></div>
      <div className="tb-item tb-hide"><small>One-way delay</small><b>{settings.one_way_delay_s} s <span className="tb-demo">demo</span></b></div>
      <div className="tb-spacer" />
      <div className="tb-item top-status" role="status"><small>Overall</small><StatusChip status={assessment.overall} /></div>
      <a className="safe-link" href="./safe/" aria-label="Open safe mode, the offline fallback view">Safe mode</a>
    </header>
  );
}
