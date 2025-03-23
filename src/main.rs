use std::process::Command;

mod server;
mod modules;

const ADDR: &str = ":3000";
fn main() {
    modules::read_centre_file::read_centre_file();

    let url = "http://127.0.0.1:3000";

    #[cfg(target_os = "windows")]
    if let Err(e) = Command::new("start http://127.0.0.1:3000") 
        .arg(url)
        .spawn()
    {
        eprintln!("Error executing default web browser command: {}", e);
    }

    #[cfg(target_os = "linux")]
    if let Err(e) = Command::new("xdg-open") 
        .arg(url)
        .spawn()
    {
        eprintln!("Error executing default web browser command: {}", e);
    }

    server::server(ADDR);
}
