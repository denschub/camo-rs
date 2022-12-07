//! The Glue that makes Magic happen

use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::HeaderValue,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::{
    header::{self, HeaderName},
    Body, HeaderMap, Method,
};
use tracing::{instrument, Span};

use crate::{
    errors::CamoError, header_wrangler::resolve_location_header, AuthenticatedTarget, Proxy,
    Settings,
};

#[derive(Clone)]
pub struct AppState {
    settings: Settings,
    proxy: Proxy,
}

/// Builds the router. This doesn't plug this into a server, so you need to
/// do that yourself.
pub fn build(settings: Settings) -> Router {
    let proxy = Proxy::new(&settings.header_via, settings.upstream_timeout);
    let state = AppState { settings, proxy };

    Router::new()
        .route(
            "/:digest/:target",
            get(proxy_handler)
                .head(proxy_handler)
                .options(proxy_handler),
        )
        .route("/__heartbeat__", get(heartbeat_handler))
        .route("/__version__", get(version_handler))
        .fallback(fallback_handler)
        .with_state(state)
}

/// The handler for all GET/HEAD/OPTION requests to a URL in the right format.
/// This is a wrapper around `process_camo_request` to allow for reasonable
/// HTTP responses depending on what goes wrong.
#[instrument(level = "warn", skip_all, fields(req_digest, req_target, target_url))]
async fn proxy_handler(
    State(app_state): State<AppState>,
    Path((req_digest, req_target)): Path<(String, String)>,
    req_method: Method,
    req_headers: HeaderMap,
) -> impl IntoResponse {
    // [ToDo] I'm currently skipping all arguments and then manually re-adding
    // them, as otherwise, I get a double-qouted JSON output, so instead of
    // `"req_digest":"aaa"`, I get `"req_digest":"\"aaa\""` - which is rather
    // hard to process. This is probably a bug somewhere, but I have to spend
    // some time debugging this.
    Span::current().record("req_digest", &req_digest);
    Span::current().record("req_target", &req_target);

    let result =
        process_camo_request(app_state, req_digest, req_target, req_method, req_headers).await;

    // explicitly call into_reponse() here instead of returning the result to
    // allow the into_response() handler to run inside this tracing span, which
    // is important for the log output.
    result.into_response()
}

async fn heartbeat_handler() -> impl IntoResponse {
    get_response_with_status_and_text(200, "ok")
}

async fn version_handler() -> impl IntoResponse {
    get_response_with_status_and_text(200, env!("CAMO_RS_VERSION"))
}

async fn fallback_handler() -> impl IntoResponse {
    get_response_with_status_and_text(404, "Not found!")
}

/// The function that actually does all the work. This isn't happening directly
/// directly inside the header to allow to return a CamoError early.
async fn process_camo_request(
    app_state: AppState,
    req_digest: String,
    req_target: String,
    req_method: Method,
    req_headers: HeaderMap,
) -> Result<Response<Body>, CamoError> {
    let settings = app_state.settings;

    let authenticated_target = AuthenticatedTarget::from_encoded_strings(
        settings.key.as_bytes(),
        &req_digest,
        &req_target,
    )
    .map_err(CamoError::AuthParsingError)?;

    let target = authenticated_target
        .validated_target_url()
        .map_err(CamoError::AuthValidationError)?;

    Span::current().record("target_url", &target);

    let mut upstream_res = app_state
        .proxy
        .run_request(&req_method, &req_headers, &target)
        .await
        .map_err(CamoError::ProxyError)?;

    if !(upstream_res.status().is_success() || upstream_res.status().is_redirection()) {
        return Err(CamoError::UnexpectedUpstreamStatus(
            upstream_res.status().as_u16(),
        ));
    }

    // Try to get the content-length, and validate it. Unfortunately, it seems to
    // be impossible to rely on it being there - there's too many servers out
    // there not sending a content-length header... m(
    let maybe_content_length =
        try_parse_header::<usize>(upstream_res.headers(), &header::CONTENT_LENGTH);
    if let Some(content_length) = maybe_content_length {
        if content_length > settings.length_limit {
            return Err(CamoError::UpstreamResponseTooLong(content_length));
        }
    }

    // For everything that is not a 3xx status code on a GET request, let's
    // enforce content-types. This will break some misconfigured servers, but
    // that's worth it.
    if req_method == Method::GET && !upstream_res.status().is_redirection() {
        let maybe_content_type =
            try_parse_header::<String>(upstream_res.headers(), &header::CONTENT_TYPE);
        if let Some(content_type) = maybe_content_type {
            let is_accepted = (settings.allow_audio && content_type.starts_with("audio/"))
                || (settings.allow_image && content_type.starts_with("image/"))
                || (settings.allow_video && content_type.starts_with("video/"));

            if !is_accepted {
                return Err(CamoError::ContentTypeNotAccepted(content_type));
            }
        } else {
            return Err(CamoError::MissingContentType);
        }
    }

    // Contrary to the original Camo, camo-rs does not follow redirects received
    // from the upstream. Instead, we pass the redirect along to the client,
    // which allows redirects to be cached by the client.
    // However, instead of providing the original Location, we have to wrap that
    // in a Camo URL again so that the redirect will be tunneled through Camo...
    let maybe_location = try_parse_header::<String>(upstream_res.headers(), &header::LOCATION);
    if let Some(location) = maybe_location {
        if let Ok(resolved_location) = resolve_location_header(&target, &location) {
            let new_target =
                AuthenticatedTarget::from_target(settings.key.as_bytes(), &resolved_location);
            let new_target = format!("{}{}", settings.root_url, new_target.encoded_full_path());

            let location_header = upstream_res
                .headers_mut()
                .get_mut(&header::LOCATION)
                .expect("header must be there if we could parse it");
            *location_header = HeaderValue::from_str(&new_target)
                .expect("AuthenticatedTarget doesn't generate invalid URLs")
        } else {
            return Err(CamoError::UpstreamRedirectLocationUnprocessable);
        }
    }

    Ok(upstream_res)
}

/// Small helper to build a response with a provided status code and a plain
/// text body.
fn get_response_with_status_and_text(status: u16, text: &str) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from(text.to_owned()))
        .expect("this is filled with static data only and should not fail")
}

/// Small helper to try to get a specific header from a HeaderMap and return its
/// value in a FromStr'able type.
fn try_parse_header<T: FromStr>(headers: &HeaderMap, name: &HeaderName) -> Option<T> {
    headers.get(name)?.to_str().ok()?.parse().ok()
}
