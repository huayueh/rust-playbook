use crate::models::errors::ApiError;
use crate::models::post::{Post, UpdatePost, ListPostRequest, PostList};

pub trait PostRepository: Send + Sync {
    fn create_post(&self, post: Post) -> Result<String, ApiError>;
    fn find_by_id(&self, post_id: i64) -> Result<Post, ApiError>;
    fn update_post(&self, post: UpdatePost) -> Result<String, ApiError>;
    fn delete_post(&self, post_id: i64) -> Result<String, ApiError>;

    fn read_post(&self, post_id: i64) -> Result<Post, ApiError>;
    fn list_post(&self, req: ListPostRequest) -> Result<PostList, ApiError>;
}
