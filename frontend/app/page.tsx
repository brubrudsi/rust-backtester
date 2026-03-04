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
          A chart-heavy backtesting webapp with a real Rust simulation engine
          (no lookahead, next-bar fills, costs, borrow/funding) — designed as a
          public GitHub portfolio repo.
        </p>
      </section>

      <CredibilityStrip />

      <section className="space-y-4">
        <div className="flex items-end justify-between">
          <h2 className="text-xl font-semibold">Strategy Gallery</h2>
        </div>
        <StrategyCards compact />
      </section>

      <Card className="p-5">
        <div className="text-sm text-neutral-300">
          <span className="font-medium text-neutral-100">Hiring-signal note:</span>{" "}
          this repo optimizes for correctness + clarity over “feature sprawl”. No auth,
          no uploads, no multi-tenant complexity.
        </div>
      </Card>
    </div>
  );
}
