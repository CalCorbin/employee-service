// Error handling
use crate::error_handler::CustomError;
// Postgres Connection!
use diesel::pg:PgConnection;
// Manage our DB connection
use diesel::r2d2::ConnectionManager;
// Initializes our database at runtime
use lazy_static::lazy_static;
use r2d2;
use std::env;
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
embed_migrations!();
lazy_static! {
    static ref POOL: Pool = {
        // The DB url!
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}
pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    embedded_migrations::run(&conn).unwrap();
}
pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}
