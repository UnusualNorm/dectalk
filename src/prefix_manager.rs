use std::{collections::HashMap, io};

use thiserror::Error;
use tokio::fs;

#[derive(Error, Debug)]
pub enum PrefixManagerError {
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub struct PrefixManager {
    pub default_prefix: String,
    pub prefixes: HashMap<u64, String>,
}

impl PrefixManager {
    pub fn new(default_prefix: &str) -> Self {
        Self {
            default_prefix: default_prefix.to_string(),
            prefixes: HashMap::new(),
        }
    }

    pub fn get(&self, guild_id: u64) -> &str {
        self.prefixes
            .get(&guild_id)
            .map(|s| s.as_str())
            .unwrap_or(&self.default_prefix)
    }

    pub async fn set(&mut self, guild_id: u64, prefix: &str) -> Result<(), PrefixManagerError> {
        self.prefixes.insert(guild_id, prefix.to_string());
        self.save().await?;
        Ok(())
    }

    pub async fn can_load(&self) -> bool {
        fs::metadata("data/prefixes.json").await.is_ok()
    }

    pub async fn load(&mut self) -> Result<(), PrefixManagerError> {
        let prefixes_string = fs::read_to_string("data/prefixes.json").await?;
        self.prefixes = serde_json::from_str(&prefixes_string)?;
        Ok(())
    }

    pub async fn save(&self) -> Result<(), PrefixManagerError> {
        let prefixes_string = serde_json::to_string(&self.prefixes)?;
        fs::write("data/prefixes.json", prefixes_string).await?;
        Ok(())
    }
}
