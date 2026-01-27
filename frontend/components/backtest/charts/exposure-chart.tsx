"use client";

import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer, CartesianGrid } from "recharts";
import { chartTheme } from "@/lib/chart-theme";

export function ExposureChart({
  timestamps,
  net,
  gross,
  turnover,
}: {
  timestamps: number[];
  net: number[];
  gross: number[];
  turnover: number[];
}) {
  const rows = timestamps.map((t, i) => ({
    t,
    net: net[i],
    gross: gross[i],
    turnover: turnover[i],
  }));

  return (
    <div className="space-y-4">
      <div className="text-sm font-semibold">Exposure / Turnover</div>

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
            <Line type="monotone" dataKey="net" dot={false} stroke={chartTheme.cyan} strokeWidth={2} />
            <Line type="monotone" dataKey="gross" dot={false} stroke={chartTheme.purple} strokeWidth={2} />
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
            <Line type="monotone" dataKey="turnover" dot={false} stroke={chartTheme.blue} strokeWidth={2} />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
