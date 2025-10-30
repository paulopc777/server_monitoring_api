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

    jobs::clear_database::clear_database(connection.clone());
    jobs::make_request::make_request(connection.clone());

    server::http::start_http_server(connection).await?;

    Ok(())
}
