#[tauri::command]
pub fn config() -> Result<String, String> {
    match config::config() {
        Ok(config) => match serde_json::to_string(config.cockpit()) {
            Ok(cockpit_config_json) => Ok(cockpit_config_json),
            Err(error) => Err(format!("Failed to serialize config: {error}")),
        },
        Err(error) => Err(format!("Failed to get config: {error}")),
    }
}
