mod server;

const ADDR: &str = ":3000";
fn main() {
    server::server(ADDR);
}
