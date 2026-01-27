import { Card } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";

export default function EngineeringPage() {
  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-semibold">Engineering / Notes</h1>

      <Card className="p-5 space-y-4">
        <h2 className="text-lg font-semibold">Architecture</h2>
        <pre className="text-sm text-neutral-200 bg-neutral-900 p-4 rounded-md overflow-auto">
{`Browser
  |
  |  Next.js (TypeScript) + Tailwind + minimal shadcn-style UI
  |  - TradingView Lightweight Charts (candles + overlays + markers)
  |  - Recharts (equity/drawdown/sharpe/hist/exposure)
  |
  v
Rust API (axum)
  - /api/universe
  - /api/backtests (job runner)
  - /api/backtests/{id}/results (stored JSON)
  - Rate limiting, tracing logs
  - Polygon/Massive data fetch + retries + disk cache
  |
  v
Rust Engine (crate)
  - Indicators
  - Strategy signals (no lookahead)
  - Next-bar execution
  - Costs + borrow/funding carry
  - Metrics + trades + series
  |
  v
Disk (volume)
  - cache/   (market data)
  - runs/    (configs + results + status)`}
        </pre>

        <Separator />

        <h2 className="text-lg font-semibold">Backtesting pitfalls handled</h2>
        <ul className="list-disc pl-5 text-neutral-300 space-y-1">
          <li>No lookahead: signals are based on bar-close data only.</li>
          <li>Next-bar execution: orders fill at next bar open (with slippage).</li>
          <li>Adjusted prices toggle (equities) with clear UI explanation.</li>
          <li>Transaction costs + slippage (bps per side) always applied.</li>
          <li>Short borrow and funding carry modeled (annualized, dt-aware).</li>
          <li>Time alignment: UTC timestamps; pairs use strict timestamp intersection.</li>
          <li>Missing bars/gaps: never forward-filled; gaps accrue carry over dt.</li>
          <li>Parameter sanity guards + bounded date ranges.</li>
          <li>Reproducible configs: stored with run results, copyable JSON.</li>
        </ul>

        <Separator />

        <h2 className="text-lg font-semibold">Local + VPS</h2>
        <div className="text-neutral-300 text-sm space-y-2">
          <p><code>docker compose up --build</code> runs web+api locally.</p>
          <p>
            For VPS TLS, use <code>docker-compose.prod.yml</code> + Caddy.
            Set <code>DOMAIN</code> and <code>CADDY_EMAIL</code> in <code>.env</code>.
          </p>
        </div>
      </Card>
    </div>
  );
}
