import { Card } from "@/components/ui/card";

export function MetricsCards({ metrics, diagnostics }: any) {
  const items = [
    ["End equity", metrics.end_equity.toFixed(2)],
    ["Total return", `${metrics.total_return_pct.toFixed(2)}%`],
    ["CAGR", `${metrics.cagr_pct.toFixed(2)}%`],
    ["Vol (ann.)", `${metrics.vol_annual_pct.toFixed(2)}%`],
    ["Sharpe", metrics.sharpe.toFixed(2)],
    ["Max drawdown", `${metrics.max_drawdown_pct.toFixed(2)}%`],
    ["Trades", `${metrics.trades}`],
    ["Win rate", `${metrics.win_rate_pct.toFixed(1)}%`],
    ["Profit factor", Number.isFinite(metrics.profit_factor) ? metrics.profit_factor.toFixed(2) : "∞"],
    ["Avg hold (bars)", metrics.avg_hold_bars.toFixed(1)],
    ["Avg turnover", metrics.turnover_avg.toFixed(3)],
    ["Bars/year (est.)", diagnostics.bars_per_year.toFixed(0)],
  ];

  return (
    <div className="grid md:grid-cols-3 lg:grid-cols-6 gap-3">
      {items.map(([k, v]) => (
        <Card key={k} className="p-3">
          <div className="text-xs text-neutral-400">{k}</div>
          <div className="text-lg font-semibold">{v}</div>
        </Card>
      ))}
    </div>
  );
}
