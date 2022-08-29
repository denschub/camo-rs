use camo_rs::header_wrangler::*;
use hyper::{header, HeaderMap};

#[test]
fn assign_filtered_request_headers_does_work() {
    let mut original_headers = HeaderMap::new();
    original_headers.append(header::AUTHORIZATION, "example".parse().unwrap());
    original_headers.append(header::COOKIE, "example".parse().unwrap());
    original_headers.append(header::USER_AGENT, "example".parse().unwrap());

    let mut new_headers = HeaderMap::new();
    assign_filtered_request_headers(&original_headers, &mut new_headers);
    assert_eq!(new_headers.len(), 0);
}

#[test]
fn force_secure_response_headers_does_work() {
    let mut headers = HeaderMap::new();
    force_secure_response_headers(&mut headers);
    assert_eq!(headers.len(), SECURE_RESPONSE_HEADERS.len());
}

#[test]
fn force_secure_response_headers_does_not_override_existing() {
    let mut headers = HeaderMap::new();
    headers.append(header::CONTENT_LENGTH, "4242".parse().unwrap());

    force_secure_response_headers(&mut headers);
    assert_eq!(headers.len(), SECURE_RESPONSE_HEADERS.len() + 1);
}
