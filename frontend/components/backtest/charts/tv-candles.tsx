"use client";

import { useEffect, useRef } from "react";
import { createChart, CrosshairMode, type IChartApi } from "lightweight-charts";
import type { Bar, Trade } from "@/lib/types";
import { chartTheme } from "@/lib/chart-theme";

function toSec(ms: number) {
  return Math.floor(ms / 1000) as any;
}

export function TvCandles({
  title,
  bars,
  overlays,
  markersFromTrades,
}: {
  title: string;
  bars: Bar[];
  overlays: Record<string, Array<number | null>>;
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

    const candle = chart.addCandlestickSeries({
      upColor: chartTheme.up,
      downColor: chartTheme.down,
      borderUpColor: chartTheme.up,
      borderDownColor: chartTheme.down,
      wickUpColor: chartTheme.up,
      wickDownColor: chartTheme.down,
    });

    candle.setData(
      bars.map((b) => ({
        time: toSec(b.t),
        open: b.o,
        high: b.h,
        low: b.l,
        close: b.c,
      }))
    );

    const candidates = [
      "fast_ma",
      "slow_ma",
      "mean",
      "upper",
      "lower",
      "donchian_upper",
      "donchian_lower",
    ];
    const overlayColors: Record<string, string> = {
      fast_ma: chartTheme.blue,
      slow_ma: chartTheme.purple,
      mean: chartTheme.white,
      upper: chartTheme.exit,
      lower: chartTheme.exit,
      donchian_upper: chartTheme.cyan,
      donchian_lower: chartTheme.cyan,
    };
    for (const k of candidates) {
      const series = overlays[k];
      if (!series) continue;
      const line = chart.addLineSeries({ lineWidth: 2, color: overlayColors[k] ?? chartTheme.white });
      line.setData(
        series
          .map((v, i) => (v == null ? null : { time: toSec(bars[i].t), value: Number(v) }))
          .filter(Boolean) as any
      );
    }

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
    candle.setMarkers(markers);

    const ro = new ResizeObserver(() => {
      chart.applyOptions({ width: ref.current?.clientWidth ?? 800 });
    });
    ro.observe(ref.current);

    return () => {
      ro.disconnect();
      chart.remove();
    };
  }, [bars, overlays, markersFromTrades]);

  return (
    <div className="space-y-2">
      <div className="text-sm font-semibold">{title}</div>
      <div ref={ref} className="w-full" />
      <div className="text-xs text-neutral-400">
        Overlays are strategy indicators; markers are entries/exits. Fills occur at next bar open.
      </div>
    </div>
  );
}
