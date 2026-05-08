use rusqlite::{Connection, Result};

pub fn create_connection() -> Result<()> {
    let connection_to_db = Connection::open("memorydle.db")?;

    connection_to_db.execute(
        "CREATE TABLE IF NOT EXISTS countries (
                country_id integer primary key,
                country_name text not null unique
        )", ()
    )?;

    Ok(())
}