# rust-backtester

A backtesting platform for trading strategies, built with a Rust simulation engine and a Next.js frontend. Supports single-asset and pairs strategies across equities, ETFs, and crypto on daily or intraday timeframes.

The engine enforces strict execution semantics: signals are computed on bar close and orders fill at the next bar's open price, with configurable fees, slippage, and borrow/funding costs. Results include a full tear-sheet with interactive charts, trade markers, and a reproducible config.

**Disclaimer:** For educational and demonstration purposes only. Not investment advice.

---

## Architecture

```
backend/
  crates/
    engine/    Deterministic simulation engine (strategies, indicators, metrics)
    api/       Axum HTTP server, Polygon market data client, caching, run storage
frontend/      Next.js app (TradingView Lightweight Charts, Recharts, Tailwind)
```

**Backend** -- The engine crate is a pure computation library with no I/O. The API crate handles market data fetching (Polygon.io), file-based caching with TTL, async job execution, and run storage. Backtest jobs run asynchronously; the frontend polls for completion.

**Frontend** -- Candle charts with indicator overlays and trade entry/exit markers use TradingView Lightweight Charts. Equity curves, drawdown, rolling Sharpe, exposure, turnover, and returns histograms use Recharts. A trade blotter supports sorting and filtering. Each run's config is stored and can be copied as JSON for reproducibility.

---

## Strategies

| Strategy | Type | Description |
|---|---|---|
| MA Crossover | Single-asset | Fast vs. slow SMA. Optional vol-targeting and stop-loss. |
| Z-Score Mean Reversion | Single-asset | Rolling z-score entry/exit thresholds. Optional stop-loss and time stop. |
| Donchian Breakout | Single-asset | Breakout above/below rolling high/low channel. Optional ATR-based fixed or trailing stop. |
| Pairs Z-Score | Pairs (2 assets) | Spread z-score using rolling ratio or OLS beta hedge. Entry/exit on spread z thresholds. |

All strategies share the same execution model:

- Signals computed from data available at bar close (no lookahead)
- Orders fill at next bar open with slippage
- Per-side transaction costs (fee bps + slippage bps)
- Short borrow and crypto funding as annualized carry
- Deterministic: identical config and data produces identical results

---

## Quickstart

Requires Docker and a [Polygon.io](https://polygon.io) API key (free tier works).

```bash
cp .env.example .env
# set POLYGON_API_KEY in .env

docker compose up --build
```

- Frontend: http://localhost:3000
- API health: http://localhost:8080/api/healthz

Click **Run Demo Backtest** on the home page to run a preconfigured backtest without touching any settings.

---

## API

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/healthz` | Health check |
| `GET` | `/api/universe` | Available symbols and pairs |
| `POST` | `/api/backtests` | Submit a backtest job (returns run ID) |
| `GET` | `/api/backtests/:id` | Run status and summary metrics |
| `GET` | `/api/backtests/:id/results` | Full results (equity series, trades, indicators) |

---

## Configuration

Set in `.env` or as environment variables:

| Variable | Default | Description |
|---|---|---|
| `POLYGON_API_KEY` | -- | Required. Polygon.io API key. |
| `POLYGON_BASE_URL` | `https://api.polygon.io` | Market data endpoint. |
| `DATA_DIR` | `/data` | Root for cache and run storage. |
| `RUN_TTL_HOURS` | `24` | How long run results are kept before cleanup. |
| `CACHE_TTL_DAYS` | `30` | Market data cache lifetime. |
| `RATE_LIMIT_RPM` | `240` | API rate limit per IP. |
| `CORS_ALLOW_ORIGINS` | `http://localhost:3000` | Comma-separated allowed origins. |

---

## Production deployment

The repo includes a `docker-compose.prod.yml` and `Caddyfile` for single-server deployment with automatic TLS.

```bash
# Set DOMAIN and CADDY_EMAIL in .env
docker compose -f docker-compose.yml -f docker-compose.prod.yml up --build -d
```

Caddy reverse-proxies `/api/*` to the Rust backend and everything else to Next.js, and handles certificate provisioning automatically.

---

## Development

```bash
make fmt      # cargo fmt + pnpm format
make lint     # cargo clippy + pnpm lint
make test     # cargo test + pnpm test (Playwright)
make build    # cargo build --release + pnpm build
```

### Tech stack

**Backend:** Rust, Axum, Tokio, Reqwest, Serde, Chrono, Tower (CORS, compression, rate limiting), Tracing

**Frontend:** Next.js 14, React 18, TradingView Lightweight Charts, Recharts, Tailwind CSS, Radix UI, Playwright

---

## License

MIT
