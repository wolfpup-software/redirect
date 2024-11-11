use http::uri::Scheme;
use http::Uri;
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::body::Incoming as IncomingBody;
use hyper::header::LOCATION;
use hyper::http::{Request, Response};
use hyper::StatusCode;
use std::io;

const BAD_REQUEST_ERROR: &str = "cannot redirect https requests";
const INTERNAL_SERVER_ERROR: &str = "500 internal server error";

pub type BoxedResponse = Response<BoxBody<bytes::Bytes, io::Error>>;

pub async fn build_response(
    req: Request<IncomingBody>,
) -> Result<BoxedResponse, hyper::http::Error> {
    let mut dest_parts = req.uri().clone().into_parts();

    if let Some(schm) = dest_parts.scheme {
        if schm.as_str() == "https" {
            // bad request
            return create_error_response(&StatusCode::BAD_REQUEST, &BAD_REQUEST_ERROR);
        }
    };

    dest_parts.scheme = Some(Scheme::HTTPS);

    let dest_url = match Uri::from_parts(dest_parts) {
        Ok(u) => u,
        Err(_) => {
            // bad url
            return create_error_response(
                &StatusCode::INTERNAL_SERVER_ERROR,
                &INTERNAL_SERVER_ERROR,
            );
        }
    };

    Response::builder()
        .status(StatusCode::PERMANENT_REDIRECT)
        .header(LOCATION, dest_url.to_string())
        .body(
            Full::new(bytes::Bytes::new())
                .map_err(|e| match e {})
                .boxed(),
        )
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
