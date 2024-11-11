use std::env;

use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use tokio::net::TcpListener;

mod service;
use service::build_response;

#[tokio::main]
async fn main() {
    let address = match env::args().nth(1) {
        Some(addr) => addr,
        None => return println!("argument error:\nurl authority not found (ie 127.0.0.1:3000)"),
    };

    let listener = match TcpListener::bind(address).await {
        Ok(lstnr) => lstnr,
        Err(e) => return println!("tcp listener error:\n{}", e),
    };

    loop {
        let (stream, _remote_address) = match listener.accept().await {
            Ok(strm) => strm,
            _ => {
                // log socket errors here
                continue;
            }
        };

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            // log service errors here
            Builder::new(TokioExecutor::new())
                .serve_connection(io, service_fn(build_response))
                .await
        });
    }
}
