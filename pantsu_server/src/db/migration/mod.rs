use rocket::{Build, fairing, Rocket};
use rocket_db_pools::Database;
use rocket_db_pools;
use rocket_db_pools::deadpool_postgres::Transaction;
use tracing::{debug, error};

use crate::common::error::Error;
use crate::common::result::Result;
use crate::db::PantsuDB;

pub async fn migrate(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = PantsuDB::fetch(&rocket) {
        debug!("checking database version");
        match run_migrations(db).await {
            Ok(()) => Ok(rocket),
            Err(e) => {
                error!("error: database migrations failed: {:?}", e);
                Err(rocket)
            }
        }
    } else {
        Err(rocket)
    }
}

async fn run_migrations<'c>(db: &PantsuDB) -> Result<()> {
    let migrations: Vec<&str> = vec![
        include_str!("v1.0.0__db_init.sql"),
    ];

    let mut client = db.get().await?;
    /* let local_version = db.fetch_one(sqlx::query("PRAGMA user_version"))
        .await?
        .get::<i32, usize>(0) as usize; */
    let local_version = 0;
    let current_version = migrations.len();

    if current_version < local_version {
        return Err(Error::ProgramOutdatedError(local_version, current_version))
    } else if current_version > local_version {
        let transaction = client.transaction().await?;
        debug!("updating database...");
        for (index, migration) in migrations.iter().enumerate().skip(local_version) {
            debug!("running database update to version {}", index + 1);
            run_migration(&transaction, migration).await?;
            /*sqlx::query(format!("PRAGMA user_version={}", index + 1).as_str())
                .execute(&mut transaction)
                .await?; */
        }
        transaction.commit().await?;
        debug!("successfully updated database!");
    } else {
        debug!("database is up to date with version '{}'", current_version);
    }

    Ok(())
}

async fn run_migration<'c>(transaction: &Transaction<'c>, migration: &str) -> Result<()> {
    let result = transaction.batch_execute(migration).await;
    if let Err(error) = result {
        println!("error: {}", error);
        return Err(Error::DbMigrationError(error));
    }
    Ok(())
}
