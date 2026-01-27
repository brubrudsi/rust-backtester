// Centralized, dark-mode-friendly chart colors.
//
// Why: Recharts and TradingView Lightweight Charts default to light-theme
// axis/tick styles. Explicitly setting colors avoids "dark on dark" issues and
// makes the demo look consistent.

export const chartTheme = {
  // Base
  bg: "#0a0a0a",
  text: "#e5e5e5",
  muted: "#a3a3a3",
  grid: "#262626",
  axis: "#404040",

  // Semantic
  up: "#22c55e", // green-500
  down: "#ef4444", // red-500
  exit: "#eab308", // amber-500

  // Accent palette
  blue: "#60a5fa", // blue-400
  purple: "#a78bfa", // violet-400
  cyan: "#22d3ee", // cyan-400
  pink: "#fb7185", // rose-400
  white: "#f5f5f5",

  tooltip: {
    bg: "#0a0a0a",
    border: "#262626",
  },
} as const;

export type ChartTheme = typeof chartTheme;
