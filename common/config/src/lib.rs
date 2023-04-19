use std::error::Error;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub type AddressHost = String;
pub type AddressPort = usize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Address {
    pub host: AddressHost,
    pub port: AddressPort,
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    linkage_lib: Box<LinkageLib>,
    carburetor: Box<Carburetor>,
    cockpit: Box<Cockpit>,
    gauge: Box<Gauge>,
}

impl Config {
    pub fn linkage_lib(&self) -> &LinkageLib {
        &self.linkage_lib
    }

    pub fn carburetor(&self) -> &Carburetor {
        &self.carburetor
    }

    pub fn cockpit(&self) -> &Cockpit {
        &self.cockpit
    }

    pub fn gauge(&self) -> &Gauge {
        &self.gauge
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LinkageLib {
    port: AddressPort,
    carburetor_address: Address,
    logger_port: AddressPort,
}

impl LinkageLib {
    pub fn port(&self) -> &AddressPort {
        &self.port
    }

    pub fn carburetor_address(&self) -> &Address {
        &self.carburetor_address
    }

    pub fn logger_port(&self) -> &AddressPort {
        &self.logger_port
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Carburetor {
    port: AddressPort,
    logger_port: AddressPort,
}

impl Carburetor {
    pub fn port(&self) -> AddressPort {
        self.port
    }

    pub fn logger_port(&self) -> AddressPort {
        self.logger_port
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cockpit {
    linkage_lib_address: Address,
    linkage_socket_address: Address,

    cockpit_backend_logger_address: Address,
    linkage_lib_logger_address: Address,
    carburetor_logger_address: Address,
}

impl Cockpit {
    pub fn linkage_lib_address(&self) -> &Address {
        &self.linkage_lib_address
    }

    pub fn linkage_socket_address(&self) -> &Address {
        &self.linkage_socket_address
    }

    pub fn cockpit_backend_logger_address(&self) -> &Address {
        &self.cockpit_backend_logger_address
    }

    pub fn linkage_lib_logger_address(&self) -> &Address {
        &self.linkage_lib_logger_address
    }

    pub fn carburetor_logger_address(&self) -> &Address {
        &self.carburetor_logger_address
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Gauge {
    port: AddressPort,
}

impl Gauge {
    pub fn port(&self) -> AddressPort {
        self.port
    }
}

pub fn config() -> Result<Config, Box<dyn Error>> {
    let config_path = xdg::BaseDirectories::with_prefix("linkage")?.get_config_file("config.toml");
    if !config_path.exists() {
        panic!(
            "No config file found at {}",
            config_path.as_path().to_str().unwrap_or("UNKNOWN")
        );
    }

    let config_file = config_crate::File::try_from(config_path)?;

    let config = config_crate::Config::builder()
        .add_source(config_file)
        .add_source(config_crate::Environment::with_prefix("LINKAGE"))
        .build()
        .unwrap()
        .try_deserialize::<Config>()?;

    Ok(config)
}
