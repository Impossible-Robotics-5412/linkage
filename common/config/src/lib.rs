use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use xdg::BaseDirectoriesError;

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
pub struct LinkageConfig {
    linkage_lib: Box<LinkageLibConfig>,
    carburetor: Box<CarburetorConfig>,
    cockpit: Box<CockpitConfig>,
    gauge: Box<GaugeConfig>,
}

impl LinkageConfig {
    pub fn linkage_lib(&self) -> &LinkageLibConfig {
        &self.linkage_lib
    }

    pub fn carburetor(&self) -> &CarburetorConfig {
        &self.carburetor
    }

    pub fn cockpit(&self) -> &CockpitConfig {
        &self.cockpit
    }

    pub fn set_cockpit(&mut self, cockpit_config: CockpitConfig) {
        self.cockpit = Box::new(cockpit_config);
    }

    pub fn gauge(&self) -> &GaugeConfig {
        &self.gauge
    }
}

impl Default for LinkageConfig {
    fn default() -> Self {
        Self {
            linkage_lib: Box::new(LinkageLibConfig {
                port: 12362,
                carburetor_address: Address {
                    host: "0.0.0.0".to_string(),
                    port: 48862,
                },
                logger_port: 7640,
            }),
            carburetor: Box::new(CarburetorConfig {
                port: 48862,
                logger_port: 7644,
            }),
            cockpit: Box::new(CockpitConfig {
                linkage_lib_address: Address {
                    host: "raspberrypi.local".to_string(),
                    port: 12362,
                },
                linkage_socket_address: Address {
                    host: "raspberrypi.local".to_string(),
                    port: 9999,
                },
                gauge_address: Address {
                    host: "raspberrypi.local".to_string(),
                    port: 4226,
                },
                cockpit_backend_logger_address: Address {
                    host: "0.0.0.0".to_string(),
                    port: 7642,
                },
                linkage_lib_logger_address: Address {
                    host: "raspberrypi.local".to_string(),
                    port: 7640,
                },
                carburetor_logger_address: Address {
                    host: "raspberrypi.local".to_string(),
                    port: 7644,
                },
            }),
            gauge: Box::new(GaugeConfig { port: 4226 }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LinkageLibConfig {
    port: AddressPort,
    carburetor_address: Address,
    logger_port: AddressPort,
}

impl LinkageLibConfig {
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
pub struct CarburetorConfig {
    port: AddressPort,
    logger_port: AddressPort,
}

impl CarburetorConfig {
    pub fn port(&self) -> AddressPort {
        self.port
    }

    pub fn logger_port(&self) -> AddressPort {
        self.logger_port
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CockpitConfig {
    linkage_lib_address: Address,
    linkage_socket_address: Address,
    gauge_address: Address,

    cockpit_backend_logger_address: Address,
    linkage_lib_logger_address: Address,
    carburetor_logger_address: Address,
}

impl CockpitConfig {
    pub fn linkage_lib_address(&self) -> &Address {
        &self.linkage_lib_address
    }

    pub fn linkage_socket_address(&self) -> &Address {
        &self.linkage_socket_address
    }

    pub fn cockpit_backend_logger_address(&self) -> &Address {
        &self.cockpit_backend_logger_address
    }

    pub fn gauge_address(&self) -> &Address {
        &self.gauge_address
    }

    pub fn linkage_lib_logger_address(&self) -> &Address {
        &self.linkage_lib_logger_address
    }

    pub fn carburetor_logger_address(&self) -> &Address {
        &self.carburetor_logger_address
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GaugeConfig {
    port: AddressPort,
}

impl GaugeConfig {
    pub fn port(&self) -> AddressPort {
        self.port
    }
}

fn config_path() -> Result<PathBuf, BaseDirectoriesError> {
    let dir = xdg::BaseDirectories::with_prefix("linkage")?;
    Ok(dir.get_config_file("config.toml"))
}

pub fn config() -> Result<LinkageConfig, Box<dyn Error>> {
    let config_path = config_path()?;
    if !config_path.exists() {
        // FIXME: This should use the logger, but we can't access it from here.
        eprintln!("No config found. Using default config.");
        // No config found. Using default config.
        return Ok(LinkageConfig::default());
    }

    let file_content = std::fs::read_to_string(config_path)?;

    let config = toml::from_str(file_content.as_str())?;

    Ok(config)
}

pub fn write_config_file(config: LinkageConfig) -> Result<(), Box<dyn Error>> {
    let toml_string = toml::to_string_pretty(&config)?;
    let config_path = config_path()?;
    std::fs::write(config_path, toml_string)?;
    Ok(())
}
