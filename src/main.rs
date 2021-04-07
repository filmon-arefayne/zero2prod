use std::net::TcpListener;
use env_logger::Env;
use sqlx::PgPool;
use zero2prod::configuration;
use zero2prod::startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    startup::run(listener, connection_pool)?.await
}
