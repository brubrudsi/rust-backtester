"use client";

import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer, CartesianGrid } from "recharts";
import { chartTheme } from "@/lib/chart-theme";

function toRows(timestamps: number[], a: number[], b?: number[], c?: Array<number | null>) {
  return timestamps.map((t, i) => ({
    t,
    equity: a[i],
    dd: b ? b[i] : undefined,
    sharpe: c ? (c[i] ?? null) : undefined,
  }));
}

export function EquityCharts({
  timestamps,
  equity,
  drawdown,
  rollingSharpe,
}: {
  timestamps: number[];
  equity: number[];
  drawdown: number[];
  rollingSharpe: Array<number | null>;
}) {
  const rows = toRows(timestamps, equity, drawdown, rollingSharpe);

  return (
    <div className="space-y-4">
      <div className="text-sm font-semibold">Equity / Drawdown / Rolling Sharpe</div>

      <div className="h-56">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={rows}>
            <CartesianGrid stroke={chartTheme.grid} strokeDasharray="3 3" />
            <XAxis
              dataKey="t"
              tickFormatter={() => ""}
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
            <Line type="monotone" dataKey="equity" dot={false} stroke={chartTheme.up} strokeWidth={2} />
          </LineChart>
        </ResponsiveContainer>
      </div>

      <div className="h-40">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={rows}>
            <CartesianGrid stroke={chartTheme.grid} strokeDasharray="3 3" />
            <XAxis
              dataKey="t"
              tickFormatter={() => ""}
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
            <Line type="monotone" dataKey="dd" dot={false} stroke={chartTheme.down} strokeWidth={2} />
          </LineChart>
        </ResponsiveContainer>
      </div>

      <div className="h-40">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={rows}>
            <CartesianGrid stroke={chartTheme.grid} strokeDasharray="3 3" />
            <XAxis
              dataKey="t"
              tickFormatter={() => ""}
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
            <Line type="monotone" dataKey="sharpe" dot={false} stroke={chartTheme.cyan} strokeWidth={2} />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
