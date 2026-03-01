# Rust Backtester (Rust Engine + Next.js UI)

A **chart-heavy, TradingView-ish** backtesting webapp designed as a **portfolio / hiring lever**:
- <2 minutes: click **Run Demo Backtest** → full tear-sheet + overlays + trade markers.
- <10 minutes: engineers see real infra/backtesting competence: Rust simulation, bias avoidance, caching, tests, CI, Docker.

**Disclaimer:** For educational/demo purposes only. Not investment advice.

---

## What’s inside

- **Rust engine** (`backend/crates/engine`)
  - No lookahead
  - Signals on bar close, orders execute at **next bar open**
  - Long/short, fees + slippage, borrow + funding accrual
  - Trades list + per-bar series + key metrics
- **Rust API** (`backend/crates/api`)
  - Polygon/Massive OHLCV via `/v2/aggs/ticker/.../range/...`
  - Retries w/ exponential backoff + rate-limit handling
  - Disk cache for market data
  - Anonymous run IDs, results stored on disk w/ TTL cleanup
  - Rate limiting, structured logs, health endpoint
- **Next.js frontend** (`frontend`)
  - TradingView Lightweight Charts candles + overlays + markers
  - Equity, drawdown, rolling Sharpe, returns histogram, exposure & turnover
  - Trade blotter with sorting + filtering
  - “Reproduce” button copies JSON config

---

## Backtesting pitfalls handled

- ✅ **No lookahead**: signals computed from data available at bar close
- ✅ **Next-bar execution**: fills occur at next bar open (with slippage)
- ✅ **Transaction costs**: fee bps + slippage bps per side
- ✅ **Short borrow / crypto funding**: simple annualized carry model
- ✅ **Time alignment + UTC hygiene**: timestamps stored/served in UTC ms
- ✅ **Missing bars + gaps**: no forward-fill; gaps accrue carry over dt
- ✅ **Parameter sanity guards**: lookback bounds, range bounds, etc.
- ✅ **Reproducible configs**: stored with run output, copyable JSON
- ✅ **Deterministic output**: identical inputs/config → identical results

---

## Local quickstart (Docker Compose)

### 1) Set env vars

Copy `.env.example` → `.env` and set:

- `POLYGON_API_KEY=...`

Optional:
- `POLYGON_BASE_URL=https://api.polygon.io` (or `https://api.massive.com`)
- `RUN_TTL_HOURS=24`
- `CACHE_TTL_DAYS=30`

### 2) Run

```bash
docker compose up --build
```

Open:
- Frontend: http://localhost:3000
- API: http://localhost:8080/api/healthz

---

## Deploy to a single Ubuntu VPS (Docker Compose + Caddy TLS)

See `docker-compose.prod.yml` + `Caddyfile`.

High-level:
1. Install Docker + Docker Compose plugin.
2. Copy repo to VPS.
3. Set `.env` (including `DOMAIN=yourdomain.com` and `CADDY_EMAIL=you@domain.com`).
4. Run:

```bash
docker compose -f docker-compose.yml -f docker-compose.prod.yml up --build -d
docker compose logs -f
```

Caddy will provision TLS automatically when DNS is pointing at the VPS.

---

## Repo navigation

```txt
backend/crates/engine   # deterministic Rust simulation engine
backend/crates/api      # axum API server + Polygon client + caching + run storage
frontend/               # Next.js UI (TradingView Lightweight Charts + Recharts)
```

---

## License

MIT. See `LICENSE`.
