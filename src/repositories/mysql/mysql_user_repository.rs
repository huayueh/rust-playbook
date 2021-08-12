use crate::models::errors::ApiError;
use crate::models::user::{User, ListUserRequest, UserList};
use crate::repositories::user_repository::UserRepository;
use crate::models::schema::users::dsl::*;
use diesel::{
    mysql::MysqlConnection,
    r2d2::ConnectionManager,
};
use diesel::prelude::*;
use log::error;
use r2d2::Pool;

pub struct MysqlUserRepository {
    pub pool: &'static Pool<ConnectionManager<MysqlConnection>>
}

impl MysqlUserRepository {
    pub fn new(p: &'static Pool<ConnectionManager<MysqlConnection>>) -> MysqlUserRepository {
        MysqlUserRepository {
            pool: p
        }
    }
}

impl UserRepository for MysqlUserRepository {
    fn find_by_id(&self, uid: i64) -> Result<User, ApiError> {
        let conn = self.pool.get()?;
        let user = users
            .find(uid)
            .first::<User>(&conn)
            .map_err(|err| {
                error!("{}", err);
                ApiError::NotFound(format!("User {} not found", uid))
            })?;
        Ok(user)
    }

    fn list_post(&self, req: ListUserRequest) -> Result<UserList, ApiError> {
        let conn = self.pool.get()?;
        let list = users
            .filter(id.ge(req.id_start))
            .filter(id.lt(req.id_end))
            .limit(req.page_size)
            .offset((req.page-1) * req.page_size)
            .load::<User>(&conn)?;

        Ok(UserList {
            page: req.page,
            page_size: req.page_size,
            posts: list
        })
    }
}
