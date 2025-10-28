mod database;
mod jobs;
mod server;
mod services;

use std::sync::{Arc, Mutex};

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let connection: rusqlite::Connection =
        database::sqlite::connection::connection_database().await?;
    database::sqlite::create_database::create_database(&connection).await?;
    let connection = Arc::new(Mutex::new(connection));

    database::sqlite::flush::flush_cpu_info(connection.clone()).await?;

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

    server::http::start_http_server(connection).await?;

    Ok(())
}
