use actix_web::{dev::Server, web, App, HttpServer};
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;
use sqlx::PgPool;
use crate::routes;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(web::Data::clone(&db_pool))
    })
    .listen(listener)?
    .run();
    Ok(server)
}