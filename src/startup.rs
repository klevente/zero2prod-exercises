use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // wrap the pool in an Arc smart pointer, which is `Clone`, as it is required
    // as actix spins up multiple servers using the closure supplied below, so it needs to clone state variables
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    // capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            // `TracingLogger` takes care of generating request IDs for all requests automatically
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // get a pointer copy and attach it to the app state
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
