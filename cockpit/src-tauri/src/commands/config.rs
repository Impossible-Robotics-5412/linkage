use config::{CockpitConfig, LinkageConfig};
use std::sync::{Arc, Mutex};
use tauri::Runtime;

pub struct ConfigState {
    config: Arc<Mutex<LinkageConfig>>,
}

impl ConfigState {
    pub fn new(config: Arc<Mutex<LinkageConfig>>) -> Self {
        Self { config }
    }
}

#[tauri::command]
pub fn get_config(state: tauri::State<'_, ConfigState>) -> Result<String, String> {
    match serde_json::to_string(state.config.lock().unwrap().cockpit()) {
        Ok(cockpit_config_json) => Ok(cockpit_config_json),
        Err(error) => Err(format!("Failed to serialize config: {error}")),
    }
}

#[tauri::command]
pub fn set_cockpit_config<R: Runtime>(
    state: tauri::State<'_, ConfigState>,
    app: tauri::AppHandle<R>,
    cockpit_config_json: String,
) -> Result<(), String> {
    match serde_json::from_str::<CockpitConfig>(&cockpit_config_json) {
        Ok(cockpit_config) => {
            state.config.lock().unwrap().set_cockpit(cockpit_config);
            config::write_config_file(state.config.lock().unwrap().to_owned()).unwrap();
            app.restart();
            Ok(())
        }
        Err(error) => Err(format!("Failed to deserialize config: {error}")),
    }
}
