"use client";

import { useEffect, useMemo, useState } from "react";
import { createBacktest, getUniverse } from "@/lib/api";
import type { CostsRequest, StrategyId, UniverseFile } from "@/lib/types";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Select } from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";

export function RunForm({
  strategyId,
  defaultParams,
  onLaunched,
}: {
  strategyId: StrategyId;
  defaultParams: any;
  onLaunched: (id: string) => void;
}) {
  const [universe, setUniverse] = useState<UniverseFile | null>(null);
  const [symbols, setSymbols] = useState<string[]>(strategyId === "pairs_zscore" ? ["SPY", "QQQ"] : ["SPY"]);
  const [timeframe, setTimeframe] = useState("1D");
  const [start, setStart] = useState(() => {
    const d = new Date(Date.now() - 1000 * 60 * 60 * 24 * 365 * 2);
    return d.toISOString().slice(0, 10);
  });
  const [end, setEnd] = useState(() => new Date().toISOString().slice(0, 10));
  const [adjusted, setAdjusted] = useState(true);
  const [initialCapital, setInitialCapital] = useState("10000");

  const [feesBps, setFeesBps] = useState("1.0");
  const [slipBps, setSlipBps] = useState("1.0");
  const [borrowPct, setBorrowPct] = useState("0.0");
  const [fundPct, setFundPct] = useState("0.0");

  const [paramsJson, setParamsJson] = useState(JSON.stringify(defaultParams, null, 2));
  const [busy, setBusy] = useState(false);
  const [err, setErr] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const u = await getUniverse();
        setUniverse(u);
      } catch (e: any) {
        setErr(e?.message ?? "Failed loading universe");
      }
    })();
  }, []);

  useEffect(() => {
    if (strategyId === "pairs_zscore") setSymbols(["SPY", "QQQ"]);
    else setSymbols(["SPY"]);
  }, [strategyId]);

  const symbolOptions = useMemo(() => universe?.items ?? [], [universe]);

  function supportsAdjusted(ids: string[]) {
    const items = universe?.items ?? [];
    for (const id of ids) {
      const it = items.find((x) => x.id === id);
      if (!it?.supports_adjusted) return false;
    }
    return true;
  }

  async function submit() {
    setBusy(true);
    setErr(null);
    try {
      const costs: CostsRequest = {
        fees_bps: Number(feesBps),
        slippage_bps: Number(slipBps),
        borrow_annual_pct: Number(borrowPct),
        funding_annual_pct: Number(fundPct),
      };

      const params = JSON.parse(paramsJson);

      const payload = {
        strategy_id: strategyId,
        symbols,
        timeframe,
        start,
        end,
        adjusted: supportsAdjusted(symbols) ? adjusted : false,
        initial_capital: Number(initialCapital),
        params,
        costs,
      };

      const res = await createBacktest(payload as any);
      onLaunched(res.id);
    } catch (e: any) {
      setErr(e?.message ?? "Failed to start backtest");
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="space-y-4">
      {err && <div className="text-sm text-red-300">{err}</div>}

      <Card className="p-4 space-y-3">
        <div className="grid md:grid-cols-2 gap-3">
          <div className="space-y-2">
            <Label>Timeframe</Label>
            <Select value={timeframe} onChange={(e) => setTimeframe(e.target.value)}>
              <option value="1D">1D</option>
              <option value="1H">1H</option>
              <option value="5m">5m</option>
            </Select>
          </div>

          <div className="space-y-2">
            <Label>Initial capital</Label>
            <Input value={initialCapital} onChange={(e) => setInitialCapital(e.target.value)} />
          </div>

          <div className="space-y-2">
            <Label>Start (YYYY-MM-DD)</Label>
            <Input value={start} onChange={(e) => setStart(e.target.value)} />
          </div>
          <div className="space-y-2">
            <Label>End (YYYY-MM-DD)</Label>
            <Input value={end} onChange={(e) => setEnd(e.target.value)} />
          </div>
        </div>

        <div className="grid md:grid-cols-2 gap-3">
          <div className="space-y-2">
            <Label>Symbol(s)</Label>
            {strategyId !== "pairs_zscore" ? (
              <Select value={symbols[0]} onChange={(e) => setSymbols([e.target.value])}>
                {symbolOptions.map((s) => (
                  <option key={s.id} value={s.id}>
                    {s.display} — {s.name}
                  </option>
                ))}
              </Select>
            ) : (
              <div className="grid grid-cols-2 gap-2">
                <Select value={symbols[0]} onChange={(e) => setSymbols([e.target.value, symbols[1]])}>
                  {symbolOptions.map((s) => (
                    <option key={s.id} value={s.id}>
                      {s.display}
                    </option>
                  ))}
                </Select>
                <Select value={symbols[1]} onChange={(e) => setSymbols([symbols[0], e.target.value])}>
                  {symbolOptions.map((s) => (
                    <option key={s.id} value={s.id}>
                      {s.display}
                    </option>
                  ))}
                </Select>
              </div>
            )}
          </div>

          <div className="space-y-2">
            <Label>Adjusted prices (equities)</Label>
            <div className="flex items-center gap-3">
              <Switch
                checked={supportsAdjusted(symbols) ? adjusted : false}
                onCheckedChange={(v) => setAdjusted(v)}
                disabled={!supportsAdjusted(symbols)}
              />
              <span className="text-sm text-neutral-400">
                {!supportsAdjusted(symbols) ? "Disabled for crypto/non-adjustable symbols" : adjusted ? "On" : "Off"}
              </span>
            </div>
          </div>
        </div>
      </Card>

      <Card className="p-4 space-y-3">
        <div className="text-sm font-medium">Costs & carry</div>
        <div className="grid md:grid-cols-4 gap-3">
          <div className="space-y-2">
            <Label>Fees (bps)</Label>
            <Input value={feesBps} onChange={(e) => setFeesBps(e.target.value)} />
          </div>
          <div className="space-y-2">
            <Label>Slippage (bps)</Label>
            <Input value={slipBps} onChange={(e) => setSlipBps(e.target.value)} />
          </div>
          <div className="space-y-2">
            <Label>Borrow (%/yr)</Label>
            <Input value={borrowPct} onChange={(e) => setBorrowPct(e.target.value)} />
          </div>
          <div className="space-y-2">
            <Label>Funding (%/yr)</Label>
            <Input value={fundPct} onChange={(e) => setFundPct(e.target.value)} />
          </div>
        </div>
      </Card>

      <Card className="p-4 space-y-2">
        <div className="text-sm font-medium">Strategy params (JSON)</div>
        <textarea
          className="w-full h-52 bg-neutral-900 border border-neutral-800 rounded-md p-3 font-mono text-xs text-neutral-200"
          value={paramsJson}
          onChange={(e) => setParamsJson(e.target.value)}
        />
        <div className="text-xs text-neutral-400">
          This JSON is stored with the run and can be copied from the results page (“Reproduce”).
        </div>
      </Card>

      <div className="flex justify-end">
        <Button disabled={busy} onClick={submit}>
          {busy ? "Starting…" : "Run backtest"}
        </Button>
      </div>
    </div>
  );
}
