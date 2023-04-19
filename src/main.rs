use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero_to_production::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use zero_to_production::configuration::get_configuration;
use zero_to_production::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let subscriber = get_subscriber(
        "zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

