import { useSyncExternalStore } from "react";
import { createRoot } from "react-dom/client";
import "@fontsource/syne/600.css";
import "@fontsource/syne/700.css";
import "@fontsource/syne/800.css";
import "@fontsource/manrope/400.css";
import "@fontsource/manrope/500.css";
import "@fontsource/manrope/600.css";
import "../../../design/tokens.css";
import "./styles.css";
import { initEngine } from "./engine";
import { CrewOverview } from "./faces/crew/body/CrewOverview";
import { CheckIn } from "./faces/crew/checkin/CheckIn";
import { CommsPage } from "./faces/crew/comms/CommsPage";
import { SendReport } from "./faces/crew/comms/SendReport";
import { Guidance } from "./faces/crew/guidance/Guidance";
import { Library } from "./faces/crew/library/Library";
import { Measure } from "./faces/crew/measure/Measure";
import { RegionDetail } from "./faces/crew/region/RegionDetail";
import { Settings } from "./faces/crew/settings/Settings";
import { Today } from "./faces/crew/today/Today";
import { createStore } from "./store";
import type { CrewStore } from "./store";
import { SafeModeBoundary } from "./ui/SafeModeBoundary";
import { NavBar } from "./ui/NavBar";
import { TopBar } from "./ui/TopBar";

const CREW_ROUTE = "#/crew/body";

function subscribeRoute(listener: () => void) {
  window.addEventListener("hashchange", listener);
  return () => window.removeEventListener("hashchange", listener);
}

type Model = ReturnType<CrewStore["getSnapshot"]>;

function Page({ route, model, store }: { route: string; model: Model; store: CrewStore }) {
  const [path, query = ""] = route.split("?");
  const [, , screen, param] = path.split("/");
  const id = param ? decodeURIComponent(param) : null;
  switch (screen) {
    case "body": return <CrewOverview model={model} />;
    case "today": return <Today store={store} />;
    case "checkin": return <CheckIn store={store} />;
    case "measure": return <Measure model={model} store={store} indicatorId={id} />;
    case "guidance": if (id) return <Guidance model={model} store={store} indicatorId={id} />; break;
    case "region": if (id) return <RegionDetail region={id} assessment={model.assessment} indicators={model.indicators} />; break;
    case "comms": return param === "send"
      ? <SendReport model={model} store={store} indicatorId={new URLSearchParams(query).get("indicator")} />
      : <CommsPage model={model} store={store} />;
    case "library": return <Library model={model} />;
    case "settings": return <Settings model={model} store={store} />;
  }
  return (
    <div className="page-heading">
      <h1>Page unavailable</h1>
      <a href={CREW_ROUTE}>Return to crew health</a>
    </div>
  );
}

const backs: Record<string, [string, string]> = {
  guidance: ["#/crew/body", "Home"], region: ["#/crew/body", "Home"], checkin: ["#/crew/today", "Today"],
};

function Back({ route }: { route: string }) {
  const [path] = route.split("?");
  const [, , screen, param] = path.split("/");
  const to = screen === "comms" && param === "send" ? (["#/crew/comms", "Ground"] as [string, string])
    : screen === "measure" && param ? (["#/crew/measure", "Measure"] as [string, string]) : backs[screen];
  return to ? <a className="back-link" href={to[0]}>← Back to {to[1]}</a> : null;
}

function App({ store }: { store: CrewStore }) {
  const model = useSyncExternalStore(store.subscribe, store.getSnapshot);
  const route = useSyncExternalStore(subscribeRoute, () => location.hash || CREW_ROUTE);
  return (
    <>
      <TopBar model={model} />
      <div className="shell">
        <NavBar route={route} />
        <main className="content">
          <Back route={route} />
          <Page route={route} model={model} store={store} />
        </main>
      </div>
    </>
  );
}

const root = createRoot(document.getElementById("root")!);
root.render(<main className="content" role="status">Loading mission assessment…</main>);

void initEngine().then(() => {
  const store = createStore();
  root.render(<SafeModeBoundary><App store={store} /></SafeModeBoundary>);
  import.meta.hot?.dispose(() => { store.dispose(); root.unmount(); });
}).catch((error: unknown) => {
  console.error(error);
  root.render(
    <main className="content boot-error" role="alert">
      <h1>Mission assessment unavailable</h1>
      <p>The assessment could not be loaded.</p>
      <a className="button-link" href="./safe/">Open safe mode</a>
      <button onClick={() => location.reload()}>Reload</button>
    </main>,
  );
});
