import { useMemo, useState } from "react";
import { HazardIcon } from "../../../ui/icons";
import type { Hazard } from "../../../contract/Hazard";
import type { Indicator } from "../../../contract/Indicator";
import type { Region } from "../../../contract/Region";
import type { CrewStore } from "../../../store";
import { Card } from "../../../ui/Card";
import { SourceChip } from "../../../ui/SourceChip";
import "./Library.css";

// Presentation labels only. All indicator text comes from the registry.
const hazardLabels: Record<Hazard, string> = {
  radiation: "Radiation", isolation: "Isolation", distance: "Distance",
  gravity: "Gravity", closed_environment: "Closed environment",
};
const regionLabels: Record<Region, string> = {
  head: "Head", eyes: "Eyes", heart: "Heart", lungs: "Lungs", bones: "Bones",
  muscles: "Muscles", gut: "Gut", immune: "Immune", skin: "Skin", mind: "Mind",
};

function matches(indicator: Indicator, query: string) {
  const haystack = [
    indicator.name,
    regionLabels[indicator.region],
    ...indicator.hazards.map((hazard) => hazardLabels[hazard]),
  ].join(" ").toLowerCase();
  return haystack.includes(query);
}

export function Library({ model }: { model: ReturnType<CrewStore["getSnapshot"]> }) {
  const [filter, setFilter] = useState("");
  const shown = useMemo(() => {
    const query = filter.trim().toLowerCase();
    return query ? model.indicators.filter((indicator) => matches(indicator, query)) : model.indicators;
  }, [model.indicators, filter]);

  return (
    <>
      <div className="page-heading">
        <h1>Indicator library</h1>
        <p className="library-note">Ranges marked placeholder are not NASA data.</p>
      </div>
      <div className="library-controls">
        <label htmlFor="library-filter">Filter by name, region or hazard</label>
        <input
          id="library-filter"
          type="search"
          value={filter}
          onChange={(event) => setFilter(event.target.value)}
          autoComplete="off"
        />
        <p className="data-label" aria-live="polite">
          Showing {shown.length} of {model.indicators.length} indicators
        </p>
      </div>
      {shown.length === 0 ? <p className="next-step">No indicators match this filter.</p> : (
        <ul className="library-list">
          {shown.map((indicator) => (
            <li key={indicator.id}>
              <Card className="library-card">
                <div className="library-head">
                  <h2>{indicator.name}</h2>
                  <SourceChip status={indicator.source.status} />
                </div>
                <div className="library-meta">
                  <span className="unit-chip">{indicator.unit}</span>
                  <span className="unit-chip">{regionLabels[indicator.region]}</span>
                  {indicator.hazards.map((hazard) => (
                    <span key={hazard} className="hazard-chip"><HazardIcon hazard={hazard} />{hazardLabels[hazard]}</span>
                  ))}
                </div>
                <details>
                  <summary>Details</summary>
                  <div className="library-details">
                    <h3>What it is</h3>
                    <p>{indicator.what}</p>
                    <h3>Why it matters in space</h3>
                    <p>{indicator.why_in_space}</p>
                    <h3>How to measure</h3>
                    <ol>
                      {indicator.how_to_measure.map((step) => <li key={step}>{step}</li>)}
                    </ol>
                    {indicator.expected_change && (
                      <>
                        <h3>Expected change in flight</h3>
                        <p>{indicator.expected_change}</p>
                      </>
                    )}
                    <h3>Source</h3>
                    <p>
                      {indicator.source.url ? (
                        <a href={indicator.source.url} target="_blank" rel="noreferrer">{indicator.source.citation}</a>
                      ) : indicator.source.citation}
                    </p>
                  </div>
                </details>
              </Card>
            </li>
          ))}
        </ul>
      )}
    </>
  );
}
