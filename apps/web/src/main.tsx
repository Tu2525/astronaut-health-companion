import { useSyncExternalStore } from "react";
import { createRoot } from "react-dom/client";
import "../../../design/tokens.css";
import "./styles.css";
import { initEngine } from "./engine";
import { CrewOverview } from "./faces/crew/body/CrewOverview";
import { RegionDetail } from "./faces/crew/region/RegionDetail";
import { createStore } from "./store";
import type { CrewStore } from "./store";
import { TopBar } from "./ui/TopBar";

const CREW_ROUTE = "#/crew/body";

function subscribeRoute(listener: () => void) {
  window.addEventListener("hashchange", listener);
  return () => window.removeEventListener("hashchange", listener);
}

function App({ store }: { store: CrewStore }) {
  const model = useSyncExternalStore(store.subscribe, store.getSnapshot);
  const route = useSyncExternalStore(subscribeRoute, () => location.hash || CREW_ROUTE);
  return (
    <>
      <TopBar model={model} />
      <main className="content">
        {route.startsWith("#/crew/region/") ? (
          <RegionDetail region={route.slice(14)} assessment={model.assessment} indicators={model.indicators} />
        ) : route === CREW_ROUTE ? <CrewOverview assessment={model.assessment} /> : (
          <div className="page-heading">
            <h1>Page unavailable</h1>
            <a href={CREW_ROUTE}>Return to crew health</a>
          </div>
        )}
      </main>
    </>
  );
}

const root = createRoot(document.getElementById("root")!);
root.render(<main className="content" role="status">Loading mission assessment…</main>);

void initEngine().then(() => {
  const store = createStore();
  root.render(<App store={store} />);
  import.meta.hot?.dispose(() => { store.dispose(); root.unmount(); });
}).catch((error: unknown) => {
  console.error(error);
  root.render(
    <main className="content boot-error" role="alert">
      <h1>Mission assessment unavailable</h1>
      <p>The assessment could not be loaded.</p>
      <button onClick={() => location.reload()}>Reload</button>
    </main>,
  );
});
