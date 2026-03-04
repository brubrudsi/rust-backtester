import { CredibilityStrip } from "@/components/credibility-strip";
import { StrategyCards } from "@/components/strategy-cards";
import { Card } from "@/components/ui/card";

export default function HomePage() {
  return (
    <div className="space-y-8">
      <section className="space-y-3">
        <h1 className="text-3xl font-semibold tracking-tight">
          Rust Backtester
        </h1>
        <p className="text-neutral-300 max-w-3xl">
          A chart-forward backtesting webapp with a robust Rust simulation engine
          (no lookahead, next-bar fills, costs, borrow/funding).
        </p>
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
