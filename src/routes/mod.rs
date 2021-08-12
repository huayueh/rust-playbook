use actix_web::web;
use crate::routes::hello_route::echo_routes;
use crate::routes::post_route::post_routes;
use crate::routes::user_route::user_routes;

mod hello_route;
mod user_route;
mod post_route;

pub fn routes(cfg: &mut web::ServiceConfig) {
    echo_routes(cfg);
    post_routes(cfg);
    user_routes(cfg);
}