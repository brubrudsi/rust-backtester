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
  regime: string;
  pitfall: string;
  defaults: any;
};

const strategies: StrategyCard[] = [
  {
    id: "ma_crossover",
    title: "A) Trend — MA Crossover",
    desc: "Classic trend-following: fast MA vs slow MA.",
    regime: "Works in sustained trends; whipsaws in sideways/chop.",
    pitfall: "Signals on close; fills at next open (no lookahead).",
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
    regime: "Works in range-bound markets; can get steamrolled in trends.",
    pitfall: "Entry/exit computed on close; no intrabar peeking.",
    defaults: { lookback: 20, entry_z: 2.0, exit_z: 0.0, stop_loss_pct: 5.0, time_stop_bars: 50 }
  },
  {
    id: "donchian_breakout",
    title: "C) Breakout — Donchian Channel",
    desc: "Breaks out above/below prior N-bar channel (exclusive band).",
    regime: "Works in breakout regimes; false breakouts hurt in chop.",
    pitfall: "Channel excludes current bar to avoid subtle lookahead.",
    defaults: { lookback: 20, atr_period: 14, atr_stop_mult: 3.0, trailing_stop: true }
  },
  {
    id: "pairs_zscore",
    title: "D) Pairs — Spread Z-score",
    desc: "Market-neutral-ish spread mean reversion using rolling hedge ratio.",
    regime: "Works when relationship is stable; fails on regime breaks.",
    pitfall: "Strict timestamp intersection; no forward-fill leakage.",
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
            <div className="text-neutral-400 text-sm">
              <span className="text-neutral-200 font-medium">Regime:</span> {s.regime}
            </div>
            <div className="text-neutral-400 text-sm">
              <span className="text-neutral-200 font-medium">Pitfall avoided:</span> {s.pitfall}
            </div>
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
