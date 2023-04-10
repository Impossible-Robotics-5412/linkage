use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub carburetor_status: bool,
    pub gauge_status: bool,
    pub linkage_socket_status: bool,
}

pub fn service_is_active(service_name: &str) -> bool {
    let exit_status = Command::new("systemctl")
        .args(["is-active", "--quiet", service_name])
        .status()
        .expect("failed to check service status");

    exit_status.success()
}
