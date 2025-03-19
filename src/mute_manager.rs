use std::{
    collections::{HashMap, HashSet},
    io,
};

use thiserror::Error;
use tokio::fs;

#[derive(Error, Debug)]
pub enum MuteManagerError {
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub struct MuteManager {
    pub mutes: HashMap<u64, HashSet<u64>>,
}

impl MuteManager {
    pub fn new() -> Self {
        Self {
            mutes: HashMap::new(),
        }
    }

    pub fn get(&self, guild_id: u64, user_id: u64) -> bool {
        let mutes = match self.mutes.get(&guild_id) {
            Some(mutes) => mutes,
            None => return false,
        };

        mutes.contains(&user_id)
    }

    pub async fn set(
        &mut self,
        guild_id: u64,
        user_id: u64,
        muted: bool,
    ) -> Result<(), MuteManagerError> {
        let mutes = self.mutes.entry(guild_id).or_insert_with(HashSet::new);
        if muted {
            mutes.insert(user_id);
        } else {
            mutes.remove(&user_id);
        }
        self.save().await?;
        Ok(())
    }

    pub async fn can_load(&self) -> bool {
        fs::metadata("data/mutes.json").await.is_ok()
    }

    pub async fn load(&mut self) -> Result<(), MuteManagerError> {
        let mutes_string = fs::read_to_string("data/mutes.json").await?;
        self.mutes = serde_json::from_str(&mutes_string)?;
        Ok(())
    }

    pub async fn save(&self) -> Result<(), MuteManagerError> {
        let mutes_string = serde_json::to_string(&self.mutes)?;
        fs::write("data/mutes.json", mutes_string).await?;
        Ok(())
    }
}
