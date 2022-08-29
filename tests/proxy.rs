use hyper::{HeaderMap, Method};
use wiremock::MockServer;

use camo_rs::{errors::ProxyError, proxy::*};

pub mod helpers;
use helpers::wiremock::*;

async fn run_proxy_request(
    upstream: &MockServer,
) -> Result<hyper::Response<hyper::Body>, ProxyError> {
    let proxy = Proxy::new("camo-rs", 10);
    let headers = HeaderMap::new();

    proxy
        .run_request(&Method::GET, &headers, &upstream.uri())
        .await
}

#[tokio::test]
async fn fails_gracefully_for_invalid_params() {
    let proxy = Proxy::new("camo-rs", 10);
    let headers = HeaderMap::new();

    let proxy_res = proxy.run_request(&Method::GET, &headers, "").await;

    assert!(proxy_res.is_err());
}

#[tokio::test]
async fn fails_gracefully_for_timeouting_connections() {
    let upstream = get_single_slow_file_mock().await;
    let proxy = Proxy::new("camo-rs", 1); // Note the 1 second timeout.
    let headers = HeaderMap::new();

    let proxy_res = proxy
        .run_request(&Method::GET, &headers, &upstream.uri())
        .await;

    assert!(proxy_res.is_err());
}

#[tokio::test]
async fn proxies_a_request() {
    let upstream = get_single_file_mock(200).await;
    let proxy_res = run_proxy_request(&upstream).await.unwrap();

    assert_eq!(proxy_res.status(), 200);
}

#[tokio::test]
async fn passes_errors_without_failing() {
    let upstream = get_single_file_mock(500).await;
    let proxy_res = run_proxy_request(&upstream).await.unwrap();

    assert_eq!(proxy_res.status(), 500);
}

#[tokio::test]
async fn passes_upstream_headers() {
    let upstream = get_single_file_mock(200).await;
    let proxy_res = run_proxy_request(&upstream).await.unwrap();

    assert_eq!(
        proxy_res.headers().get("x-upstream-header").unwrap(),
        "hello"
    )
}

#[tokio::test]
async fn sets_the_camo_via_and_ua() {
    let upstream = get_single_file_mock_with_camo_headers().await;
    let _ = run_proxy_request(&upstream).await.unwrap();

    // no assertions - test will happen when the mock goes out of scope.
}

#[tokio::test]
async fn sets_the_original_url_response_header() {
    let upstream = get_single_file_mock(200).await;
    let proxy_res = run_proxy_request(&upstream).await.unwrap();

    assert_eq!(
        proxy_res.headers().get("x-camo-original-url").unwrap(),
        &upstream.uri()
    );
}

#[tokio::test]
async fn sets_response_security_headers() {
    let upstream = get_single_file_mock(200).await;
    let proxy_res = run_proxy_request(&upstream).await.unwrap();

    // This test is only testing the existence of a single security header.
    // There are tests for header_wrangler, this is more of a "did the
    // headers go through the machine" kind of test.
    assert!(proxy_res.headers().get("content-security-policy").is_some());
}
