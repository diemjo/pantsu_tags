use rocket_db_pools::Database;

pub use migration::migrate;

mod migration;

#[derive(Database)]
#[database("pantsu_db")]
pub struct PantsuDB(pub rocket_db_pools::deadpool_postgres::Pool);
