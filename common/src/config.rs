use std::{error::Error, fmt::Display, str};

use serde::Deserialize;

pub type AddressHost = String;
pub type AddressPort = usize;

#[derive(Deserialize)]
pub struct Address {
    pub host: AddressHost,
    pub port: AddressPort,
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize)]
pub struct Config {
    linkage_lib: Box<LinkageLib>,
    runtime: Box<Runtime>,
    carburetor: Box<Carburetor>,
    cockpit_backend: Box<CockpitBackend>,
}

impl Config {
    pub fn linkage_lib(&self) -> &LinkageLib {
        &self.linkage_lib
    }

    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    pub fn carburetor(&self) -> &Carburetor {
        &self.carburetor
    }

    pub fn cockpit_backend(&self) -> &CockpitBackend {
        &self.cockpit_backend
    }
}

#[derive(Deserialize)]
pub struct LinkageLib {
    address: Address,
}

impl LinkageLib {
    pub fn address(&self) -> &Address {
        &self.address
    }
}

#[derive(Deserialize)]
pub struct Runtime {
    port: AddressPort,
    linkage_lib_entry_point: String,
    node_path: String,
    carburetor_path: String,
}

impl Runtime {
    pub fn port(&self) -> &AddressPort {
        &self.port
    }

    pub fn linkage_lib_entry_point(&self) -> &String {
        &self.linkage_lib_entry_point
    }

    pub fn node_path(&self) -> &String {
        &self.node_path
    }

    pub fn carburetor_path(&self) -> &String {
        &self.carburetor_path
    }
}

#[derive(Deserialize)]
pub struct Carburetor {
    port: AddressPort,
}

impl Carburetor {
    pub fn port(&self) -> AddressPort {
        self.port
    }
}

#[derive(Deserialize)]
pub struct CockpitBackend {
    port: AddressPort,
    runtime_address: Address,
    linkage_lib_address: Address,
}

impl CockpitBackend {
    pub fn port(&self) -> AddressPort {
        self.port
    }

    pub fn runtime_address(&self) -> &Address {
        &self.runtime_address
    }

    pub fn linkage_lib_address(&self) -> &Address {
        &self.linkage_lib_address
    }
}

pub fn config() -> Result<Config, Box<dyn Error>> {
    let config_path = xdg::BaseDirectories::with_prefix("linkage")?.get_config_file("config.toml");
    if !config_path.exists() {
        // FIXME: Create config file if it does not exist.
        panic!(
            "No config file found at {}",
            config_path.as_path().to_str().unwrap_or("UNKNOWN")
        );
    }
    let config_file = config::File::try_from(config_path)?;

    let config = config::Config::builder()
        .add_source(config::File::from(config_file))
        .add_source(config::Environment::with_prefix("LINKAGE"))
        .build()
        .unwrap()
        .try_deserialize::<Config>()?;

    Ok(config)
}
