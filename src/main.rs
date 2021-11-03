use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    // the `?` operator bubbles up the error so it does not need explicit handling
    run(listener)?.await
}
