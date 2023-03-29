use axum::{body::BoxBody, http::{Request, Response}};
use std::convert::Infallible;
use tracing::error;
use std::net::SocketAddr;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::{DateTime, Utc};

pub async fn log_request<T: Send + 'static>(
    start_time: DateTime<Utc>,
    addr: SocketAddr,
    req: Request<T>
) -> Result<Request<T>, Infallible> {
    let mut headers = String::new();
    for (name, value) in req.headers() {
        headers.push_str(&format!("{}: {}\n", name.as_str(), value.to_str().unwrap()));
    }
    let log_message = format!(
        "{} - {}, \"{} {}\", {}",
        start_time.format("%Y-%m-%d %H:%M:%S"),
        req.method(),
        req.uri(),
        addr,
        headers
    );
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("http-logs.log")
        .unwrap();
    writeln!(file, "{}", log_message).unwrap();
    Ok(req)
}

pub async fn log_response<T: Send + 'static>(
    res: Response<BoxBody>,
    req: Request<T>,
    start_time: DateTime<Utc>
) -> Result<Response<BoxBody>, Infallible> {
    let status = res.status();
    let duration = Utc::now().signed_duration_since(start_time).num_milliseconds();
    let log_message = format!(
        "{} - {}, \"{} {}\", {} ms\n",
        start_time.format("%Y-%m-%d %H:%M:%S"),
        req.method(),
        req.uri(),
        status.as_u16(),
        duration,
    );
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("http-logs.log")
        .unwrap();
    writeln!(file, "{}", log_message).unwrap();
    Ok(res)
}

pub async fn log_error(err: axum::Error, addr: SocketAddr, start_time: DateTime<Utc>) {
    error!("{} - {} - {}", addr, start_time, err);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("error-logs.log")
        .unwrap();
    writeln!(file, "{} - {}", start_time, err).unwrap();
}