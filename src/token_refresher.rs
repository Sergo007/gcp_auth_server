use std::process::Command;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::{sync::RwLock, time::sleep};
use tracing::{error, info};

use crate::token_transformation::token_transformation;

#[allow(dead_code)]
pub struct TokenRefresher {
    value: Arc<RwLock<Option<String>>>,
    survey_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

#[allow(dead_code)]
impl TokenRefresher {
    pub fn arc_new() -> Arc<Self> {
        Arc::new(Self {
            value: Arc::new(RwLock::new(None)),
            survey_task: Arc::new(RwLock::new(None)),
        })
    }
    pub async fn survey(&self) {
        let value = self.value.clone();
        let mut survey_task = self.survey_task.write().await;
        *survey_task = Some(tokio::task::spawn_local(async move {
            loop {
                let token = get_token().await;
                if let Ok(val) = token {
                    let mut value_write = value.write().await;
                    *value_write = Some(val);
                }
                sleep(Duration::from_secs(41 * 60)).await;
            }
        }));
    }
    pub async fn get(&self) -> Option<String> {
        let value_read = self.value.read().await;
        value_read.clone()
    }
}

pub async fn get_token() -> Result<String, ()> {
    let start = SystemTime::now();
    let token = if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .arg("/C")
            .arg("gcloud")
            .arg("auth")
            .arg("print-identity-token")
            .output()
            .map_err(|e| {
                error!(
                    "Error: 'gcloud auth print-identity-token'  {}",
                    e.to_string()
                );
                ()
            })?;
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        let output = Command::new("gcloud")
            .arg("auth")
            .arg("print-identity-token")
            .output()
            .map_err(|e| {
                error!(
                    "Error: 'gcloud auth print-identity-token'  {}",
                    e.to_string()
                );
                ()
            })?;
        String::from_utf8_lossy(&output.stdout).to_string()
    };
    let end = SystemTime::now();
    let duration = end.duration_since(start.clone()).unwrap();
    info!(
        "gcloud auth print-identity-token, latency {duration} seconds",
        duration = (duration.as_millis() as f64) / 1000.0
    );
    Ok(token_transformation(&token))
}
