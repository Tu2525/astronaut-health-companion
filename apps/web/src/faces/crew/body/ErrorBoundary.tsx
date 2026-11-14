import { Component } from "react";
import type { ReactNode } from "react";

// WebGL or model failure: render the fallback (nothing), the legend beside it carries the same information.
export class ErrorBoundary extends Component<{ onError(): void; children: ReactNode }, { failed: boolean }> {
  state = { failed: false };
  static getDerivedStateFromError() { return { failed: true }; }
  componentDidCatch(error: unknown) { console.error("3D body unavailable:", error); this.props.onError(); }
  render() { return this.state.failed ? null : this.props.children; }
}
