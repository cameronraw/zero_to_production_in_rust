use zero_to_production::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("172.0.0.1:8000");
    match listener {
        Ok(valid_listener) => run(valid_listener)?.await,
        Err(err) => panic!("{}", err)
    }
}
