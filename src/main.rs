mod server;
mod modules;

const ADDR: &str = ":3000";
fn main() {
    let time = modules::get_system_time::get_system_time();
    println!("Starting time from: {time}");

    server::server(ADDR);
}
