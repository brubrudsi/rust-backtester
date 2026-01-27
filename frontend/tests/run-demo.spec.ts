import { test, expect } from "@playwright/test";
import demo from "./fixtures/demo_results.json";

test("Run Demo button renders results page", async ({ page }) => {
  await page.route("**/api/backtests", async (route) => {
    const body = JSON.stringify({
      id: "00000000-0000-0000-0000-000000000000",
      status_url: "/api/backtests/00000000-0000-0000-0000-000000000000",
      results_url: "/api/backtests/00000000-0000-0000-0000-000000000000/results",
    });
    await route.fulfill({ status: 200, body, contentType: "application/json" });
  });

  await page.route("**/api/backtests/00000000-0000-0000-0000-000000000000", async (route) => {
    const body = JSON.stringify({
      id: "00000000-0000-0000-0000-000000000000",
      status: "complete",
      message: null,
      summary: {
        end_equity: 10250,
        total_return_pct: 2.5,
        sharpe: 0.0,
        max_drawdown_pct: -0.6,
        trades: 1,
      },
      links: {
        self_url: "/api/backtests/00000000-0000-0000-0000-000000000000",
        results_url: "/api/backtests/00000000-0000-0000-0000-000000000000/results",
      },
    });
    await route.fulfill({ status: 200, body, contentType: "application/json" });
  });

  await page.route("**/api/backtests/00000000-0000-0000-0000-000000000000/results", async (route) => {
    await route.fulfill({ status: 200, body: JSON.stringify(demo), contentType: "application/json" });
  });

  await page.goto("/");
  await page.getByRole("button", { name: /Run Demo Backtest/i }).click();

  await expect(page).toHaveURL(/\/backtests\/00000000-0000-0000-0000-000000000000/);
  await expect(page.getByText(/Backtest Results/i)).toBeVisible();
  await expect(page.getByText(/Trade blotter/i)).toBeVisible();
});
