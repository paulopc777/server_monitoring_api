use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub async fn save_cpu_info(
    con: Arc<Mutex<Connection>>,
    total_cpus: u32,
    total_cpu_usage: u32,
    cores_usage: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let con = con.lock().unwrap();
    let query: &'static str =
        "INSERT INTO cpu (total_cpus, total_cpu_usage, cores_usage) VALUES (?1, ?2, ?3)";
    con.execute(query, (&total_cpus, &total_cpu_usage, &cores_usage))?;
    Ok(())
}

pub async fn get_cpu_history(
    con: Arc<Mutex<Connection>>,
) -> Result<Vec<(u32, u32, u32, String, String)>, Box<dyn std::error::Error>> {
    let con = con.lock().unwrap();
    let mut stmt = con.prepare(
        "SELECT id, total_cpus, total_cpu_usage, cores_usage, create_at FROM cpu ORDER BY create_at DESC LIMIT 30",
    )?;
    let cpu_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ))
    })?;

    let mut cpu_info = Vec::new();
    for cpu in cpu_iter {
        cpu_info.push(cpu?);
    }
    Ok(cpu_info)
}
