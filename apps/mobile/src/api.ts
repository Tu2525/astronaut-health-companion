import type { Assessment } from "@contract/Assessment";
import type { AssessInput } from "@contract/AssessInput";
import type { Demo } from "@contract/Demo";
import type { Indicator } from "@contract/Indicator";

// The Android emulator reaches the host's localhost at 10.0.2.2; the API default port is 8080.
const BASE = process.env.EXPO_PUBLIC_API_URL ?? "http://10.0.2.2:8080";

async function call<T>(path: string, init?: RequestInit): Promise<T> {
  // Fail fast when the engine is out of reach, so the UI can say so.
  // (RN fetch on a dropped connection can hang past abort, so race a timer too.)
  const ctl = new AbortController();
  let timer: ReturnType<typeof setTimeout> | undefined;
  const timeout = new Promise<never>((_, reject) => {
    timer = setTimeout(() => {
      ctl.abort();
      reject(new Error(`${path}: timeout`));
    }, 5000);
  });
  const res = await Promise.race([fetch(BASE + path, { ...init, signal: ctl.signal }), timeout]).finally(() =>
    clearTimeout(timer),
  );
  if (!res.ok) throw new Error(`${path}: HTTP ${res.status}`);
  return (await res.json()) as T;
}

export const getDemo = () => call<Demo>("/api/demo");
export const getRegistry = () => call<Indicator[]>("/api/registry");
export const assess = (input: AssessInput) =>
  call<Assessment>("/api/assess", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(input),
  });
