import type { SourceStatus } from "../contract/SourceStatus";

export function SourceChip({ status }: { status: SourceStatus }) {
  return <span className="source-chip" data-source={status}>{status}</span>;
}
