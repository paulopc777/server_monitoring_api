use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};

use http::{HeaderValue, Method, Request, Response, StatusCode};
use http_body_util::Full;
use hyper::body::Bytes;
use rusqlite::Connection;
use sysinfo::System;

use crate::{database, services};

fn request_options(
    allow_origin: HeaderValue,
    allow_methods: HeaderValue,
    allow_headers: HeaderValue,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut res = Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Full::new(Bytes::new()))
        .unwrap();
    {
        let headers = res.headers_mut();
        headers.insert("Access-Control-Allow-Origin", allow_origin.clone());
        headers.insert("Access-Control-Allow-Methods", allow_methods.clone());
        headers.insert("Access-Control-Allow-Headers", allow_headers.clone());
        // headers.insert("Access-Control-Allow-Credentials", HeaderValue::from_static("true"));
    }
    return Ok(res);
}

pub async fn received_request(
    request: Request<hyper::body::Incoming>,
    sys: &System,
    con: Arc<Mutex<Connection>>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let allow_origin = HeaderValue::from_static("*");
    let allow_methods = HeaderValue::from_static("GET, POST, OPTIONS");
    let allow_headers = HeaderValue::from_static("Content-Type, Authorization");

    if request.method() == Method::OPTIONS {
        return request_options(allow_origin, allow_methods, allow_headers);
    }

    if request.uri().path() == "/memory" {
        let memory_info = services::os::memory::print_memory_info(&sys);
        let response_data = format!(
            "{{\"total_memory\": {},\"used_memory\": {},\"free_memory\": {}}}",
            memory_info[0], memory_info[1], memory_info[2]
        );
        let mut res = Response::new(Bytes::from(response_data));
        res.headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        res.headers_mut()
            .insert("Access-Control-Allow-Origin", allow_origin.clone());
        return Ok(res.map(Full::new));
    }

    if request.uri().path() == "/cpu" {
        let cpu_info = services::os::cpu::print_cpu_info(&sys);
        database::sqlite::query::save_cpu_info(
            con,
            cpu_info.total_cpus,
            cpu_info.total_cpu_usage,
            cpu_info.cores_usage.clone(),
        )
        .await
        .unwrap();
        let cpu_data = format!(
            "{{\"total_cpus\": {},\"total_cpu_usage\": {},\"cores_usage\": {}}}",
            cpu_info.total_cpus, cpu_info.total_cpu_usage, cpu_info.cores_usage
        );
        let mut res = Response::new(Bytes::from(cpu_data));
        res.headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        res.headers_mut()
            .insert("Access-Control-Allow-Origin", allow_origin.clone());
        return Ok(res.map(Full::new));
    }

    if request.uri().path() == "/uptime" {
        let uptime_info = services::os::uptime::get_uptime().unwrap();
        let response_data = format!("{{\"data\": \"{}\"}}", uptime_info);
        let mut res = Response::new(Bytes::from(response_data));
        res.headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        res.headers_mut()
            .insert("Access-Control-Allow-Origin", allow_origin.clone());
        return Ok(res.map(Full::new));
    }

    if request.uri().path() == "/cpu/history" {
        let cpu_history = database::sqlite::query::get_cpu_history(con).await.unwrap();
        let cpu_data = cpu_history
            .into_iter()
            .map(|(id, total_cpus, total_cpu_usage, cores_usage, created_at)| {
                format!(
                    "{{\"id\": {}, \"total_cpus\": {}, \"total_cpu_usage\": {}, \"cores_usage\": {}, \"created_at\": \"{}\"}}",
                    id, total_cpus, total_cpu_usage, cores_usage, created_at
                )
            })
            .collect::<Vec<String>>()
            .join(",");
        let response_data = format!("{{\"data\": [{}]}}", cpu_data);
        let mut res = Response::new(Bytes::from(response_data));
        res.headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        res.headers_mut()
            .insert("Access-Control-Allow-Origin", allow_origin.clone());
        return Ok(res.map(Full::new));
    }

    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
