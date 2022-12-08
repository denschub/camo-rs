//! Collection of Error types used by camo-rs

use std::string::FromUtf8Error;

use axum::response::{IntoResponse, Response};
use hex::FromHexError;
use hyper::{header, Body, StatusCode};
use thiserror::Error;
use tracing::{info, warn};

/// Error returned during parsing Authentication details (HMAC and Target
/// provided via URL parameters).
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthParsingError {
    /// Returned if the Digest has a non-hex format.
    #[error("digest is not encoded as hex: {0}")]
    DigestEncodingError(#[source] FromHexError),

    /// Returned if the provided key is empty.
    #[error("the provided key is empty")]
    EmptyKeyError,

    /// Returned if the Target URL has a non-hex format.
    #[error("target url is not encoded as hex: {0}")]
    TargetEncodingError(#[source] FromHexError),

    /// Returned if the Target URL cannot be encoded into a utf8 string.
    #[error("target url is not a utf8 string: {0}")]
    TargetNotUtf8(#[source] FromUtf8Error),
}

/// Error returned during validating the URL-provided HMAC.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthValidationError {
    /// Returned if the input Digest is not what we expect.
    #[error("HMAC is invalid")]
    HmacInvalid,
}

/// Error returned during the Camo Processing Pipeline.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CamoError {
    /// Returned if parsing the URL-provided authentication data failed.
    #[error("authentication data could not be parsed: {0}")]
    AuthParsingError(#[source] AuthParsingError),

    /// Returned if the URL-provided authentication data is invalid.
    #[error("authentication data was invalid: {0}")]
    AuthValidationError(#[source] AuthValidationError),

    /// Returned if the returned content-type is invalid.
    #[error("upstream content-type not accepted: {0}")]
    ContentTypeNotAccepted(String),

    /// Returned if the upstream didn't send a content-type.
    #[error("upstream did not provide a content-type")]
    MissingContentType,

    /// Returned if the Upstream Proxy failed.
    #[error("upstream proxy failed: {0}")]
    ProxyError(#[source] ProxyError),

    /// Returned when the upstream returns an unexpected status code
    #[error("unexpected upstream status: {0}")]
    UnexpectedUpstreamStatus(u16),

    /// Returned if the upstream returned a redirect, but we couldn't process
    /// the Location header
    #[error("upstream redirect location: header not processable")]
    UpstreamRedirectLocationUnprocessable,

    /// Returned if the upstream content-length exceeds the limit.
    #[error("upstream content-length exceeds limit")]
    UpstreamResponseTooLong(usize),
}

impl CamoError {
    fn status_code(&self) -> StatusCode {
        use CamoError::*;

        match self {
            AuthParsingError(_) | AuthValidationError(_) => StatusCode::FORBIDDEN,
            ContentTypeNotAccepted(_)
            | MissingContentType
            | UpstreamRedirectLocationUnprocessable
            | UpstreamResponseTooLong(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ProxyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UnexpectedUpstreamStatus(status_code) => {
                StatusCode::from_u16(*status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

impl IntoResponse for CamoError {
    fn into_response(self) -> Response {
        use CamoError::*;

        // [ToDo] Logging in here feels quite a bit dirty. However, this let's
        // me get rid of the verbose logic inside the handler completely, so
        // let's keep it for now...
        match self {
            AuthParsingError(_) | AuthValidationError(_) => {
                info!("{:?}", self);
            }
            _ => {
                warn!("{:?}", self);
            }
        }

        Response::builder()
            .status(self.status_code())
            .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(Body::from(self.to_string()))
            .expect("Generating this error never fails")
            .into_response()
    }
}

/// Error returned from the proxy if the Upstream request failed.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProxyError {
    /// Returned if building the upstream request failed, usually due to invalid
    /// parameters like target URLs, headers, ...
    #[error("building the upstream request failed: {0}")]
    RequestBuildingFailed(#[source] hyper::http::Error),

    /// Returned if the request to the upstream failed.
    #[error("upstream error: {0}")]
    UpstreamError(#[source] hyper::Error),

    /// Returned if the connection didn't get established until the configurable
    /// timeout expired.
    #[error("upstream request timed out: {0}")]
    UpstreamTimeout(#[source] tokio::time::error::Elapsed),
}
