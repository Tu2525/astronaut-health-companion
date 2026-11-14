import type { Environment } from "../../../contract/Environment";
import type { CrewStore } from "../../../store";
import { Card } from "../../../ui/Card";
import "./Settings.css";

type Role = "astronaut" | "flight_surgeon";

const environments: [Environment, string][] = [
  ["station", "Station"], ["transit", "Mars transit"], ["lunar_base", "Lunar base"],
  ["mars_base", "Mars base"], ["earth_clinic", "Earth clinic"],
];
const roles: [Role, string][] = [["astronaut", "Astronaut"], ["flight_surgeon", "Flight surgeon"]];

const modelCredit = {
  name: "body.glb",
  text: "built from Z-Anatomy (based on BodyParts3D, DBCLS), licensed CC BY-SA 4.0. Merged into 10 region meshes in Blender for this project; the derived model is shared under the same licence.",
  url: "https://github.com/Z-Anatomy/Models-of-human-anatomy",
};

export function Settings({ model, store }: { model: ReturnType<CrewStore["getSnapshot"]>; store: CrewStore }) {
  const { settings } = model;

  function resetDemo() {
    if (!window.confirm("Reset the demo? This clears all readings, symptoms and settings saved on this device.")) return;
    try {
      Object.keys(localStorage)
        .filter((key) => key.startsWith("ahc."))
        .forEach((key) => localStorage.removeItem(key));
    } catch {
      // Storage blocked: reloading still returns the app to its seeded state.
    }
    location.reload();
  }

  return (
    <>
      <div className="page-heading">
        <h1>Settings</h1>
      </div>
      <div className="settings-grid">
        <Card>
          <h2>Mission</h2>
          <div className="field">
            <label htmlFor="setting-environment">Environment</label>
            <select
              id="setting-environment"
              value={settings.environment}
              onChange={(event) => store.updateSettings({ environment: event.target.value as Environment })}
            >
              {environments.map(([value, label]) => <option key={value} value={value}>{label}</option>)}
            </select>
          </div>
          <div className="field">
            <label htmlFor="setting-role">Role</label>
            <select
              id="setting-role"
              value={settings.role}
              onChange={(event) => store.updateSettings({ role: event.target.value as Role })}
            >
              {roles.map(([value, label]) => <option key={value} value={value}>{label}</option>)}
            </select>
          </div>
        </Card>

        <Card>
          <h2>AI explainer</h2>
          <label className="toggle">
            <input
              type="checkbox"
              checked={settings.ai_enabled}
              onChange={(event) => store.updateSettings({ ai_enabled: event.target.checked })}
            />
            <span>Turn on the AI explainer</span>
          </label>
          <p className="setting-note">Off by default. The AI only explains; status always comes from the engine.</p>
        </Card>

        <Card>
          <h2>Demo data</h2>
          <p className="setting-note">Clears everything saved on this device and restores the seeded demo.</p>
          <button type="button" className="danger-button" onClick={resetDemo}>Reset demo</button>
        </Card>

        <Card className="licences-card">
          <h2>Licences</h2>
          <h3>3D body model</h3>
          <p>
            <strong>{modelCredit.name}</strong> {modelCredit.text}{" "}
            <a href={modelCredit.url} target="_blank" rel="noreferrer">Z-Anatomy source</a>
          </p>
          <p className="setting-note">Any app that ships this model must keep this credit in its About or Licences screen.</p>
        </Card>
      </div>
    </>
  );
}
