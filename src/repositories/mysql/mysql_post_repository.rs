use crate::models::errors::ApiError;
use crate::models::post::{Post, UpdatePost, ListPostRequest, PostList};
use crate::models::schema::posts::dsl::*;
use crate::repositories::post_repository::PostRepository;

use diesel::{mysql::MysqlConnection, r2d2::ConnectionManager, insert_into, delete, update};
use diesel::prelude::*;
use log::{info, error};
use chrono::Utc;
use diesel::r2d2::Pool;

pub struct MysqlPostRepository {
    pub pool: &'static Pool<ConnectionManager<MysqlConnection>>,
}

impl MysqlPostRepository {
    pub fn new(p: &'static Pool<ConnectionManager<MysqlConnection>>) -> Self {
        Self {
            pool: p
        }
    }
}

impl PostRepository for MysqlPostRepository {
    fn create_post(&self, post: Post) -> Result<String, ApiError> {
        let conn = self.pool.get()?;
        insert_into(posts)
            .values(&post)
            .execute(&conn)?;
        let msg = format!("post {} created", post.title);
        info!("{}", &msg);
        Ok(msg)
    }

    fn find_by_id(&self, post_id: i64) -> Result<Post, ApiError> {
        let conn = self.pool.get()?;
        let post = posts
            .find(post_id)
            .first::<Post>(&conn)
            .map_err(|err| {
                error!("{}", err);
                ApiError::NotFound(format!("User {} not found", post_id))
            })?;
        info!("post {} found", &post.title);
        Ok(post)
    }

    fn update_post(&self, post: UpdatePost) -> Result<String, ApiError> {
        let conn = self.pool.get()?;
        update(posts.find(post.id))
            .set((title.eq(post.title),
                  content.eq(post.content),
                  updated_at.eq(Utc::now().naive_utc())))
            .execute(&conn)?;
        let msg = format!("post id:{} updated", post.id);
        info!("{}", &msg);
        Ok(msg)
    }

    fn delete_post(&self, post_id: i64) -> Result<String, ApiError> {
        let conn = self.pool.get()?;
        delete(posts.find(post_id))
            .execute(&conn)?;
        let msg = format!("post {} deleted", post_id);
        info!("{}", &msg);
        Ok(msg)
    }

    fn read_post(&self, post_id: i64) -> Result<Post, ApiError> {
        let conn = self.pool.get()?;
        update(posts.find(post_id))
            .set((is_read.eq(true),
                  updated_at.eq(Utc::now().naive_utc())))
            .execute(&conn)?;
        let msg = format!("post id:{} read", post_id);
        info!("{}", &msg);
        self.find_by_id(post_id)
    }

    fn list_post(&self, req: ListPostRequest) -> Result<PostList, ApiError> {
        let conn = self.pool.get()?;
        let list = posts
            .filter(id.ge(req.id_start))
            .filter(id.lt(req.id_end))
            .limit(req.page_size)
            .offset((req.page-1) * req.page_size)
            .load::<Post>(&conn)?;

        // let list = sql_query(include_str!("summary.sql"))
        //     // .bind::<Integer, _>(req.id_start)
        //     .load::<PostWithRow>(&conn)?;
        // let ret_list = list.into_iter().map(|p|{
        //     Post{
        //         id: p.id,
        //         user_id: 1,
        //         title: p.title,
        //         content: p.content,
        //         is_read: p.is_read,
        //         created_at: p.created_at,
        //         updated_at: p.updated_at
        //     }
        // }).collect();
        info!("id_start {} id_end {}", &req.id_start, &req.id_end);
        Ok(PostList {
            // count: 0, //TODO: make custom struct work with diesel
            page: req.page,
            page_size: req.page_size,
            posts: list
        })
    }
}
