use config::Config;
use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Settings {
    port: usize,
    entrypoint: String,
}

impl Settings {
    pub(crate) fn port(&self) -> &usize {
        &self.port
    }

    pub(crate) fn entrypoint(&self) -> &String {
        &self.entrypoint
    }
}

pub(crate) fn settings() -> Result<Settings, Box<dyn Error>> {
    let config_file = xdg::BaseDirectories::with_prefix("runtime")?
        .find_config_file("config.toml")
        .expect("config file should be present");

    let settings = Config::builder()
        .add_source(config::File::from(config_file))
        .add_source(config::Environment::with_prefix("RUNTIME_BACKEND"))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()?;

    Ok(settings)
}
