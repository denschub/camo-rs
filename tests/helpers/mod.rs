/// Common helpers to set up camo-rs in a test environment
pub mod application {
    use std::net::SocketAddr;

    use camo_rs::{AuthenticatedTarget, Settings};

    /// Builds a `Settings` instance with common values used throughout the
    /// test suite. This config will accept images, but block other mime-types.
    #[cfg(test)]
    pub fn get_test_settings() -> Settings {
        Settings {
            allow_audio: false,
            allow_image: true,
            allow_video: false,
            header_via: "camo-rs".to_owned(),
            key: "camo-rs".to_owned(),
            upstream_timeout: 10,

            // the test harness will always generate empty bodies, so any body
            // length that's there can be used to test the too-long checks.
            length_limit: 0,

            // full integration tests run on randomized ports and always on
            // localhost. these should not get used in tests.
            listen: "UNUSED! THIS WILL PANIC!".to_owned(),
            root_url: "UNUSED! THIS WILL PANIC!".to_owned(),
        }
    }

    pub fn get_test_url(listen_addr: SocketAddr, target: &AuthenticatedTarget) -> String {
        format!("http://{}/{}", listen_addr, target.encoded_full_path())
    }
}

/// Some helpers to make using Wiremock less boilerplate'y
pub mod wiremock {
    use std::time::Duration;

    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    /// Builds a response with some basic header values that should pass the
    /// Upstream Checks
    fn build_valid_response(status_code: u16, content_type: &str) -> ResponseTemplate {
        ResponseTemplate::new(status_code)
            .insert_header("x-upstream-header", "hello")
            .set_body_raw("", content_type)
    }

    async fn mount_one_time_mock_with_response(response: ResponseTemplate) -> MockServer {
        let mockserver = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(response)
            .expect(1)
            .mount(&mockserver)
            .await;

        mockserver
    }

    /// Sets up a Wiremock to respond one time to `GET /` with a given status
    /// code, and a `x-upstream-header` test header with `hello` as content.
    pub async fn get_single_file_mock(status_code: u16) -> MockServer {
        mount_one_time_mock_with_response(build_valid_response(status_code, "image/webp")).await
    }

    /// Sets up Wiremock to respond one time to `GET /` with a 200 status code,
    /// but it does delay the response by 60 seconds, and thus can be used for
    /// testing timeouts.
    pub async fn get_single_slow_file_mock() -> MockServer {
        mount_one_time_mock_with_response(
            build_valid_response(200, "image/webp").set_delay(Duration::from_secs(60)),
        )
        .await
    }

    /// Sets up Wiremock to respond one time to `GET /` with a 200 status code.
    /// The Mock does expect the `user-agent` and `via` headers to be set to
    /// `camo-rs`, and will fail otherwise.
    pub async fn get_single_file_mock_with_camo_headers() -> MockServer {
        let mockserver = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .and(header("user-agent", "camo-rs"))
            .and(header("via", "camo-rs"))
            .respond_with(build_valid_response(200, "image/webp"))
            .expect(1)
            .mount(&mockserver)
            .await;

        mockserver
    }

    /// Sets up a Wiremock to respond one time to `GET /` with a 200 status
    /// and a body that's too long for the test settings.
    pub async fn get_long_response_mock() -> MockServer {
        mount_one_time_mock_with_response(
            ResponseTemplate::new(200)
                .insert_header("x-upstream-header", "hello")
                .set_body_raw("loooooooong", "image/webp"),
        )
        .await
    }

    /// Sets up a Wiremock to respond one time to `GET /` with a 200 status
    /// code, but an empty content-type header
    pub async fn get_missing_content_type_mock() -> MockServer {
        mount_one_time_mock_with_response(
            ResponseTemplate::new(200).insert_header("x-upstream-header", "hello"),
        )
        .await
    }

    /// Sets up a Wiremock to respond one time to `GET /` with a 200 status
    /// code, but with a content-type of text/plain
    pub async fn get_textplain_content_type_mock() -> MockServer {
        mount_one_time_mock_with_response(build_valid_response(200, "text/plain")).await
    }

    /// Sets up a Wiremock to respond one time to `GET /`, but with a 302
    /// redirect to a different URL.
    pub async fn get_redirect_mock(location: &str) -> MockServer {
        mount_one_time_mock_with_response(
            ResponseTemplate::new(302).insert_header("location", location),
        )
        .await
    }
}
