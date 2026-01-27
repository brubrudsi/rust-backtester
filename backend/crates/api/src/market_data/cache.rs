use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Clone)]
pub struct FileCache {
    base: PathBuf,
    ttl_days: u64,
}

impl FileCache {
    pub fn new(base: PathBuf, ttl_days: u64) -> Self {
        Self { base, ttl_days }
    }

    fn key_path(&self, key: &str) -> PathBuf {
        let safe = key
            .replace(':', "_")
            .replace('/', "_")
            .replace('\\', "_")
            .replace('?', "_")
            .replace('&', "_")
            .replace('=', "_");
        self.base.join(format!("{safe}.json"))
    }

    pub async fn get_json(&self, key: &str) -> anyhow::Result<Option<String>> {
        let path = self.key_path(key);
        if !path.exists() {
            return Ok(None);
        }
        if let Ok(meta) = fs::metadata(&path).await {
            if let Ok(modified) = meta.modified() {
                let age = modified.elapsed().unwrap_or_default();
                let ttl = std::time::Duration::from_secs(self.ttl_days * 24 * 3600);
                if age > ttl {
                    let _ = fs::remove_file(&path).await;
                    return Ok(None);
                }
            }
        }
        let bytes = fs::read(&path).await?;
        Ok(Some(String::from_utf8(bytes)?))
    }

    pub async fn put_json(&self, key: &str, value: &str) -> anyhow::Result<()> {
        fs::create_dir_all(&self.base).await?;
        let path = self.key_path(key);
        let tmp = path.with_extension("json.tmp");
        let mut f = fs::File::create(&tmp).await?;
        f.write_all(value.as_bytes()).await?;
        f.flush().await?;
        drop(f);
        fs::rename(tmp, path).await?;
        Ok(())
    }

    pub async fn ensure_dir(&self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.base).await?;
        Ok(())
    }

    pub fn base_dir(&self) -> &Path {
        &self.base
    }
}
