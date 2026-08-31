use std::ops::{Deref, DerefMut};

use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;
use tokio::{fs, io, sync::RwLock};

#[derive(Error, Debug)]
pub enum SerdeDataError {
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("JSON error: {0}")]
    JSONError(#[from] serde_json::Error),
}

pub struct SerdeData<V> {
    path: String,
    lock: RwLock<V>,
}

impl<V: Serialize + DeserializeOwned + Default> SerdeData<V> {
    pub async fn new(name: &str) -> Result<Self, SerdeDataError> {
        let data = Self {
            path: format!("data/{}.json", name),
            lock: RwLock::new(V::default()),
        };
        if fs::metadata(&data.path).await.is_ok() {
            data.load().await?;
        }
        Ok(data)
    }

    pub async fn get(&self) -> impl Deref<Target = V> {
        self.lock.read().await
    }

    pub async fn get_mut(&self) -> impl DerefMut<Target = V> {
        self.lock.write().await
    }

    pub async fn load(&self) -> Result<(), SerdeDataError> {
        let data = fs::read(&self.path).await?;
        let value: V = serde_json::from_slice(&data)?;
        let mut lock = self.lock.write().await;
        *lock = value;
        Ok(())
    }

    pub async fn save(&self) -> Result<(), SerdeDataError> {
        let lock = self.lock.read().await;
        let data = serde_json::to_vec(&*lock)?;
        fs::write(&self.path, data).await?;
        Ok(())
    }
}
