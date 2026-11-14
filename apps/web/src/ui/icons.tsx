import type { ReactNode } from "react";
import type { Hazard } from "../contract/Hazard";
import type { Region } from "../contract/Region";
import type { Status } from "../contract/Status";

// Line icons on a 24 grid, lifted from design/references/crew-home-proposal-v4-3d.html.
// Attributes (not CSS) so they render correctly on any screen. Status shapes differ on purpose.
function Svg({ children, size = "20px", stroke = 1.7 }: { children: ReactNode; size?: string; stroke?: number }) {
  return (
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke="currentColor" strokeWidth={stroke}
      strokeLinecap="round" strokeLinejoin="round" aria-hidden="true" focusable="false" style={{ flex: "none", display: "block" }}>
      {children}
    </svg>
  );
}

const regionPaths: Record<Region, ReactNode> = {
  head: <><path d="M12 5.5a3 3 0 0 0-5.7 1.2A3 3 0 0 0 4 10.5a3 3 0 0 0 1.2 4.8A3 3 0 0 0 10 19a2 2 0 0 0 2-1.5Z" /><path d="M12 5.5a3 3 0 0 1 5.7 1.2 3 3 0 0 1 2.3 3.8 3 3 0 0 1-1.2 4.8A3 3 0 0 1 14 19a2 2 0 0 1-2-1.5" /><path d="M8.5 10.5c1 0 1.8.7 1.8 1.7M15.5 10.5c-1 0-1.8.7-1.8 1.7" /></>,
  eyes: <><path d="M2.5 12S6 5.5 12 5.5 21.5 12 21.5 12 18 18.5 12 18.5 2.5 12 2.5 12Z" /><circle cx="12" cy="12" r="3" /></>,
  mind: <><path d="M19.5 14.5A8 8 0 1 1 9.5 4.5a6.2 6.2 0 0 0 10 10Z" /><path d="M16 4v3M14.5 5.5h3" /></>,
  lungs: <><path d="M12 3v8m0 0-2.5 2m2.5-2 2.5 2" /><path d="M9 7.5C6 8.5 4 12.5 4 16.5c0 2 1 3 2.8 3 1.7 0 2.2-1 2.2-3Z" /><path d="M15 7.5c3 1 5 5 5 9 0 2-1 3-2.8 3-1.7 0-2.2-1-2.2-3Z" /></>,
  heart: <><path d="M12 20.5 4.6 13.4A5 5 0 0 1 12 6.6a5 5 0 0 1 7.4 6.8Z" /><path d="M4 12h4l1.5-2.5 3 5 1.5-2.5h6" /></>,
  gut: <path d="M7 3.5h8.5a3 3 0 0 1 0 6H8.5a3 3 0 0 0 0 6H15a2.5 2.5 0 0 1 0 5h-3.5" />,
  immune: <><path d="M12 3 19 6v5.5c0 4.5-3 7.8-7 9.5-4-1.7-7-5-7-9.5V6Z" /><path d="M12 9v5.5M9.2 11.8h5.6" /></>,
  skin: <><path d="M3 8.5c3-2 6 2 9 0s6-2 9 0M3 13.5c3-2 6 2 9 0s6-2 9 0" /><path d="M3 18.5h18" /></>,
  muscles: <path d="M6.5 7v10M17.5 7v10M3.5 9.5v5M20.5 9.5v5M6.5 12h11" />,
  bones: <><path d="m8.5 8.5 7 7" /><path d="M8.5 8.5a2 2 0 1 0-2.8-2.8 2 2 0 1 0-1.5 3.5 2 2 0 0 0 4.3-.7ZM15.5 15.5a2 2 0 1 0 2.8 2.8 2 2 0 1 0 1.5-3.5 2 2 0 0 0-4.3.7Z" /></>,
};

const statusPaths: Record<Status, ReactNode> = {
  nominal: <><circle cx="12" cy="12" r="9" /><path d="m8 12.5 3 3 5-6" /></>,
  watch: <><circle cx="12" cy="12" r="9" /><path d="M12 3a9 9 0 0 1 0 18Z" fill="currentColor" /></>,
  caution: <><path d="M12 3.5 21.5 20h-19Z" /><path d="M12 10v4.5M12 17.2v.1" /></>,
  urgent: <><path d="M8 2.5h8l5.5 5.5v8L16 21.5H8L2.5 16V8Z" /><path d="M12 7.5v6M12 16.5v.1" /></>,
  unknown: <><circle cx="12" cy="12" r="9" strokeDasharray="3 2.6" /><path d="M9.8 9.6a2.3 2.3 0 1 1 3.3 2c-.7.4-1.1.9-1.1 1.8M12 16.6v.1" /></>,
};

export const RegionIcon = ({ region }: { region: Region }) => <Svg>{regionPaths[region]}</Svg>;
export const StatusGlyph = ({ status }: { status: Status }) => <Svg size="1.15em" stroke={2.2}>{statusPaths[status]}</Svg>;

const hazardPaths: Record<Hazard, ReactNode> = {
  radiation: <><circle cx="12" cy="12" r="1.1" /><path d="M7.75 4.64A8.5 8.5 0 0 1 16.25 4.64L13.80 8.88A3.6 3.6 0 0 0 10.20 8.88Z" /><path d="M20.50 12.00A8.5 8.5 0 0 1 16.25 19.36L13.80 15.12A3.6 3.6 0 0 0 15.60 12.00Z" /><path d="M7.75 19.36A8.5 8.5 0 0 1 3.50 12.00L8.40 12.00A3.6 3.6 0 0 0 10.20 15.12Z" /></>,
  isolation: <><rect x="7" y="2.5" width="10" height="19" rx="5" /><circle cx="12" cy="9.5" r="1.8" /><path d="M9.5 17v-2.2a2.5 2.5 0 0 1 5 0V17" /></>,
  distance: <><circle cx="6" cy="17" r="3.5" /><path d="M9 12.5C11 8 14.5 6 18 6" strokeDasharray="2 3" /><circle cx="20" cy="5.5" r="1.2" /></>,
  gravity: <><path d="M12 3v11m0 0-4-4m4 4 4-4" /><path d="M3 21c2-3 5-4.5 9-4.5s7 1.5 9 4.5" /></>,
  closed_environment: <><path d="M3 20h18M4.5 20a7.5 7.5 0 0 1 15 0" /><path d="M9 14.5a3.2 3.2 0 0 1 5.6-1.2M15 14.5a3.2 3.2 0 0 1-5.6 1.2" /><path d="M14.6 11.5v2h-2M9.4 17.5v-2h2" /></>,
};

export const HazardIcon = ({ hazard }: { hazard: Hazard }) => <Svg size="16px">{hazardPaths[hazard]}</Svg>;
