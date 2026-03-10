"use client";

import { useMemo } from "react";
import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer, CartesianGrid } from "recharts";
import { chartTheme } from "@/lib/chart-theme";

function histogram(values: number[], bins = 30) {
  const xs = values.slice(1).filter((x) => Number.isFinite(x));
  if (xs.length === 0) return [];
  const min = Math.min(...xs);
  const max = Math.max(...xs);
  const w = (max - min) / bins || 1e-9;

  const counts = new Array(bins).fill(0);
  for (const x of xs) {
    let idx = Math.floor((x - min) / w);
    idx = Math.max(0, Math.min(bins - 1, idx));
    counts[idx] += 1;
  }
  return counts.map((c, i) => ({
    bin: (min + i * w) * 100,
    count: c,
  }));
}

export function ReturnsHistogram({ returns }: { returns: number[] }) {
  const data = useMemo(() => histogram(returns, 30), [returns]);

  return (
    <div className="space-y-3">
      <div className="text-sm font-semibold">Returns distribution (per-bar)</div>
      <div className="h-72">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart data={data}>
            <CartesianGrid stroke={chartTheme.grid} strokeDasharray="3 3" />
            <XAxis
              dataKey="bin"
              tickFormatter={(x) => `${Number(x).toFixed(2)}%`}
              tick={{ fill: chartTheme.muted }}
              axisLine={{ stroke: chartTheme.axis }}
              tickLine={{ stroke: chartTheme.axis }}
            />
            <YAxis
              tick={{ fill: chartTheme.muted }}
              axisLine={{ stroke: chartTheme.axis }}
              tickLine={{ stroke: chartTheme.axis }}
            />
            <Tooltip
              contentStyle={{
                backgroundColor: chartTheme.tooltip.bg,
                border: `1px solid ${chartTheme.tooltip.border}`,
                color: chartTheme.text,
              }}
              labelStyle={{ color: chartTheme.text }}
              itemStyle={{ color: chartTheme.text }}
            />
            <Bar dataKey="count" fill={chartTheme.blue} />
          </BarChart>
        </ResponsiveContainer>
      </div>
      <div className="text-xs text-neutral-400">
        Simple returns (equity[i]/equity[i-1]-1).
      </div>
    </div>
  );
}
