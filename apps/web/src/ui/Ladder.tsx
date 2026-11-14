import type { Escalation } from "../contract/Escalation";

const steps: [Escalation, string][] = [
  ["monitor", "Monitor"], ["self_care", "Self-care"],
  ["consult_ground", "Consult ground"], ["urgent", "Urgent"],
];

export function Ladder({ current }: { current: Escalation }) {
  return (
    <ol className="ladder" aria-label="Escalation steps">
      {steps.map(([id, label]) => (
        <li key={id} aria-current={current === id ? "step" : undefined}>{label}</li>
      ))}
    </ol>
  );
}
