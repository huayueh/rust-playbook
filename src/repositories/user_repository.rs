use crate::models::errors::ApiError;
use crate::models::user::{User, ListUserRequest, UserList};

pub trait UserRepository: Send + Sync {
    fn find_by_id(&self, user_id: i64) -> Result<User, ApiError>;
    fn list_post(&self, req: ListUserRequest) -> Result<UserList, ApiError>;
}

