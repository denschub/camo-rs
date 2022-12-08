//! Hyper-based HTTP proxy to connect to the upstream

use std::time::Duration;

use axum::http::HeaderValue;
use hyper::{header, Body, HeaderMap, Method, Request, Response};

use crate::{errors::ProxyError, header_wrangler};

/// The thing that actually does the requests to the upstream!
#[derive(Clone)]
pub struct Proxy {
    via_header: String,
    upstream_timeout: usize,
    http_client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl Proxy {
    /// Creates a new Proxy instance with a set Via/User-Agent value and
    /// a timeout.
    ///
    /// This will internally also create the hyper HttpsConnector and hyper
    /// Client, which will be used throughout the life of this Proxy.
    pub fn new(via_header: &str, upstream_timeout: usize) -> Self {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();
        let http_client = hyper::Client::builder().build::<_, Body>(https);

        Self {
            via_header: via_header.to_owned(),
            upstream_timeout,
            http_client,
        }
    }

    /// Runs a request to the upstream, and returns the Response to be used
    /// further down the app, for example for body streaming.
    ///
    /// This function will make sure that request headers are filtered, and it
    /// will ensure the response headers have the security header set.
    pub async fn run_request(
        &self,
        method: &Method,
        headers: &HeaderMap,
        target: &str,
    ) -> Result<Response<Body>, ProxyError> {
        let mut req = Request::builder()
            .method(method)
            .uri(target)
            .header(header::USER_AGENT, &self.via_header)
            .header(header::VIA, &self.via_header)
            .body(Body::empty())
            .map_err(ProxyError::RequestBuildingFailed)?;

        header_wrangler::assign_filtered_request_headers(headers, req.headers_mut());

        let request_future = self.http_client.request(req);
        let mut res = tokio::time::timeout(
            Duration::from_secs(self.upstream_timeout as u64),
            request_future,
        )
        .await
        .map_err(ProxyError::UpstreamTimeout)?
        .map_err(ProxyError::UpstreamError)?;

        header_wrangler::force_secure_response_headers(res.headers_mut());
        res.headers_mut().append(
            "x-camo-original-url",
            HeaderValue::from_str(target).expect("target is always a valid URL at this point"),
        );

        Ok(res)
    }
}
