//! Collection of functions to deal with Camo's request and response headers.

use axum::http::{HeaderMap, HeaderValue};
use hyper::header::{self, HeaderName};
use url::Url;

/// These are the only headers that will be passed to the upstream. The main
/// motivation is to avoid leaking potential secrets like the Cookie or
/// Authorization headers, but it probably also helps to reduce client
/// fingerprinting. Maybe. This isn't really a privacy-product anyway.
pub const ALLOWED_REQUEST_HEADERS: [HeaderName; 8] = [
    header::ACCEPT,
    header::ACCEPT_ENCODING,
    header::ACCEPT_LANGUAGE,
    header::CACHE_CONTROL,
    header::IF_MODIFIED_SINCE,
    header::IF_NONE_MATCH,
    header::PRAGMA,
    header::TE,
];

/// These headers get set to the provided value in any HTTP response that gets
/// proxied through Camo. The motivation here is to reduce the security risk
/// of running a proxy like this. For example, the CSP header ensures that no
/// JavaScript inside a SVG can do Evil Stuff(tm).
pub const SECURE_RESPONSE_HEADERS: [(HeaderName, &str); 4] = [
    (
        header::CONTENT_SECURITY_POLICY,
        "default-src 'none'; img-src data:; style-src 'unsafe-inline'",
    ),
    (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
    (header::X_FRAME_OPTIONS, "deny"),
    (header::X_XSS_PROTECTION, "1; mode=block"),
];

/// Assigns allowlisted request headers from an original `HeaderMap` into a
/// second map for use in the request to the upstream.
///
/// Check the definition of `const ALLOWED_REQUEST_HEADERS` for the copied
/// header names. Existing values in the target `HeaderMap` for allow-listed
/// headers will be overwriten, but other headers will be left untouched.
pub fn assign_filtered_request_headers(from: &HeaderMap, to: &mut HeaderMap) {
    for key in ALLOWED_REQUEST_HEADERS {
        if let Some(value) = from.get(&key) {
            if let Some(header) = to.get_mut(&key) {
                *header = value.clone();
            } else {
                to.append(&key, value.clone());
            }
        }
    }
}

/// Sets response headers to increase security a bit.
///
/// Check the definition of `const SECURE_RESPONSE_HEADERS` for details. If one
/// of the headers is already set, this function will override the values. All
/// other existing headers will not be touched.
pub fn force_secure_response_headers(headers: &mut HeaderMap) {
    for (key, value) in SECURE_RESPONSE_HEADERS {
        let header_value = HeaderValue::from_static(value);
        if let Some(header) = headers.get_mut(&key) {
            *header = header_value;
        } else {
            headers.append(&key, header_value);
        }
    }
}

/// Tries to resolve a URL from a Location header and turns absolute redirect
/// targets into absolute URLs.
pub fn resolve_location_header(base: &str, location: &str) -> Result<String, url::ParseError> {
    let base = Url::parse(base)?;
    Ok(base.join(location)?.to_string())
}
