use diesel::prelude::*;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use std::error::Error;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub name: String,
    pub surname: String,
    pub country: String,
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn connect_to_database_and_run_migrations() -> PgConnection {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set as an environment variable");

    let mut conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    run_migrations(&mut conn).expect("Error running migrations");

    conn
}

fn run_migrations(
    connection: &mut PgConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // HarnessWithOutput todo
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
