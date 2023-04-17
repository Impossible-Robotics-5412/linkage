use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{process::Command, thread};
use systemstat::{saturating_sub_bytes, Platform, System};

const UPDATE_INTERVAL_MILLIS: u64 = 500;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub carburetor_status: bool,
    pub gauge_status: bool,
    pub linkage_socket_status: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cpu {
    pub user: f32,
    pub system: f32,
    pub idle: f32,
    pub temp: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swap {
    pub used: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mem {
    pub used: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub swap: Option<Swap>,
    pub mem: Option<Mem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub cpu: Option<Cpu>,
    pub memory: Memory,
    pub uptime: Option<u64>,
    pub service_info: ServiceInfo,
    pub robot_code_exists: bool,
}

impl SystemInfo {
    pub fn new(system: &systemstat::System) -> Self {
        let service_info = ServiceInfo {
            carburetor_status: service_is_active("carburetor.service"),
            gauge_status: service_is_active("gauge.service"),
            linkage_socket_status: service_is_active("linkage.socket"),
        };

        Self {
            cpu: get_cpu(system),
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
    match home::home_dir() {
        Some(home) => home.join("robot_code/main").is_file(),
        None => false,
    }
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

fn get_cpu(system: &System) -> Option<Cpu> {
    match system.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_millis(UPDATE_INTERVAL_MILLIS));
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
