"use client";

import { useEffect, useRef } from "react";
import { createChart, CrosshairMode, type IChartApi } from "lightweight-charts";
import type { Trade } from "@/lib/types";
import { chartTheme } from "@/lib/chart-theme";

function toSec(ms: number) {
  return Math.floor(ms / 1000) as any;
}

export function TvLine({
  title,
  timestamps,
  series,
  markersFromTrades,
}: {
  title: string;
  timestamps: number[];
  series: { name: string; values: Array<number | null> }[];
  markersFromTrades: Trade[];
}) {
  const ref = useRef<HTMLDivElement | null>(null);
  const chartRef = useRef<IChartApi | null>(null);

  useEffect(() => {
    if (!ref.current) return;

    const chart = createChart(ref.current, {
      height: 420,
      layout: {
        background: { color: chartTheme.bg as any },
        textColor: chartTheme.text as any,
      },
      grid: {
        vertLines: { color: chartTheme.grid as any },
        horzLines: { color: chartTheme.grid as any },
      },
      crosshair: { mode: CrosshairMode.Normal },
      timeScale: { timeVisible: true, secondsVisible: false },
    });

    chartRef.current = chart;

    const linePalette = [chartTheme.blue, chartTheme.purple, chartTheme.cyan, chartTheme.pink, chartTheme.white];
    const lines = series.map((_, idx) => chart.addLineSeries({ lineWidth: 2, color: linePalette[idx % linePalette.length] }));

    series.forEach((s, idx) => {
      lines[idx].setData(
        s.values
          .map((v, i) => (v == null ? null : { time: toSec(timestamps[i]), value: v }))
          .filter(Boolean) as any
      );
    });

    const markers: any[] = [];
    for (const t of markersFromTrades) {
      const isShort = t.direction.includes("short");
      const entryColor = isShort ? chartTheme.down : chartTheme.up;
      markers.push({
        time: toSec(t.entry_time),
        position: isShort ? "aboveBar" : "belowBar",
        shape: isShort ? "arrowDown" : "arrowUp",
        color: entryColor,
        textColor: chartTheme.text,
        text: `Entry #${t.id}`,
      });
      markers.push({
        time: toSec(t.exit_time),
        position: isShort ? "belowBar" : "aboveBar",
        shape: "circle",
        color: chartTheme.exit,
        textColor: chartTheme.text,
        text: `Exit #${t.id}`,
      });
    }
    lines[0].setMarkers(markers);

    const ro = new ResizeObserver(() => {
      chart.applyOptions({ width: ref.current?.clientWidth ?? 800 });
    });
    ro.observe(ref.current);

    return () => {
      ro.disconnect();
      chart.remove();
    };
  }, [timestamps, series, markersFromTrades]);

  return (
    <div className="space-y-2">
      <div className="text-sm font-semibold">{title}</div>
      <div ref={ref} className="w-full" />
      <div className="text-xs text-neutral-400">
        Pairs strategy plots the spread line (TradingView Lightweight Charts).
      </div>
    </div>
  );
}
