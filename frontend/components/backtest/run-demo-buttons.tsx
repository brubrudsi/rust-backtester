"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { createBacktest } from "@/lib/api";

export function RunDemoButtons() {
  const [busy, setBusy] = useState(false);
  const [err, setErr] = useState<string | null>(null);

  async function runDemo(kind: "spy" | "eth") {
    setBusy(true);
    setErr(null);
    try {
      const now = new Date();
      const end = now.toISOString().slice(0, 10);
      const start = new Date(now.getTime() - 1000 * 60 * 60 * 24 * 365 * 2);
      const startStr = start.toISOString().slice(0, 10);

      const payload =
        kind === "spy"
          ? {
              strategy_id: "ma_crossover",
              symbols: ["SPY"],
              timeframe: "1D",
              start: startStr,
              end,
              adjusted: true,
              initial_capital: 10000,
              params: {
                fast: 20,
                slow: 100,
                sizing_mode: "fixed_notional",
                vol_lookback: 20,
                vol_target: 0.2,
                max_leverage: 1.5,
                stop_loss_pct: null
              },
              costs: { fees_bps: 1.0, slippage_bps: 1.0, borrow_annual_pct: 0.0, funding_annual_pct: 0.0 }
            }
          : {
              strategy_id: "donchian_breakout",
              symbols: ["ETH-USD"],
              timeframe: "1D",
              start: startStr,
              end,
              adjusted: false,
              initial_capital: 10000,
              params: { lookback: 20, atr_period: 14, atr_stop_mult: 3.0, trailing_stop: true },
              costs: { fees_bps: 2.0, slippage_bps: 2.0, borrow_annual_pct: 0.0, funding_annual_pct: 0.0 }
            };

      const res = await createBacktest(payload as any);
      window.location.href = `/backtests/${res.id}`;
    } catch (e: any) {
      setErr(e?.message ?? "Failed to launch demo");
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="flex flex-wrap gap-3 items-center">
      <Button disabled={busy} onClick={() => runDemo("spy")}>
        {busy ? "Launching…" : "Run Demo Backtest (SPY)"}
      </Button>
      <Button disabled={busy} variant="secondary" onClick={() => runDemo("eth")}>
        {busy ? "Launching…" : "Demo: ETH Breakout"}
      </Button>

      {err && <div className="text-sm text-red-300">{err}</div>}
    </div>
  );
}
