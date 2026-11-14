const DAY_SECONDS = 86_400;
export const MISSION_START_KEY = "ahc.mission_start.v1";
const wallTime = () => performance.timeOrigin + performance.now();

export function createClock(seedMet: number, launchEarthMs: number, startedAt: number) {
  return {
    now: (offset: number) => seedMet + Math.max(0, wallTime() - startedAt) / 1000 + offset,
    earthMs: (met: number) => launchEarthMs + met * 1000,
  };
}

export function missionStart() {
  const saved = localStorage.getItem(MISSION_START_KEY);
  if (saved && Number.isFinite(Number(saved))) return Number(saved);
  const startedAt = wallTime();
  localStorage.setItem(MISSION_START_KEY, String(startedAt));
  return startedAt;
}

export function missionDay(met: number) {
  return Math.floor(met / DAY_SECONDS);
}

export function formatMet(met: number) {
  const day = missionDay(met);
  const hours = Math.floor(met / 3600) % 24;
  const minutes = Math.floor(met / 60) % 60;
  return `${day}d ${String(hours).padStart(2, "0")}:${String(minutes).padStart(2, "0")}`;
}

export function formatEarth(earthMs: number) {
  return `${new Date(earthMs).toISOString().slice(11, 16)} UTC`;
}
