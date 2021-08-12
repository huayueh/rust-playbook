use crate::repositories::post_repository::PostRepository;
use crate::repositories::user_repository::UserRepository;
use crate::repositories::mysql::mysql_post_repository::MysqlPostRepository;
use crate::repositories::mysql::mysql_user_repository::MysqlUserRepository;
use crate::models::config::CONFIG;
use crate::models::config::Database;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;
use actix_web::web;
use log::info;

pub mod user_repository;
pub mod post_repository;

mod mongodb;
mod mysql;

lazy_static! {
    static ref POOL: Pool<ConnectionManager<MysqlConnection>> = new_pool();
}

pub fn repositories(cfg: &mut web::ServiceConfig) {
    match &CONFIG.database {
        Database::Mysql => config_mysql(cfg),
        Database::MongoDB => panic!("Not support database: MongoDB"),
    }
}

fn new_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let url = format!("mysql://{}:{}@{}:{}/{}",
                      &CONFIG.database_account, &CONFIG.database_password,
                      &CONFIG.database_host, &CONFIG.database_port, &CONFIG.database_db);
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = Pool::new(manager)
        .expect("MySQL connection pool");
    info!("MySQL connection pool created host: {}:{} database: {}",
        &CONFIG.database_host, &CONFIG.database_port, &CONFIG.database_db);
    pool
}

fn config_mysql(cfg: &mut web::ServiceConfig) {
    let post_rep: Box<dyn PostRepository> = Box::new(MysqlPostRepository::new(&POOL));
    let user_rep: Box<dyn UserRepository> = Box::new(MysqlUserRepository::new(&POOL));
    cfg.data(post_rep);
    cfg.data(user_rep);
}
