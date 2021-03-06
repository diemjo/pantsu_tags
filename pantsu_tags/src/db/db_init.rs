use std::path::Path;
use rusqlite::{Connection, OpenFlags};
use crate::common::error::{Error, get_path};
use crate::db::sqlite_statements;

pub fn open(db_path: &Path) -> Result<Connection, Error> {
    let pantsu_db_updates: Vec<&dyn Fn(&mut Connection) -> Result<(), Error>> = vec![
        &db_init_1,
        //eg: &db_update_1_2,
    ];
    let  pantsu_db_version = pantsu_db_updates.len();

    let mut conn = match Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_WRITE) {
        Ok(conn) => conn,
        Err(_) => {
            eprintln!("Database {} does not exist, creating new...", get_path(db_path));
            let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE)?;
            conn.pragma_update(None, "foreign_key", "ON")?;
            conn.pragma_update(None, "user_version", 0)?;
            conn
        }
    };
    let old_db_version = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
    if pantsu_db_version < old_db_version {
        return Err(Error::ProgramOutdated(format!("Expected database version <={} but found version {}", pantsu_db_version, old_db_version)));
    } else if pantsu_db_version > old_db_version {
        for i in old_db_version..pantsu_db_version {
            pantsu_db_updates[i](&mut conn)?;
            conn.pragma_update(None, "user_version", i+1)?;
        }
    } else {
        //println!("opened database with version {}", pantsu_db_version);
    }
    Ok(conn)
}

fn db_init_1(connection: &mut Connection) -> Result<(), Error> {
    println!("Initializing database version 1");
    connection.execute_batch(sqlite_statements::DB_INIT_TABLES)?;
    Ok(())
}