use std::net::{SocketAddr, TcpListener};

use wiremock::MockServer;

use camo_rs::{server::*, AuthenticatedTarget};

pub mod helpers;
use helpers::{application::*, wiremock::*};

async fn run_test_server() -> (SocketAddr, reqwest::Client) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("could not bind ephemeral socket");
    let listen_addr = listener.local_addr().unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let mut settings = get_test_settings();
    settings.root_url = format!("http://{}/", listen_addr);

    let _ = tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(build(settings).into_make_service())
            .await
            .unwrap()
    });

    (listen_addr, client)
}

async fn run_valid_upstream_request(
    upstream: &MockServer,
) -> Result<reqwest::Response, reqwest::Error> {
    let settings = get_test_settings();
    let auth_target = AuthenticatedTarget::from_target(settings.key.as_bytes(), &upstream.uri());

    let (listen_addr, client) = run_test_server().await;
    client
        .get(get_test_url(listen_addr, &auth_target))
        .send()
        .await
}

#[tokio::test]
async fn passes_valid_requests() {
    let upstream = get_single_file_mock(200).await;
    let resp = run_valid_upstream_request(&upstream).await.unwrap();

    assert_eq!(resp.status(), 200);
}

/// This test just tests one example with a valid URL generated with the
/// wrong key. This is enough - if this fails, we know that our verification
/// logic in AuthenticatedTarget works, and that has unit tests.
#[tokio::test]
async fn rejects_invalid_targets() {
    let (listen_addr, client) = run_test_server().await;
    let auth_target =
        AuthenticatedTarget::from_target("some random key".as_bytes(), "http://example.com");

    let resp = client
        .get(get_test_url(listen_addr, &auth_target))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 403);
}

#[tokio::test]
async fn rejects_unexpected_status_codes() {
    let upstream = get_single_file_mock(418).await;
    let resp = run_valid_upstream_request(&upstream).await.unwrap();

    assert_eq!(resp.status(), 502);
}

#[tokio::test]
async fn rejects_long_responses() {
    let upstream = get_long_response_mock().await;
    let resp = run_valid_upstream_request(&upstream).await.unwrap();

    assert_eq!(resp.status(), 422);
}

#[tokio::test]
async fn rejects_missing_content_type() {
    let upstream = get_missing_content_type_mock().await;
    let resp = run_valid_upstream_request(&upstream).await.unwrap();

    assert_eq!(resp.status(), 422);
}

#[tokio::test]
async fn rejects_invalid_content_type() {
    let upstream = get_textplain_content_type_mock().await;
    let resp = run_valid_upstream_request(&upstream).await.unwrap();

    assert_eq!(resp.status(), 422);
}

#[tokio::test]
async fn rewrites_redirects_to_camo_urls() {
    let settings = get_test_settings();
    let (listen_addr, client) = run_test_server().await;

    let redirect_target = "https://example.com/another-site";
    let upstream = get_redirect_mock(redirect_target).await;

    let auth_target = AuthenticatedTarget::from_target(settings.key.as_bytes(), &upstream.uri());

    let resp = client
        .get(get_test_url(listen_addr, &auth_target))
        .send()
        .await
        .unwrap();

    let expected_target =
        AuthenticatedTarget::from_target(settings.key.as_bytes(), redirect_target)
            .encoded_full_path();

    assert_eq!(resp.status(), 302);
    assert_eq!(
        resp.headers().get("location").unwrap().to_str().unwrap(),
        format!("http://{}/{}", listen_addr, expected_target)
    );
}

#[tokio::test]
async fn rewrites_relative_redirects_to_absolute_camo_urls() {
    let settings = get_test_settings();
    let (listen_addr, client) = run_test_server().await;

    let redirect_target = "relative/redirect-target";
    let upstream = get_redirect_mock(redirect_target).await;

    let auth_target = AuthenticatedTarget::from_target(settings.key.as_bytes(), &upstream.uri());

    let resp = client
        .get(get_test_url(listen_addr, &auth_target))
        .send()
        .await
        .unwrap();

    let expected_full_url = format!("{}/{}", upstream.uri(), redirect_target);
    let expected_target =
        AuthenticatedTarget::from_target(settings.key.as_bytes(), &expected_full_url)
            .encoded_full_path();

    assert_eq!(resp.status(), 302);
    assert_eq!(
        resp.headers().get("location").unwrap().to_str().unwrap(),
        format!("http://{}/{}", listen_addr, expected_target)
    );
}
