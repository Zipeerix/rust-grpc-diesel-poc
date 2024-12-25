use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use tonic::Status;

pub trait Validatable {
    fn validate(&self) -> Result<(), String>;
}

pub type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub type PooledDbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct GrpcDbPoolWrapper {
    pub pool: DbPool,
}

impl GrpcDbPoolWrapper {
    pub fn new(pool: DbPool) -> GrpcDbPoolWrapper {
        GrpcDbPoolWrapper { pool }
    }

    pub fn get_db_connection_or_return_unavailable(&self) -> Result<PooledDbConnection, Status> {
        self.pool
            .get()
            .map_err(|e| Status::unavailable(format!("Database error: {}", e)))
    }
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn create_db_pool(database_url: &str, timeout: u64) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    diesel::r2d2::Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(timeout))
        .build(manager)
        .expect("Failed to create database connection pool")
}

pub fn connect_to_database_and_run_migrations(timeout: u64) -> DbPool {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set as an environment variable");

    let pool = create_db_pool(&database_url, timeout);
    pool.get()
        .expect("Unable to get connection from pool")
        .run_pending_migrations(MIGRATIONS)
        .expect("Unable to run pending migrations");

    pool
}
