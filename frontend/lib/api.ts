import type {
  UniverseFile,
  BacktestRequest,
  CreateBacktestResponse,
  BacktestStatusResponse,
  BacktestResults,
} from "@/lib/types";

const API_BASE = process.env.NEXT_PUBLIC_API_BASE_URL ?? "";

async function http<T>(path: string, init?: RequestInit): Promise<T> {
  const url = API_BASE ? `${API_BASE}${path}` : path;
  const res = await fetch(url, {
    ...init,
    headers: {
      "content-type": "application/json",
      ...(init?.headers ?? {}),
    },
  });

  if (!res.ok) {
    const text = await res.text();
    let msg = text;
    try {
      const j = JSON.parse(text);
      msg = j.error ?? text;
    } catch {}
    throw new Error(msg);
  }
  return res.json();
}

export function getUniverse() {
  return http<UniverseFile>("/api/universe");
}

export function createBacktest(req: BacktestRequest) {
  return http<CreateBacktestResponse>("/api/backtests", {
    method: "POST",
    body: JSON.stringify(req),
  });
}

export function getBacktestStatus(id: string) {
  return http<BacktestStatusResponse>(`/api/backtests/${id}`);
}

export function getBacktestResults(id: string) {
  return http<BacktestResults>(`/api/backtests/${id}/results`);
}
