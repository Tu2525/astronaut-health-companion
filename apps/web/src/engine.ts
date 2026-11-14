import type { Sbar } from "./contract/Sbar";
import type { AssessInput } from "./contract/AssessInput";
import type { Assessment } from "./contract/Assessment";
import type { Demo } from "./contract/Demo";
import type { Event } from "./contract/Event";
import type { Indicator } from "./contract/Indicator";
import type { Score } from "./contract/Score";
import type { Task } from "./contract/Task";
import init, * as wasm from "./engine-pkg/ahc_core_wasm.js";

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

<<<<<<< HEAD
export function draftSbar(assessment: Assessment, indicators: Indicator[]): Sbar {
  return JSON.parse(wasm.draft_sbar(JSON.stringify(assessment), JSON.stringify(indicators)));
=======
export function dueTasks(input: AssessInput): Task[] {
  return JSON.parse(wasm.due_tasks(JSON.stringify(input)));
>>>>>>> web/today-checkin
}
