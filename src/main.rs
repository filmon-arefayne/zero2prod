use std::net::TcpListener;

use zero2prod::startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");
    startup::run(address)?.await
}
