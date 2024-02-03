use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use routes::{health_check, subscribe};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub mod configuration;
pub mod routes;

pub fn run(listener: TcpListener, connection: PgPool) -> std::io::Result<Server> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
