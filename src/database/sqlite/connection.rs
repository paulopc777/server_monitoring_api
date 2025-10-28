use rusqlite::Connection;

pub async fn connection_database() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("my_database.db")?;
    Ok(conn)
}
