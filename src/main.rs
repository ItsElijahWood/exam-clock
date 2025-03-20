use std::process::Command;

mod server;

const ADDR: &str = ":3000";
fn main() {
    server::server(ADDR);

    let winbrowser = "start msedge";

    Command::new(winbrowser)
        .spawn()
        .expect("Failed to start msedge in main.rs");
}
