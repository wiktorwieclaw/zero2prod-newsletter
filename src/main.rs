use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration, telemetry};
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to create Postgres connection pool.");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    zero2prod::run(listener, connection_pool)?.await?;
    Ok(())
}
