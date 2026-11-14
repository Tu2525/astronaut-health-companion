import type { Status } from "../contract/Status";
import { StatusGlyph } from "./icons";

const words: Record<Status, string> = {
  nominal: "Nominal", watch: "Watch", caution: "Caution", urgent: "Urgent", unknown: "Unknown",
};

export function StatusChip({ status }: { status: Status }) {
  return <span className="status-chip" data-status={status}><StatusGlyph status={status} />{words[status]}</span>;
}
