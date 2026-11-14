import type { AssessInput } from "./contract/AssessInput";
import type { Assessment } from "./contract/Assessment";
import type { Event } from "./contract/Event";
import type { Indicator } from "./contract/Indicator";
import type { KitItem } from "./contract/KitItem";
import type { Score } from "./contract/Score";
import init, * as wasm from "./engine-pkg/ahc_core_wasm.js";

interface Demo {
  crew_id: string;
  now_met_s: number;
  launch_earth_ms: number;
  events: Event[];
  kit: KitItem[];
}

export async function initEngine() {
  await init();
}

export function assess(input: AssessInput): Assessment {
  return JSON.parse(wasm.assess(JSON.stringify(input)));
}

export function registry(): Indicator[] {
  return JSON.parse(wasm.registry());
}

export function demo(): Demo {
  return JSON.parse(wasm.demo());
}

export function score(crewId: string, events: Event[]): Score {
  return JSON.parse(wasm.score(crewId, JSON.stringify(events)));
}
