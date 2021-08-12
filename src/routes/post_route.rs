use crate::models::errors::ApiError;
use actix_web::{get, post, delete, put, web};
use actix_web::web::{Json, Path, Data, Query, block};
use crate::models::post::{Post, NewPost, UpdatePost, ListPostRequest, PostList};
use crate::repositories::post_repository::PostRepository;

pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api/v1/posts")
                .service(create_post)
                .service(find_by_id)
                .service(update_by_id)
                .service(delete_by_id)
                .service(read_by_id)
                .service(list)
        );
}

#[post("")]
async fn create_post(
    post: Json<NewPost>,
    repository: Data<Box<dyn PostRepository>>,
) -> Result<String, ApiError> {
    let new_post: Post = NewPost {
        user_id: post.user_id,
        title: post.title.to_string(),
        content: post.content.to_string(),
    }.into();
    let ret = block(move ||
        repository.create_post(new_post)).await?;
    Ok(ret)
}

#[put("")]
async fn update_by_id(
    post: Json<UpdatePost>,
    repository: Data<Box<dyn PostRepository>>,
) -> Result<String, ApiError> {
    let update_post = UpdatePost {
        id: post.id,
        title: post.title.to_string(),
        content: post.content.to_string(),
    };
    let ret = block(move ||
        repository.update_post(update_post)).await?;
    Ok(ret)
}

#[get("/{id}")]
async fn find_by_id(
    id: Path<i64>,
    repository: Data<Box<dyn PostRepository>>,
) -> Result<Json<Post>, ApiError> {
    let post = block(move ||
        repository.find_by_id(*id)).await?;
    Ok(Json(post))
}

#[post("/read/{id}")]
async fn read_by_id(
    id: Path<i64>,
    repository: Data<Box<dyn PostRepository>>,
) -> Result<Json<Post>, ApiError> {
    let post = block(move ||
        repository.read_post(*id)).await?;
    Ok(Json(post))
}

#[delete("/{id}")]
async fn delete_by_id(
    id: Path<i64>,
    repository: Data<Box<dyn PostRepository>>,
) -> Result<String, ApiError> {
    let ret = block(move ||
        repository.delete_post(*id)).await?;
    Ok(ret)
}

#[get("")]
async fn list(
    req: Query<ListPostRequest>,
    repository: Data<Box<dyn PostRepository>>,
) -> Result<Json<PostList>, ApiError> {
    let posts = block(move || repository.list_post(req.clone())).await?;
    Ok(Json(posts))
}