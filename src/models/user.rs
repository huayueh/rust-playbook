use crate::models::schema::users;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListUserRequest {
    pub id_start: i64,
    pub id_end: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserList {
    pub page: i64,
    pub page_size: i64,
    pub posts: Vec<User>,
}
