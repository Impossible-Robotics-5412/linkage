use std::{
    io,
    process::{Child, Command, Stdio},
};

use common::config;
use simple_signal::Signal;

pub(crate) fn handle_alrm_signal(sender: crossbeam::channel::Sender<()>) {
    eprintln!("Start listening for ALRM Signal");

    simple_signal::set_handler(&[Signal::Alrm], {
        move |_signals| {
            eprintln!("Caught ALRM signal");
            sender
                .send(())
                .expect("failed to send ALRM signal over channel");
        }
    });
}

pub(crate) fn start_processes(config: &config::Runtime) -> Vec<Child> {
    let carburetor_process = Command::new(config.carburetor_path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute carburetor");

    let linkage_process = Command::new(config.node_path())
        .current_dir("/")
        .arg(config.linkage_lib_entry_point())
        .spawn()
        .expect("failed to execute linkage");

    vec![carburetor_process, linkage_process]
}

pub(crate) fn stop_processes(children: &mut Vec<Child>) -> io::Result<()> {
    for child in children {
        child.kill()?;
        child.wait()?;
    }

    Ok(())
}
