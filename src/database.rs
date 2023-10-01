use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
// use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

// ======

pub fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}