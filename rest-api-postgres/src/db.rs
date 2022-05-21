use super::errors::ServiceError;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use std::env::var;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = var("DATABASE_URL").expect("DATABASE_URL not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::builder()
            .build(manager)
            .expect("Failed to create database pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
}

pub fn connection() -> Result<DbConnection, ServiceError> {
    POOL.get().map_err(|e| {
        ServiceError::new(
            500,
            format!("Failed to establish database connection: {}", e),
        )
    })
}
