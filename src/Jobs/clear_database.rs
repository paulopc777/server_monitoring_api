use crate::database;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub fn clear_database(connection: Arc<Mutex<Connection>>) {
    // set interval to 5 minutes flush cpu info
    let flush_connection = connection.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));
        loop {
            interval.tick().await;
            if let Err(e) = database::sqlite::flush::flush_cpu_info(flush_connection.clone()).await
            {
                eprintln!("Error flushing CPU info: {}", e);
            }
        }
    });
}
