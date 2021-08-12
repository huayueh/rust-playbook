use crate::models::errors::ApiError;
use crate::models::user::{User, ListUserRequest, UserList};
use crate::repositories::user_repository::UserRepository;
use actix_web::{get, web};
use actix_web::web::{Json, Path, Data, block, Query};


pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api/v1/users")
                .service(by_id)
                .service(list)
        );
}

#[get("")]
async fn list(
    req: Query<ListUserRequest>,
    repository: Data<Box<dyn UserRepository>>,
) -> Result<Json<UserList>, ApiError> {
    let posts = block(move ||
        repository.list_post(req.clone())).await?;
    Ok(Json(posts))
}

#[get("/{id}")]
async fn by_id(
    id: Path<i64>,
    repository: Data<Box<dyn UserRepository>>,
) -> Result<Json<User>, ApiError> {
    let user = block(move ||
        repository.find_by_id(*id)).await?;
    Ok(Json(user))
}
