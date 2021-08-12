use crate::models::schema::posts;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, QueryableByName, Identifiable, Insertable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub is_read: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, QueryableByName)]
pub struct PostWithRow {
    #[sql_type = "diesel::sql_types::BigInt"]
    pub row_num: i64,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub count: i64,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub id: i64,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub user_id: i64,
    #[sql_type = "diesel::sql_types::Text"]
    pub title: String,
    #[sql_type = "diesel::sql_types::Text"]
    pub content: String,
    #[sql_type = "diesel::sql_types::Bool"]
    pub is_read: bool,
    #[sql_type = "diesel::sql_types::Timestamp"]
    pub created_at: NaiveDateTime,
    #[sql_type = "diesel::sql_types::Timestamp"]
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewPost {
    pub user_id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatePost {
    pub id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct ListPostRequest {
    pub id_start: i64,
    pub id_end: i64,
    pub page: i64,
    #[validate(range(min = 10, max = 100))]
    pub page_size: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostList {
    // pub count: i64,
    pub page: i64,
    pub page_size: i64,
    pub is_end: bool,
    pub posts: Vec<Post>,
}

impl From<NewPost> for Post {
    fn from(post: NewPost) -> Self {
        Post {
            id: 0,
            user_id: post.user_id,
            title: post.title,
            content: post.content,
            is_read: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
