"use client";

import { useMemo, useState } from "react";
import type { Trade } from "@/lib/types";
import { Input } from "@/components/ui/input";
import { Table, TBody, TD, TH, THead, TR } from "@/components/ui/table";

export function TradeBlotter({ trades }: { trades: Trade[] }) {
  const [q, setQ] = useState("");

  const filtered = useMemo(() => {
    if (!q.trim()) return trades;
    const needle = q.toLowerCase();
    return trades.filter((t) => t.direction.toLowerCase().includes(needle));
  }, [q, trades]);

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between gap-3">
        <div className="text-sm font-semibold">Trade blotter</div>
        <Input
          className="max-w-xs"
          placeholder="filter by direction…"
          value={q}
          onChange={(e) => setQ(e.target.value)}
        />
      </div>

      <div className="text-xs text-neutral-400">
        PnL includes estimated financing and recorded costs (fees + slippage). Times are Unix ms (UTC).
      </div>

      <div className="overflow-auto border border-neutral-800 rounded-md">
        <Table>
          <THead>
            <TR>
              <TH>ID</TH>
              <TH>Direction</TH>
              <TH>Entry</TH>
              <TH>Exit</TH>
              <TH>Hold</TH>
              <TH className="text-right">PnL</TH>
              <TH className="text-right">PnL%</TH>
            </TR>
          </THead>
          <TBody>
            {filtered.map((t) => (
              <TR key={t.id}>
                <TD>{t.id}</TD>
                <TD>{t.direction}</TD>
                <TD>{t.entry_time}</TD>
                <TD>{t.exit_time}</TD>
                <TD>{t.holding_period_bars}</TD>
                <TD className="text-right">{t.pnl.toFixed(2)}</TD>
                <TD className="text-right">{t.pnl_pct.toFixed(2)}%</TD>
              </TR>
            ))}
          </TBody>
        </Table>
      </div>
    </div>
  );
}
