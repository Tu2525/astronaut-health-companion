import type { Report } from "../../../contract/Report";

const KEY = "ahc.outbox.v1";
// Base URL of the Ground Station API; unset means the app runs fully offline.
export const API_URL: string = (import.meta.env.VITE_API_URL ?? "").replace(/\/$/, "");
// Contract section 9: 20 s of real delay is shown as 18 min of mission time.
const DEMO_SCALE = 54;

export interface Queued { report: Report; posted: boolean }

export function readOutbox(): Queued[] {
  try {
    const saved = JSON.parse(localStorage.getItem(KEY) ?? "[]");
    return Array.isArray(saved) ? saved : [];
  } catch { return []; }
}

export function writeOutbox(items: Queued[]) {
  try { localStorage.setItem(KEY, JSON.stringify(items)); } catch { /* storage full or blocked: keep running */ }
}

export function delayLabel(oneWaySeconds: number) {
  return `${Math.round((oneWaySeconds * DEMO_SCALE) / 60)} min (compressed for demo)`;
}

/** Resolves true when the server has the report; never throws. */
async function post(report: Report) {
  try {
    const res = await fetch(`${API_URL}/api/reports`, {
      method: "POST", headers: { "Content-Type": "application/json" },
      body: JSON.stringify(report), signal: AbortSignal.timeout(4000),
    });
    return res.ok || res.status === 409; // 409: the server already holds this id
  } catch { return false; }
}

/** Try every queued report once. */
export async function flushOutbox() {
  if (!API_URL) return;
  for (const item of readOutbox().filter((q) => !q.posted)) {
    if (await post(item.report)) {
      writeOutbox(readOutbox().map((q) => (q.report.id === item.report.id ? { ...q, posted: true } : q)));
    }
  }
}
