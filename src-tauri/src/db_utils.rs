use std::fs::{File, remove_file};
use std::path::Path;
use rusqlite::{Connection, Result as SqlResult, params, Params};

static LOCK_FILE: &str = "database.lock";
static DB_FILE: &str = "my_database.db";

pub fn initialize() -> SqlResult<()> {
    if Path::new(LOCK_FILE).exists() {
        panic!("Another instance of the application is already running.");
    }

    // Create the lock file
    File::create(LOCK_FILE).expect("Failed to create lock file");

    setup_db()
}

pub fn execute<P>(sql: &str, params: P) -> Result<usize, rusqlite::Error> {
    let con = Connection::open(DB_FILE).expect("Couldn't open a connection to the database");
    con.execute(&sql, params![])
}

pub fn query_single_row<T, P, F>(sql: &str, params: P, f: F) -> Result<usize, rusqlite::Error>
    where
        P: Params,
        F: FnOnce(&rusqlite::Row<'_>) -> Result<usize, rusqlite::Error>,
{
    let con = Connection::open(DB_FILE)?;
    con.query_row(sql, params, f)
}



pub fn query_map<T, P, F>(sql: &str, params: P, f: F) -> Result<Vec<T>, rusqlite::Error>
    where
        P: Params,
        F: FnMut(&rusqlite::Row<'_>) -> Result<T, rusqlite::Error>,
    {
    let con = Connection::open(DB_FILE)?;
    let mut stmt = con.prepare(sql).unwrap();
    stmt.query_map(params, f).unwrap().collect()
}


pub fn close() {
    remove_file(LOCK_FILE).expect("Failed to remove lock file");
}

fn setup_db() -> SqlResult<()> {
    let create_tutee_table = "
        CREATE TABLE IF NOT EXISTS tutee (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
    ";
    let create_tutor_table = "
        CREATE TABLE IF NOT EXISTS tutor (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
    ";

    Ok(())
}
