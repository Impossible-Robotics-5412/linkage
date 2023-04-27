use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use std::{process::Command, thread};
use systemstat::{saturating_sub_bytes, Platform, System};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub carburetor_status: bool,
    pub gauge_status: bool,
    pub linkage_socket_status: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cpu {
    pub user: f32,
    pub system: f32,
    pub idle: f32,
    pub temp: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Swap {
    pub used: u64,
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mem {
    pub used: u64,
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Memory {
    pub swap: Option<Swap>,
    pub mem: Option<Mem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemInfo {
    pub cpu: Option<Cpu>,
    pub memory: Memory,
    pub uptime: Option<u64>,
    pub service_info: ServiceInfo,
    pub robot_code_exists: bool,
}

impl SystemInfo {
    pub fn new(system: &System, delay: Duration) -> Self {
        let service_info = ServiceInfo {
            carburetor_status: service_is_active("carburetor.service"),
            gauge_status: service_is_active("gauge.service"),
            linkage_socket_status: service_is_active("linkage.socket"),
        };

        Self {
            cpu: get_cpu(system, delay),
            memory: Memory {
                swap: get_swap(system),
                mem: get_mem(system),
            },
            uptime: get_uptime(system),
            service_info,
            robot_code_exists: robot_code_exists(),
        }
    }
}

fn robot_code_exists() -> bool {
    Path::new("/home/linkage/robot_code/main").exists()
}

fn service_is_active(service_name: &str) -> bool {
    let exit_status = Command::new("systemctl")
        .args(["is-active", "--quiet", service_name])
        .status()
        .expect("failed to check service status");

    exit_status.success()
}

fn get_cpu_temp(system: &System) -> Option<f32> {
    match system.cpu_temp() {
        Ok(temp) => Some(temp),
        Err(error) => {
            println!("Failed finish getting CPU info: {}", error);
            None
        }
    }
}

fn get_cpu(system: &System, delay: Duration) -> Option<Cpu> {
    match system.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(delay);
            match cpu.done() {
                Ok(cpu_load) => Some(Cpu {
                    user: cpu_load.user * 100.0,
                    system: cpu_load.system * 100.0,
                    idle: cpu_load.idle * 100.0,
                    temp: get_cpu_temp(system),
                }),
                Err(error) => {
                    println!("Failed finish getting CPU info: {}", error);
                    None
                }
            }
        }
        Err(error) => {
            println!("Failed to get CPU info: {}", error);
            None
        }
    }
}

fn get_uptime(system: &System) -> Option<u64> {
    match system.uptime() {
        Ok(uptime) => Some(uptime.as_secs()),
        Err(error) => {
            println!("Failed to get uptime: {}", error);
            None
        }
    }
}

fn get_swap(system: &System) -> Option<Swap> {
    match system.swap() {
        Ok(swap) => Some(Swap {
            used: saturating_sub_bytes(swap.total, swap.free).as_u64(),
            total: swap.total.as_u64(),
        }),
        Err(error) => {
            println!("Failed to get swap: {}", error);
            None
        }
    }
}

fn get_mem(system: &System) -> Option<Mem> {
    match system.memory() {
        Ok(mem) => Some(Mem {
            used: saturating_sub_bytes(mem.total, mem.free).as_u64(),
            total: mem.total.as_u64(),
        }),
        Err(error) => {
            println!("Failed to get mem: {}", error);
            None
        }
    }
}

pub fn encode_system_info(system_info: &SystemInfo) -> String {
    // Jsonify the system info.
    let mut json_string = serde_json::to_string(system_info).unwrap();
    json_string.push('\n');
    json_string
}

pub fn decode_system_info_from_string(data: String) -> SystemInfo {
    serde_json::from_str(data.trim()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{
        decode_system_info_from_string, encode_system_info, Cpu, Mem, Memory, ServiceInfo, Swap,
        SystemInfo,
    };

    #[test]
    fn encode_decode() {
        let mock_system_info = SystemInfo {
            cpu: Some(Cpu {
                user: 42.0,
                system: 43.0,
                idle: 44.0,
                temp: Some(45.0),
            }),
            memory: Memory {
                swap: Some(Swap {
                    used: 46,
                    total: 47,
                }),
                mem: Some(Mem {
                    used: 48,
                    total: 49,
                }),
            },
            uptime: Some(50),
            service_info: ServiceInfo {
                carburetor_status: true,
                gauge_status: true,
                linkage_socket_status: true,
            },
            robot_code_exists: true,
        };

        let encoded = encode_system_info(&mock_system_info);
        let decoded = decode_system_info_from_string(encoded);

        assert_eq!(decoded, mock_system_info);
    }
}
