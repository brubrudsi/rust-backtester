import { StrategyCards } from "@/components/strategy-cards";

export default function StrategiesPage() {
  return (
    <div className="space-y-4">
      <h1 className="text-2xl font-semibold">Strategy Gallery</h1>
      <p className="text-neutral-300 max-w-3xl">
        Four curated, bias-safe strategies. Click a card to configure and run a backtest.
      </p>
      <StrategyCards compact={false} />
    </div>
  );
}
