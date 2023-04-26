use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port.");
    run(listener)?.await
}
