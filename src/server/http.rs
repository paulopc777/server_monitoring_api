use std::{
    env,
    sync::{Arc, Mutex},
};

use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use rusqlite::Connection;
use std::net::SocketAddr;
use sysinfo::System;
use tokio::net::TcpListener;

use crate::server::request::received_request;

async fn create_host_addr() -> Result<TcpListener, Box<dyn std::error::Error>> {
    let host: [u8; 4] = env::var("HOST")
        .unwrap_or("127.0.0.1".into())
        .split(".")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();

    println!("Binding to host: {:?}", host);
    let addr = SocketAddr::from((host, 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    return Ok(listener);
}

pub async fn start_http_server(
    connection: Arc<Mutex<Connection>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = create_host_addr().await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let sys: System = System::new_all();
        let connection_thread = Arc::clone(&connection);

        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            let builder = http1::Builder::new();

            if let Err(err) = builder
                .serve_connection(
                    io,
                    service_fn(|req| received_request(req, &sys, connection_thread.clone())),
                )
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
