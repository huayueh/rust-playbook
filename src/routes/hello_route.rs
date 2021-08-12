use actix_web::{get, web, Responder};
use actix_web::web::{Path, Json, block};
use crate::models::errors::ApiError;
use crate::models::post::Post;
use chrono::Utc;
use log::info;
use actix_web::error::BlockingError;

pub fn echo_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api/v1/echo")
                .service(hello)
                .service(echo_post)
        );
}

#[get("/{id}/{name}")]
async fn hello(Path((id, name)): Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/post")]
async fn echo_post() -> Result<Json<Post>, BlockingError<ApiError>> {
    info!("hey worker thread");
    let post = block(move ||{
        info!("hey thread-pool");
        Ok(Post {
            id: 0,
            user_id: 0,
            title: "".to_string(),
            content: "".to_string(),
            is_read: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
    }).await?;
    Ok(Json(post))
}