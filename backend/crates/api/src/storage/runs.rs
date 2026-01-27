use crate::types::*;
use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};
use tokio::fs;
use uuid::Uuid;

#[derive(Clone)]
pub struct RunStore {
    base: PathBuf,
    ttl_hours: u64,
}

impl RunStore {
    pub fn new(data_dir: PathBuf, ttl_hours: u64) -> Self {
        Self {
            base: data_dir.join("runs"),
            ttl_hours,
        }
    }

    pub async fn init(&self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.base).await?;
        Ok(())
    }

    fn run_dir(&self, id: Uuid) -> PathBuf {
        self.base.join(id.to_string())
    }

    pub async fn create_run(&self, req: &BacktestRequest) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let dir = self.run_dir(id);
        fs::create_dir_all(&dir).await?;

        let now: DateTime<Utc> = Utc::now();

        fs::write(dir.join("config.json"), serde_json::to_vec_pretty(req)?).await?;

        let status = BacktestStatusFile {
            id,
            status: BacktestStatus::Queued,
            created_at: now.to_rfc3339(),
            updated_at: now.to_rfc3339(),
            message: None,
        };
        fs::write(dir.join("status.json"), serde_json::to_vec_pretty(&status)?).await?;

        Ok(id)
    }

    pub async fn set_status(&self, id: Uuid, status: BacktestStatus, message: Option<String>) -> anyhow::Result<()> {
        let dir = self.run_dir(id);
        let path = dir.join("status.json");
        let now: DateTime<Utc> = Utc::now();

        let mut cur: BacktestStatusFile = if path.exists() {
            serde_json::from_slice(&fs::read(&path).await?)?
        } else {
            BacktestStatusFile {
                id,
                status: BacktestStatus::Queued,
                created_at: now.to_rfc3339(),
                updated_at: now.to_rfc3339(),
                message: None,
            }
        };

        cur.status = status;
        cur.updated_at = now.to_rfc3339();
        cur.message = message;

        fs::write(path, serde_json::to_vec_pretty(&cur)?).await?;
        Ok(())
    }

    pub async fn write_results(&self, id: Uuid, results: &BacktestResults) -> anyhow::Result<()> {
        let dir = self.run_dir(id);
        fs::write(dir.join("results.json"), serde_json::to_vec_pretty(results)?).await?;

        let summary = BacktestSummary {
            end_equity: results.engine.metrics.end_equity,
            total_return_pct: results.engine.metrics.total_return_pct,
            sharpe: results.engine.metrics.sharpe,
            max_drawdown_pct: results.engine.metrics.max_drawdown_pct,
            trades: results.engine.metrics.trades,
        };
        fs::write(dir.join("summary.json"), serde_json::to_vec_pretty(&summary)?).await?;
        Ok(())
    }

    pub async fn read_status(&self, id: Uuid) -> anyhow::Result<BacktestStatusFile> {
        let dir = self.run_dir(id);
        let bytes = fs::read(dir.join("status.json")).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn read_summary(&self, id: Uuid) -> anyhow::Result<Option<BacktestSummary>> {
        let dir = self.run_dir(id);
        let path = dir.join("summary.json");
        if !path.exists() {
            return Ok(None);
        }
        let bytes = fs::read(path).await?;
        Ok(Some(serde_json::from_slice(&bytes)?))
    }

    pub async fn read_results(&self, id: Uuid) -> anyhow::Result<BacktestResults> {
        let dir = self.run_dir(id);
        let bytes = fs::read(dir.join("results.json")).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn cleanup_old_runs(&self) -> anyhow::Result<()> {
        let ttl = std::time::Duration::from_secs(self.ttl_hours * 3600);
        let mut rd = fs::read_dir(&self.base).await?;
        while let Some(entry) = rd.next_entry().await? {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let meta = fs::metadata(&path).await?;
            if let Ok(modified) = meta.modified() {
                if modified.elapsed().unwrap_or_default() > ttl {
                    let _ = fs::remove_dir_all(&path).await;
                }
            }
        }
        Ok(())
    }

    pub fn base_dir(&self) -> &Path {
        &self.base
    }
}
