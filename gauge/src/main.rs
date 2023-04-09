extern crate systemstat;

use serde::Serialize;
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

const INTERVAL_MILLIS: u64 = 500;

#[derive(Debug, Serialize)]
struct Cpu {
    user: f32,
    system: f32,
    idle: f32,
    temp: Option<f32>,
}

#[derive(Debug, Serialize)]
struct Swap {
    used: u64,
    total: u64,
}

#[derive(Debug, Serialize)]
struct Mem {
    used: u64,
    total: u64,
}

#[derive(Debug, Serialize)]
struct Memory {
    swap: Option<Swap>,
    mem: Option<Mem>,
}

#[derive(Debug, Serialize)]
struct SystemInfo {
    cpu: Option<Cpu>,
    memory: Memory,
    uptime: Option<u64>,
}

fn main() {
    let system = System::new();

    let system_info = SystemInfo {
        cpu: get_cpu(&system),
        memory: Memory {
            swap: get_swap(&system),
            mem: get_mem(&system),
        },
        uptime: get_uptime(&system),
    };

    println!("{system_info:?}");
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
            thread::sleep(Duration::from_millis(INTERVAL_MILLIS));
            match cpu.done() {
                Ok(cpu_load) => Some(Cpu {
                    user: cpu_load.user * 100.0,
                    system: cpu_load.system * 100.0,
                    idle: cpu_load.idle * 100.0,
                    temp: get_cpu_temp(&system),
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
