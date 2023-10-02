use lz_fnv::{Fnv1a, FnvHasher};

pub fn calculate_migration_hash(migration: &str) -> String {
    let mut hasher = Fnv1a::<u64>::new();
    hasher.write(migration.as_bytes());
    format!("{:016x}", hasher.finish())
}

macro_rules! migration {
    ($file:literal) => {
        {
            let sql = include_str!($file);
            let (version, description_sql) = ($file).rsplit('/').next().unwrap().split_once("__").unwrap();
            Migration {
                version: version.to_owned(),
                description: description_sql.strip_suffix(".sql").unwrap().to_owned(),
                hash: $crate::db::migration::migration_macro::calculate_migration_hash(sql),
                sql: sql.to_owned(),
            }
        }
    }
}
