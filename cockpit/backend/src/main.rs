use settings::settings;
use std::error::Error;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

mod frontend;
mod gamepad;
mod linkage;
mod settings;

fn main() -> Result<(), Box<dyn Error>> {
    let settings = settings()?;

    let mut runtime_stream =
        TcpStream::connect(settings.runtime().to_string()).expect("should connect to runtime");
    eprintln!("Connected to Runtime on address {}.", settings.runtime());

    let (frontend_tx, frontend_rx) = mpsc::channel();
    thread::spawn({
        let mut runtime_stream = runtime_stream.try_clone().unwrap();
        move || frontend::channel(&mut runtime_stream, frontend_rx)
    });
    thread::spawn(move || {
        frontend::handle_runtime_confirmations(&mut runtime_stream, settings.linkage())
    });

    frontend::listen(frontend_tx);

    Ok(())
}
