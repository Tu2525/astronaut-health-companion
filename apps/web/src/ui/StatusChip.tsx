import type { Status } from "../contract/Status";

const presentation: Record<Status, { icon: string; word: string }> = {
  nominal: { icon: "●", word: "Nominal" },
  watch: { icon: "◐", word: "Watch" },
  caution: { icon: "▲", word: "Caution" },
  urgent: { icon: "◆", word: "Urgent" },
  unknown: { icon: "○", word: "Unknown" },
};

export function StatusChip({ status }: { status: Status }) {
  const { icon, word } = presentation[status];
  return <span className="status-chip" data-status={status}><span aria-hidden="true">{icon}</span>{word}</span>;
}
