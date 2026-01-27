"use client";

import type { BacktestResults } from "@/lib/types";
import { Card } from "@/components/ui/card";
import { MetricsCards } from "@/components/backtest/metrics-cards";
import { AssumptionsPanel } from "@/components/backtest/assumptions-panel";
import { TradeBlotter } from "@/components/backtest/trade-blotter";
import { TvCandles } from "@/components/backtest/charts/tv-candles";
import { TvLine } from "@/components/backtest/charts/tv-line";
import { EquityCharts } from "@/components/backtest/charts/equity-charts";
import { ExposureChart } from "@/components/backtest/charts/exposure-chart";
import { ReturnsHistogram } from "@/components/backtest/charts/returns-histogram";
import { Button } from "@/components/ui/button";

export function ResultsDashboard({ results }: { results: BacktestResults }) {
  const { meta, engine, bars, config } = results;

  const isPairs = meta.strategy_id === "pairs_zscore";
  const reproduceJson = JSON.stringify(config, null, 2);

  return (
    <div className="space-y-5">
      <Card className="p-5 space-y-2">
        <div className="flex flex-wrap items-start justify-between gap-3">
          <div>
            <div className="text-lg font-semibold">
              Backtest Results — {meta.symbols.join(isPairs ? " × " : "")}
            </div>
            <div className="text-sm text-neutral-400">
              {meta.timeframe_used} • {meta.start} → {meta.end} • adjusted: {meta.adjusted ? "on" : "off"}
            </div>
          </div>
          <Button
            variant="secondary"
            onClick={async () => {
              await navigator.clipboard.writeText(reproduceJson);
              alert("Copied JSON config to clipboard.");
            }}
          >
            Reproduce (copy JSON)
          </Button>
        </div>
      </Card>

      <Card className="p-4">
        {isPairs ? (
          <TvLine
            title="Spread (TradingView Lightweight Charts)"
            timestamps={engine.timestamps}
            series={[
              { name: "spread", values: (engine.indicators["spread"] ?? []).map((x) => x ?? null) }
            ]}
            markersFromTrades={engine.trades}
          />
        ) : (
          <TvCandles
            title="Price (TradingView Lightweight Charts)"
            bars={bars[0].bars}
            overlays={engine.indicators}
            markersFromTrades={engine.trades}
          />
        )}
      </Card>

      <MetricsCards metrics={engine.metrics} diagnostics={engine.diagnostics} />

      <div className="grid lg:grid-cols-2 gap-4">
        <Card className="p-4">
          <EquityCharts
            timestamps={engine.timestamps}
            equity={engine.equity}
            drawdown={engine.drawdown}
            rollingSharpe={engine.rolling_sharpe}
          />
        </Card>
        <Card className="p-4">
          <ExposureChart
            timestamps={engine.timestamps}
            net={engine.net_exposure}
            gross={engine.gross_exposure}
            turnover={engine.turnover}
          />
        </Card>
      </div>

      <div className="grid lg:grid-cols-2 gap-4">
        <Card className="p-4">
          <ReturnsHistogram returns={engine.returns} />
        </Card>
        <Card className="p-4">
          <AssumptionsPanel assumptions={results.assumptions} />
        </Card>
      </div>

      <Card className="p-4">
        <TradeBlotter trades={engine.trades} />
      </Card>
    </div>
  );
}
