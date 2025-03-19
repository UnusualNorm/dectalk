use std::{collections::HashMap, io};

use crate::dectalk::{DECtalkVoice, PAUL_VOICE};
use thiserror::Error;
use tokio::fs;

#[derive(Error, Debug)]
pub enum VoiceManagerError {
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub struct VoiceManager {
    pub voices: HashMap<u64, DECtalkVoice>,
}

impl VoiceManager {
    pub fn new() -> Self {
        Self {
            voices: HashMap::new(),
        }
    }

    pub fn get(&self, id: u64) -> &DECtalkVoice {
        self.voices.get(&id).unwrap_or(&PAUL_VOICE)
    }

    pub async fn remove(&mut self, id: u64) -> Result<(), VoiceManagerError> {
        self.voices.remove(&id);
        self.save().await?;
        Ok(())
    }

    pub async fn set(&mut self, id: u64, voice: &DECtalkVoice) -> Result<(), VoiceManagerError> {
        self.voices.insert(id, voice.clone());
        self.save().await?;
        Ok(())
    }

    pub async fn can_load(&self) -> bool {
        fs::metadata("data/voices.json").await.is_ok()
    }

    pub async fn load(&mut self) -> Result<(), VoiceManagerError> {
        let voices_string = fs::read_to_string("data/voices.json").await?;
        self.voices = serde_json::from_str(&voices_string)?;
        Ok(())
    }

    pub async fn save(&self) -> Result<(), VoiceManagerError> {
        let voices_string = serde_json::to_string(&self.voices)?;
        fs::write("data/voices.json", voices_string).await?;
        Ok(())
    }
}
