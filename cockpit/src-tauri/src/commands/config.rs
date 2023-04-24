use config::LinkageConfig;
use std::sync::Arc;

pub struct ConfigState {
    config: Arc<LinkageConfig>,
}

impl ConfigState {
    pub fn new(config: Arc<LinkageConfig>) -> Self {
        Self { config }
    }
}

#[tauri::command]
pub fn config(state: tauri::State<'_, ConfigState>) -> Result<String, String> {
    match serde_json::to_string(state.config.cockpit()) {
        Ok(cockpit_config_json) => Ok(cockpit_config_json),
        Err(error) => Err(format!("Failed to serialize config: {error}")),
    }
}
