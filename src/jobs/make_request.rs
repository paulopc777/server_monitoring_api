use crate::database;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub fn make_request(connection: Arc<Mutex<Connection>>) {
    let request_connection: Arc<Mutex<Connection>> = connection.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            let urls = database::sqlite::urls::get_urls(&request_connection);

            for url_data in urls {
                let url = url_data.url;
                let id = url_data.id;
                println!("make request {}", url);
                let response = reqwest::get(&url).await;

                match response {
                    Ok(resp) => {
                        let status_code = resp.status().as_u16() as i32;
                        println!("URL: {}, Status Code: {}", url, status_code);
                        let conn = connection.lock().unwrap();
                        database::sqlite::urls::update_url_status(&conn, id, status_code).unwrap();
                    }
                    Err(err) => {
                        eprintln!("Error making request to {}: {}", url, err);
                        let conn = connection.lock().unwrap();
                        database::sqlite::urls::update_url_status(&conn, id, 404).unwrap();
                    }
                }
            }
        }
    });
}
