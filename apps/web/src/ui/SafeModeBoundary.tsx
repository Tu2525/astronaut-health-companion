import { Component } from "react";
import type { ReactNode } from "react";

type Props = { children: ReactNode };
type State = { failed: boolean };

// Catches render errors so a crash never leaves the crew with a blank screen.
// The safe-mode app lives at ./safe/ and does not depend on this app's render tree.
export class SafeModeBoundary extends Component<Props, State> {
  state: State = { failed: false };

  static getDerivedStateFromError(): State {
    return { failed: true };
  }

  componentDidCatch(error: unknown) {
    console.error(error);
  }

  render() {
    if (!this.state.failed) return this.props.children;
    return (
      <main className="content boundary-fallback" role="alert">
        <section className="card boundary-card">
          <h1>Something broke. Your log is safe.</h1>
          <p>Your events are stored on this device. Open safe mode, or reload this page.</p>
          <div className="boundary-actions">
            <a className="button-link" href="./safe/">Open safe mode</a>
            <button type="button" onClick={() => location.reload()}>Reload</button>
          </div>
        </section>
      </main>
    );
  }
}
