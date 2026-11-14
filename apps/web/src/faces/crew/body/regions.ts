import type { Hazard } from "../../../contract/Hazard";
import type { Region } from "../../../contract/Region";
import type { CrewStore } from "../../../store";

export type Model = ReturnType<CrewStore["getSnapshot"]>;

// Presentation only: labels and display order. Medical text and status come from the engine.
export const REGIONS: { id: Region; name: string }[] = [
  { id: "head", name: "Head and brain" }, { id: "eyes", name: "Eyes" },
  { id: "mind", name: "Mind and mood" }, { id: "lungs", name: "Lungs" },
  { id: "heart", name: "Heart and circulation" }, { id: "gut", name: "Gut and nutrition" },
  { id: "immune", name: "Immune system" }, { id: "skin", name: "Skin" },
  { id: "muscles", name: "Muscles" }, { id: "bones", name: "Bones" },
];

export const regionName = (region: Region) => REGIONS.find(({ id }) => id === region)!.name;

export const HAZARDS: { id: Hazard; label: string }[] = [
  { id: "radiation", label: "Radiation" },
  { id: "isolation", label: "Isolation and confinement" },
  { id: "distance", label: "Distance from Earth" },
  { id: "gravity", label: "Gravity fields" },
  { id: "closed_environment", label: "Closed environment" },
];
