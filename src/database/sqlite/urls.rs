use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub struct UrlData {
    pub id: i32,
    pub url: String,
    pub status_code: i32,
    pub created_at: String,
}

pub fn get_urls(sqlite: &Arc<std::sync::Mutex<rusqlite::Connection>>) -> Vec<UrlData> {
    let sqlite = sqlite.lock().unwrap();
    let mut stmt = sqlite.prepare("SELECT * FROM urls").unwrap();
    let url_iter = stmt
        .query_map([], |row| {
            Ok(UrlData {
                id: row.get(0).unwrap(),
                url: row.get(1).unwrap(),
                status_code: row.get(2).unwrap_or(0),
                created_at: row.get(3).unwrap(),
            })
        })
        .unwrap();

    let mut urls = Vec::new();
    for url in url_iter {
        urls.push(url.unwrap());
    }
    urls
}

pub fn create_url(
    sqlite: Arc<Mutex<Connection>>,
    url: &str,
    status_code: Option<i32>,
) -> rusqlite::Result<()> {
    let sqlite = sqlite.lock().unwrap();
    sqlite.execute(
        "INSERT INTO urls (url, status_code) VALUES (?1, ?2)",
        (url, status_code),
    )?;
    Ok(())
}

pub fn update_url_status(sqlite: &Connection, id: i32, status_code: i32) -> rusqlite::Result<()> {
    sqlite.execute(
        "UPDATE urls SET status_code = ?1 WHERE id = ?2",
        (status_code, id),
    )?;
    Ok(())
}

pub fn delete_url(sqlite: &Connection, id: i32) -> rusqlite::Result<()> {
    sqlite.execute("DELETE FROM urls WHERE id = ?1", (id,))?;
    Ok(())
}
