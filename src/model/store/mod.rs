mod error;

use std::time::Duration;

pub use self::error::{Error, Result};

use crate::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_millis(10000))
        .connect(&config().DATABASE_URL)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
