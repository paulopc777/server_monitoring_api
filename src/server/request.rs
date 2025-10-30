use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};

use http::{HeaderValue, Method, Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Frame};
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

async fn response(data: &str, status: StatusCode) -> Result<Response<Full<Bytes>>, Infallible> {
    let allow_origin = HeaderValue::from_static("*");
    let allow_methods = HeaderValue::from_static("GET, POST, DELETE, OPTIONS");
    let allow_headers = HeaderValue::from_static("Content-Type, Authorization");

    let res = Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", allow_origin)
        .header("Access-Control-Allow-Methods", allow_methods)
        .header("Access-Control-Allow-Headers", allow_headers)
        .body(Full::new(Bytes::from(data.to_string())))
        .unwrap();

    Ok(res)
}

pub async fn received_request(
    request: Request<hyper::body::Incoming>,
    sys: &System,
    con: Arc<Mutex<Connection>>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let allow_origin = HeaderValue::from_static("*");
    let allow_methods = HeaderValue::from_static("GET, POST, DELETE, OPTIONS");
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

    if request.uri().path() == "/urls" && request.method() == Method::GET {
        let response = database::sqlite::urls::get_urls(&con);
        let urls_data = response
            .into_iter()
            .map(|url_data| {
                format!(
                    "{{\"id\": {}, \"url\": \"{}\", \"status_code\": \"{:?}\", \"created_at\": \"{}\"}}",
                    url_data.id, url_data.url, url_data.status_code, url_data.created_at
                )
            })
            .collect::<Vec<String>>()
            .join(",");
        let response_data = format!("{{\"data\": [{}]}}", urls_data);
        let mut res = Response::new(Bytes::from(response_data));
        res.headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        res.headers_mut()
            .insert("Access-Control-Allow-Origin", allow_origin.clone());
        return Ok(res.map(Full::new));
    }

    if request.uri().path() == "/urls" && request.method() == Method::POST {
        let body_bytes = request.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        // Parse manual do JSON para extrair a URL
        if let Some(start) = body_str.find("\"url\"") {
            if let Some(colon_pos) = body_str[start..].find(':') {
                let after_colon = &body_str[start + colon_pos + 1..];
                if let Some(quote_start) = after_colon.find('"') {
                    if let Some(quote_end) = after_colon[quote_start + 1..].find('"') {
                        let url = &after_colon[quote_start + 1..quote_start + 1 + quote_end];

                        // Salvar a URL no banco de dados
                        let result = database::sqlite::urls::create_url(con, url, None);

                        let response_data = match result {
                            Ok(_) => format!(
                                "{{\"message\": \"URL criada com sucesso\", \"url\": \"{}\"}}",
                                url
                            ),
                            Err(_) => "{\"error\": \"Erro ao criar URL\"}".to_string(),
                        };

                        let mut res = Response::new(Bytes::from(response_data));
                        res.headers_mut()
                            .insert("Content-Type", HeaderValue::from_static("application/json"));
                        res.headers_mut()
                            .insert("Access-Control-Allow-Origin", allow_origin.clone());
                        return Ok(res.map(Full::new));
                    }
                }
            }
        }

        // Resposta de erro se JSON inválido ou sem campo 'url'
        let error_response = "{\"error\": \"JSON inválido ou campo 'url' ausente\"}";
        let mut res = Response::new(Bytes::from(error_response));
        res.headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        res.headers_mut()
            .insert("Access-Control-Allow-Origin", allow_origin.clone());
        return Ok(res.map(Full::new));
    }

    if request.uri().path() == "/urls" && request.method() == Method::DELETE {
        let body_bytes = request.into_body().collect().await.unwrap().to_bytes();
        let json = serde_json::from_slice::<serde_json::Value>(&body_bytes);

        if json.is_err() {
            let response = response(
                "{\"error\": \"JSON inválido\"}",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .await;
            return response;
        }
        let json = json.unwrap();
        let id = json.get("id");
        if id.is_none() || !id.unwrap().is_i64() {
            let response = response(
                "{\"error\": \"ID inválido ou ausente\"}",
                StatusCode::BAD_REQUEST,
            )
            .await;
            return response;
        }

        let id = id.unwrap().as_i64().unwrap() as i32;
        let result = database::sqlite::urls::delete_url(&con.lock().unwrap(), id);
        let response_data = match result {
            Ok(_) => format!(
                "{{\"message\": \"URL com ID {} deletada com sucesso\"}}",
                id
            ),
            Err(_) => "{\"error\": \"Erro ao deletar URL\"}".to_string(),
        };
        let response = response(&response_data, StatusCode::OK).await;
        return response;
    }

    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
