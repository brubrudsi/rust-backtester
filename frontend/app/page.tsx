import { CredibilityStrip } from "@/components/credibility-strip";
import { StrategyCards } from "@/components/strategy-cards";

export default function HomePage() {
  return (
    <div className="flex flex-col flex-1 min-h-0 space-y-8">
      <section className="space-y-3 shrink-0">
        <h1 className="text-4xl font-bold tracking-tight text-neutral-50">
          Rust Backtester
        </h1>
        <p className="text-neutral-300 max-w-3xl text-lg leading-relaxed">
          A chart-forward backtesting webapp with a robust Rust simulation engine
          (no lookahead, next-bar fills, costs, borrow/funding).
        </p>
      </section>

      <CredibilityStrip />

      <section className="flex flex-col flex-1 min-h-0 space-y-4">
        <div className="flex items-end justify-between shrink-0">
          <h2 className="text-2xl font-semibold text-neutral-100">Strategy Gallery</h2>
        </div>
        <StrategyCards compact />
      </section>
    </div>
  );
}
