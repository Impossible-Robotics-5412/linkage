use common::config;

use std::io;
use std::process::{Child, Command, Stdio};

use log::info;
use simple_signal::Signal;

pub(crate) fn handle_alrm_signal(sender: crossbeam::channel::Sender<()>) {
    info!("Start listening for ALRM Signal");

    simple_signal::set_handler(&[Signal::Alrm], {
        move |_signals| {
            info!("Caught ALRM signal");
            sender
                .send(())
                .expect("should send ALRM signal over channel");
        }
    });
}

pub(crate) fn start_processes(config: &config::Runtime) -> Vec<Child> {
    let carburetor_process = Command::new(config.carburetor_path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("should execute carburetor");

    // FIXME: Add error message when Node is not found.
    let linkage_process = Command::new(config.node_path())
        .current_dir("/")
        .arg(config.linkage_lib_entry_point())
        .spawn()
        .expect("should execute linkage");

    vec![carburetor_process, linkage_process]
}

pub(crate) fn stop_processes(children: &mut Vec<Child>) -> io::Result<()> {
    for child in children {
        child.kill()?;
        child.wait()?;
    }

    Ok(())
}
