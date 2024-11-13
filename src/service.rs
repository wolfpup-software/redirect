use http::uri::{Authority, Scheme};
use http::Uri;
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::body::Incoming as IncomingBody;
use hyper::header::{HOST, LOCATION};
use hyper::http::{Request, Response};
use hyper::service::Service;
use hyper::StatusCode;
use std::future::Future;
use std::io;
use std::pin::Pin;

const BAD_REQUEST_ERROR: &str = "cannot redirect https requests";
const BAD_REQUEST_URL_ERROR: &str = "cannot redirect when url does not have an authority";
const INTERNAL_SERVER_ERROR: &str = "500 internal server error";

pub type BoxedResponse = Response<BoxBody<bytes::Bytes, io::Error>>;

pub struct Svc {}

impl Service<Request<IncomingBody>> for Svc {
    type Response = BoxedResponse;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        Box::pin(async move { build_response(req).await })
    }
}

pub async fn build_response(
    req: Request<IncomingBody>,
) -> Result<BoxedResponse, hyper::http::Error> {
    let mut dest_parts = req.uri().clone().into_parts();
    dest_parts.scheme = Some(Scheme::HTTPS);

    // if http 1.1
    if None == dest_parts.authority {
        if let Some(host_header) = req.headers().get(HOST) {
            if let Ok(uri) = Uri::from_maybe_shared(host_header.clone()) {
                if let Some(athrty) = uri.authority() {
                    dest_parts.authority = Some(athrty.clone());
                }
            }
        };
    }

    if let Ok(dest_url) = Uri::from_parts(dest_parts) {
        return Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header(LOCATION, dest_url.to_string())
            .body(
                Full::new(bytes::Bytes::new())
                    .map_err(|e| match e {})
                    .boxed(),
            );
    };

    create_error_response(&StatusCode::BAD_REQUEST, &BAD_REQUEST_ERROR)
}

fn create_error_response(
    code: &StatusCode,
    body: &'static str,
) -> Result<BoxedResponse, hyper::http::Error> {
    Response::builder().status(code).body(
        Full::new(bytes::Bytes::from(body))
            .map_err(|e| match e {})
            .boxed(),
    )
}
