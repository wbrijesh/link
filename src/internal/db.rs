use crate::utils::env_var::load_env;
use crate::DbPool;
use diesel::{prelude::*, r2d2};

pub fn initialize_db_pool() -> DbPool {
    let conn_spec = load_env("DATABASE_URL").unwrap();
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);

    return r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");
}
