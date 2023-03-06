use crate::common_lib::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER not found.");
        let db_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not found.");
        let db_host = env::var("DB_HOST").expect("DB_HOST not found.");
        let db_port = env::var("DB_PORT").expect("DB_PORT not found.");
        let db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB not found.");

        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            &db_user, &db_password, &db_host, &db_port, &db_name
        );

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}
