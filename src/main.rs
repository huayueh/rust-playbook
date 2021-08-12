#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use crate::repositories::repositories;
use crate::routes::routes;
use crate::models::config::CONFIG;
use actix_web::{App, HttpServer};
use log4rs;

mod routes;
mod models;
mod repositories;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    HttpServer::new(||
        App::new()
            .configure(repositories)
            .configure(routes)
    )
        .shutdown_timeout(300)
        // .workers(32)
        .bind(&CONFIG.server)?
        .run()
        .await
}