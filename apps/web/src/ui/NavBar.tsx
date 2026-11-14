import { useEffect, useState } from "react";

const main = [["body", "Home"], ["today", "Today"], ["measure", "Measure"], ["comms", "Ground"]] as const;
const more = [["library", "Library"], ["settings", "Settings"]] as const;

// Left rail on desktop; fixed bottom tab bar on phones (More opens Library and Settings).
export function NavBar({ route }: { route: string }) {
  const screen = route.split(/[/?]/)[2];
  const [open, setOpen] = useState(false);
  useEffect(() => setOpen(false), [route]);
  const link = ([id, label]: readonly [string, string]) => (
    <a key={id} href={`#/crew/${id}`} aria-current={screen === id ? "page" : undefined}>{label}</a>
  );
  return (
    <nav className="rail" aria-label="Crew app">
      {main.map(link)}
      <button type="button" className="rail-more" aria-expanded={open} aria-controls="rail-more-list"
        data-active={more.some(([id]) => id === screen) || undefined} onClick={() => setOpen(!open)}>More</button>
      <div className="rail-sub" aria-hidden="true">More</div>
      <div id="rail-more-list" className="rail-more-list" data-open={open || undefined}>{more.map(link)}</div>
    </nav>
  );
}
