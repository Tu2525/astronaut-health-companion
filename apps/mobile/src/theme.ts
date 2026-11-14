// Mirrors design/tokens.css (same names, camelCased). Change values there first.
// This is the only file with colour literals.
export const colors = {
  statusNominal: "#3ecf8e",
  statusWatch: "#4c9aff",
  statusCaution: "#f5a524",
  statusUrgent: "#f0445a",
  statusUnknown: "#8a93a6",
  bg: "#0b0f17",
  surface: "#121826",
  surfaceRaised: "#1a2233",
  border: "#263046",
  text: "#e6eaf2",
  textMuted: "#8a93a6",
  textOnStatus: "#0b0f17",
  accent: "#7c9cff",
  focusRing: "#a9bcff",
} as const;

export const radius = { sm: 6, md: 12, lg: 20 } as const;
export const space = { 1: 4, 2: 8, 3: 12, 4: 16, 6: 24, 8: 32 } as const;
export const touchTarget = 48;
