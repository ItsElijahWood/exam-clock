mod server;
mod modules;

const ADDR: &str = ":3000";
fn main() {
    modules::read_centre_file::read_centre_file();

    server::server(ADDR);
}
