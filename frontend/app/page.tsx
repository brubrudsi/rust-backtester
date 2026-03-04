import { CredibilityStrip } from "@/components/credibility-strip";
import { StrategyCards } from "@/components/strategy-cards";
import { RunDemoButtons } from "@/components/backtest/run-demo-buttons";

export default function HomePage() {
  return (
    <div className="space-y-8">
      <section className="space-y-3">
        <h1 className="text-3xl font-semibold tracking-tight">
          Rust Backtester
        </h1>
        <p className="text-neutral-300 max-w-3xl">
          A chart-heavy backtesting webapp with a real Rust simulation engine
          (no lookahead, next-bar fills, costs, borrow/funding) — designed as a
          public GitHub portfolio repo.
        </p>

        <RunDemoButtons />
      </section>

      <CredibilityStrip />

      <section className="space-y-4">
        <div className="flex items-end justify-between">
          <h2 className="text-xl font-semibold">Strategy Gallery</h2>
        </div>
        <StrategyCards compact />
      </section>
    </div>
  );
}
