"use client";

import { useState } from "react";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { RunForm } from "@/components/backtest/run-form";
import type { StrategyId } from "@/lib/types";

type StrategyCard = {
  id: StrategyId;
  title: string;
  desc: string;
  defaults: any;
};

const strategies: StrategyCard[] = [
  {
    id: "ma_crossover",
    title: "A) Trend — MA Crossover",
    desc: "Classic trend-following: fast MA vs slow MA.",
    defaults: {
      fast: 20,
      slow: 100,
      sizing_mode: "fixed_notional",
      vol_lookback: 20,
      vol_target: 0.2,
      max_leverage: 1.5,
      stop_loss_pct: null
    }
  },
  {
    id: "zscore_mr",
    title: "B) Mean Reversion — Z-score Bands",
    desc: "Bollinger-like mean reversion using rolling z-score.",
    defaults: { lookback: 20, entry_z: 2.0, exit_z: 0.0, stop_loss_pct: 5.0, time_stop_bars: 50 }
  },
  {
    id: "donchian_breakout",
    title: "C) Breakout — Donchian Channel",
    desc: "Breaks out above/below prior N-bar channel (exclusive band).",
    defaults: { lookback: 20, atr_period: 14, atr_stop_mult: 3.0, trailing_stop: true }
  },
  {
    id: "pairs_zscore",
    title: "D) Pairs — Spread Z-score",
    desc: "Market-neutral-ish spread mean reversion using rolling hedge ratio.",
    defaults: { lookback: 60, entry_z: 2.0, exit_z: 0.0, hedge_method: "ols" }
  }
];

export function StrategyCards({ compact }: { compact: boolean }) {
  const [open, setOpen] = useState(false);
  const [selected, setSelected] = useState<StrategyCard | null>(null);

  const grid = compact ? "grid md:grid-cols-2 gap-4" : "grid lg:grid-cols-2 gap-4";

  return (
    <>
      <div className={grid}>
        {strategies.map((s) => (
          <Card key={s.id} className="p-5 space-y-3">
            <div className="text-lg font-semibold">{s.title}</div>
            <div className="text-neutral-300 text-sm">{s.desc}</div>
            <div>
              <Button
                onClick={() => {
                  setSelected(s);
                  setOpen(true);
                }}
              >
                Configure & Run
              </Button>
            </div>
          </Card>
        ))}
      </div>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="max-w-3xl">
          <DialogHeader>
            <DialogTitle>{selected?.title ?? "Run backtest"}</DialogTitle>
          </DialogHeader>
          {selected && (
            <RunForm
              strategyId={selected.id}
              defaultParams={selected.defaults}
              onLaunched={(id) => {
                setOpen(false);
                window.location.href = `/backtests/${id}`;
              }}
            />
          )}
        </DialogContent>
      </Dialog>
    </>
  );
}
