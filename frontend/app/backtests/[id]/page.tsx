"use client";

import { useEffect, useMemo, useState } from "react";
import { getBacktestStatus, getBacktestResults } from "@/lib/api";
import type { BacktestResults, BacktestStatusResponse } from "@/lib/types";
import { ResultsDashboard } from "@/components/backtest/results-dashboard";
import { Card } from "@/components/ui/card";

export default function BacktestRunPage({ params }: { params: { id: string } }) {
  const id = params.id;

  const [status, setStatus] = useState<BacktestStatusResponse | null>(null);
  const [results, setResults] = useState<BacktestResults | null>(null);
  const [err, setErr] = useState<string | null>(null);

  const done = useMemo(
    () => status?.status === "complete" || status?.status === "failed",
    [status]
  );

  useEffect(() => {
    let alive = true;

    async function poll() {
      try {
        const s = await getBacktestStatus(id);
        if (!alive) return;
        setStatus(s);

        if (s.status === "complete") {
          const r = await getBacktestResults(id);
          if (!alive) return;
          setResults(r);
        } else if (s.status === "failed") {
          setErr(s.message ?? "Backtest failed");
        }
      } catch (e: any) {
        if (!alive) return;
        setErr(e?.message ?? "Failed loading backtest");
      }
    }

    poll();
    const t = setInterval(() => poll(), done ? 60_000 : 1200);

    return () => {
      alive = false;
      clearInterval(t);
    };
  }, [id, done]);

  if (err) {
    return (
      <Card className="p-5">
        <div className="text-red-300 font-medium">Error</div>
        <div className="text-neutral-300 mt-2">{err}</div>
      </Card>
    );
  }

  if (!status) {
    return (
      <Card className="p-5">
        <div className="text-neutral-300">Loading…</div>
      </Card>
    );
  }

  if (status.status !== "complete") {
    return (
      <Card className="p-5 space-y-2">
        <div className="text-lg font-semibold">Backtest {id}</div>
        <div className="text-neutral-300">
          Status: <span className="font-medium text-neutral-50">{status.status}</span>
        </div>
        {status.message && <div className="text-neutral-400 text-sm">{status.message}</div>}
        <div className="text-neutral-400 text-sm">
          This page auto-refreshes while the job runs.
        </div>
      </Card>
    );
  }

  if (!results) {
    return (
      <Card className="p-5">
        <div className="text-neutral-300">Loading results…</div>
      </Card>
    );
  }

  return <ResultsDashboard results={results} />;
}
