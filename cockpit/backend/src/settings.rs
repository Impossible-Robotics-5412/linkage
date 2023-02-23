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
pub(crate) struct Settings {
    runtime: Address,
    linkage: Address,
}

impl Settings {
    pub(crate) fn runtime(&self) -> &Address {
        &self.runtime
    }

    pub(crate) fn linkage(&self) -> &Address {
        &self.linkage
    }
}

pub(crate) fn settings() -> Result<Settings, Box<dyn Error>> {
    let config_file = xdg::BaseDirectories::with_prefix("cockpit")?
        .find_config_file("config.toml")
        .expect("config file should be present");

    let settings = Config::builder()
        .add_source(config::File::from(config_file))
        .add_source(config::Environment::with_prefix("COCKPIT_BACKEND"))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()?;

    Ok(settings)
}
