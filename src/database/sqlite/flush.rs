use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub async fn flush_cpu_info(con: Arc<Mutex<Connection>>) -> Result<(), Box<dyn std::error::Error>> {
    let con = con.lock().unwrap();
    // Flush CPU info
    let query: &'static str = "DELETE FROM cpu WHERE create_at < datetime('now', '-5 minutes')";
    con.execute(query, ())?;
    Ok(())
}
