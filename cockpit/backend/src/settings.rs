use config::Config;
use std::{error::Error, fmt::Display};

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Address {
    host: String,
    port: usize,
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize)]
pub(crate) struct Port {
    port: usize,
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.port)
    }
}

#[derive(Deserialize)]
pub(crate) struct Settings {
    runtime: Address,
    linkage: Address,
    cockpit_backend: Port,
}

impl Settings {
    pub(crate) fn runtime_address(&self) -> &Address {
        &self.runtime
    }

    pub(crate) fn linkage_address(&self) -> &Address {
        &self.linkage
    }

    pub(crate) fn cockpit_backend_port(&self) -> &Port {
        &self.cockpit_backend
    }
}

pub(crate) fn settings() -> Result<Settings, Box<dyn Error>> {
    let config_path = xdg::BaseDirectories::with_prefix("cockpit")?.get_config_file("config.toml");
    if !config_path.exists() {
        panic!(
            "No config file found at {}",
            config_path.as_path().to_str().unwrap_or("UNKNOWN")
        );
    }
    let config_file = config::File::try_from(config_path)?;

    let settings = Config::builder()
        .add_source(config::File::from(config_file))
        .add_source(config::Environment::with_prefix("COCKPIT_BACKEND"))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()?;

    Ok(settings)
}
