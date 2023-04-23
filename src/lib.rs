use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;

mod routes;
mod startup;
pub mod configuration;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
    })
        .listen(listener)?
        .run();
    Ok(server)
}