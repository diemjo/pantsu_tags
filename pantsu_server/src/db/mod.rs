// use rocket_db_pools::Database;
//
// pub use migration::migrate;
//
// mod migration;
//
// #[derive(Database)]
// #[database("pantsu_db")]
// pub struct PantsuDB(pub rocket_db_pools::deadpool_postgres::Pool);
//
// pub fn config() -> rocket_db_pools::Config {
//     rocket_db_pools::Config {
//         url: "postgres://postgres:postgres@localhost/pantsudb".to_string(),
//         min_connections: None,
//         max_connections: 16,
//         connect_timeout: 5,
//         idle_timeout: None,
//     }
// }
