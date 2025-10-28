use rusqlite::Connection;

pub async fn create_database(sqlite: &Connection) -> rusqlite::Result<()> {
    // Create memory table
    //  "{{\"total_memory\": {},\"used_memory\": {},\"free_memory\": {}}}",
    sqlite.execute(
        "CREATE TABLE IF NOT EXISTS memory (id INTEGER PRIMARY KEY, total_memory INTEGER NOT NULL, used_memory INTEGER NOT NULL, free_memory INTEGER NOT NULL)",
        (),
    )?;
    // Create cpu table
    // "{{\"total_cpus\": {},\"total_cpu_usage\": {},\"cores_usage\": {:?}}}",
    sqlite.execute(
        "CREATE TABLE IF NOT EXISTS cpu (id INTEGER PRIMARY KEY, total_cpus INTEGER NOT NULL, total_cpu_usage INTEGER NOT NULL, cores_usage TEXT NOT NULL, create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
        (),
    )?;
    Ok(())
}
