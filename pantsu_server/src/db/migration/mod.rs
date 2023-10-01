use lz_fnv::{Fnv1a, FnvHasher};
use rocket::{Build, fairing, Rocket};
use rocket_db_pools;
use rocket_db_pools::Database;
use rocket_db_pools::deadpool_postgres::{Client, GenericClient, tokio_postgres, Transaction};
use rocket_db_pools::deadpool_postgres::tokio_postgres::Row;
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
                error!("{}", e);
                Err(rocket)
            }
        }
    } else {
        Err(rocket)
    }
}

async fn run_migrations(db: &PantsuDB) -> Result<()> {
    let migrations: Vec<Migration> = vec![
        ("v1.0.0", include_str!("migrations/v1.0.0__db_init.sql")),
    ].into_iter()
        .map(|(version, sql)|
            Migration {
                version: version.to_owned(),
                hash: calculate_migration_hash(sql),
                sql: sql.to_owned(),
            }
        )
        .collect();

    let mut client: Client = db.get().await?;
    init_migration_schema(&client).await?;
    let applied_migrations = get_migrations(&client).await?;
    for (migration, applied_migration) in migrations.iter().zip(applied_migrations.iter()) {
        verify_migration(migration, applied_migration)?;
    }
    if migrations.len() < applied_migrations.len() {
        return Err(Error::ProgramOutdatedError(
            applied_migrations.last().map(|m| m.version.as_str()).unwrap_or("<none>").to_owned(),
            migrations.last().map(|m| m.version.as_str()).unwrap_or("<none>").to_owned()
        ))
    }
    else if migrations.len() > applied_migrations.len() {
        let transaction = client.transaction().await?;
        debug!("updating database...");
        for migration in migrations.into_iter().skip(applied_migrations.len()) {
            debug!("running database update to version '{}'", migration.version);
            if let Err(e) = run_migration(&transaction, migration).await {
                return Err(Error::DbMigrationError(e));
            }
        }
        transaction.commit().await?;
        debug!("successfully updated database!");
    } else {
        debug!("database is up to date with version '{}'", migrations.last().map(|m| m.version.as_str()).unwrap_or("<none>"));
    }

    Ok(())
}

fn calculate_migration_hash(migration: &str) -> String {
    let mut hasher = Fnv1a::<u64>::new();
    hasher.write(migration.as_bytes());
    format!("{:016x}", hasher.finish())
}

fn verify_migration(migration: &Migration, applied_migration: &Migration) -> Result<()> {
    if migration.version != applied_migration.version {
        return Err(Error::DbMigrationVersionMissing(migration.version.to_owned()));
    }
    if applied_migration.hash != migration.hash {
        return Err(Error::DbMigrationHashMismatch(migration.version.to_owned(), applied_migration.hash.to_owned(), migration.hash.to_owned()))
    }
    Ok(())
}

async fn init_migration_schema(client: &Client) -> Result<()> {
    client.batch_execute(include_str!("sql/init_migration_schema.sql")).await?;
    Ok(())
}

async fn get_migrations(client: &Client) -> Result<Vec<Migration>> {
    let result = client.query(include_str!("sql/get_migrations.sql"), &[]).await?;
    let migrations = result.into_iter()
        .map(|row| Migration::try_from(row))
        .collect::<Result<Vec<Migration>>>()?;
    // debug!("migrations: {:?}", migrations);
    Ok(migrations)
}

async fn run_migration<'a>(transaction: &Transaction<'a>, migration: Migration) -> std::result::Result<(), tokio_postgres::Error> {
    transaction.batch_execute(migration.sql.as_str()).await?;
    transaction.execute(
        include_str!("sql/insert_migration.sql"),
        &[&migration.version, &migration.hash, &migration.sql],
    ).await?;
    Ok(())
}

#[derive(Debug)]
struct Migration {
    pub version: String,
    pub hash: String,
    pub sql: String,
}

impl TryFrom<Row> for Migration {
    type Error = Error;

    fn try_from(row: Row) -> std::result::Result<Self, Self::Error> {
        Ok(Migration {
            version: row.try_get(0)?,
            hash: row.try_get(1)?,
            sql: row.try_get(2)?,
        })
    }
}
