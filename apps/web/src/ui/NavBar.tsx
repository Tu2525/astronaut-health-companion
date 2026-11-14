const links = [
  ["body", "Body"], ["today", "Today"], ["measure", "Measure"],
  ["comms", "Ground"], ["library", "Library"], ["settings", "Settings"],
] as const;

export function NavBar({ route }: { route: string }) {
  const screen = route.split(/[/?]/)[2];
  return (
    <nav className="nav-bar" aria-label="Crew app">
      {links.map(([id, label]) => (
        <a key={id} href={`#/crew/${id}`} aria-current={screen === id ? "page" : undefined}>{label}</a>
      ))}
    </nav>
  );
}
