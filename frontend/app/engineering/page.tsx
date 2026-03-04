import { Card } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";

export default function EngineeringPage() {
  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-semibold">Engineering / Notes</h1>

      <Card className="p-5 space-y-4">
        <h2 className="text-lg font-semibold">Architecture</h2>
        <div className="text-sm text-neutral-200 bg-neutral-900 p-4 rounded-md space-y-4">
          <section className="space-y-1">
            <h3 className="font-semibold text-neutral-100">1) Frontend</h3>
            <ul className="list-disc pl-5 text-neutral-300 space-y-0.5">
              <li>Next.js (TypeScript) + Tailwind + minimal shadcn-style UI.</li>
              <li>TradingView Lightweight Charts for candles, overlays, and markers.</li>
              <li>Recharts for equity, drawdown, Sharpe, histogram, and exposure views.</li>
            </ul>
          </section>

          <section className="space-y-1">
            <h3 className="font-semibold text-neutral-100">2) Rust API (axum)</h3>
            <ul className="list-disc pl-5 text-neutral-300 space-y-0.5">
              <li><code>/api/universe</code></li>
              <li><code>/api/backtests</code> (job runner)</li>
              <li><code>/api/backtests/{`{id}`}/results</code> (stored JSON)</li>
              <li>Rate limiting and tracing logs.</li>
              <li>Polygon/Massive data fetch with retries and disk cache.</li>
            </ul>
          </section>

          <section className="space-y-1">
            <h3 className="font-semibold text-neutral-100">3) Rust engine crate</h3>
            <ul className="list-disc pl-5 text-neutral-300 space-y-0.5">
              <li>Indicators.</li>
              <li>Strategy signals (no lookahead).</li>
              <li>Next-bar execution.</li>
              <li>Costs plus borrow/funding carry.</li>
              <li>Metrics, trades, and series output.</li>
            </ul>
          </section>

          <section className="space-y-1">
            <h3 className="font-semibold text-neutral-100">4) Disk volume</h3>
            <ul className="list-disc pl-5 text-neutral-300 space-y-0.5">
              <li><code>cache/</code> for market data.</li>
              <li><code>runs/</code> for configs, results, and status.</li>
            </ul>
          </section>
        </div>

        <Separator />

        <h2 className="text-lg font-semibold">Backtesting pitfalls handled</h2>
        <ul className="list-disc pl-5 text-neutral-300 space-y-1">
          <li>No lookahead: signals are based on bar-close data only.</li>
          <li>Next-bar execution: orders fill at next bar open (with slippage).</li>
          <li>Adjusted prices toggle for equities.</li>
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
