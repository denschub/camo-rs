//! The Application Settings Module(tm)

/// Application Settings Struct, designed to be primarily used by Clap
#[derive(clap::Parser, Clone, Debug)]
#[clap(about, author, version = env!("CAMO_RS_VERSION"))]
pub struct Settings {
    /// If present, `audio/*` MIME types will be allowed
    #[clap(long = "allow-audio", env = "CAMO_ALLOW_AUDIO")]
    pub allow_audio: bool,

    /// If present, `image/*` MIME types will be allowed
    #[clap(long = "allow-image", env = "CAMO_ALLOW_IMAGE")]
    pub allow_image: bool,

    /// If present, `video/*` MIME types will be allowed
    #[clap(long = "allow-video", env = "CAMO_ALLOW_VIDEO")]
    pub allow_video: bool,

    /// The string used to identify this instance in upstream requests in Via and User-Agent
    #[clap(
        long = "header-via",
        env = "CAMO_HEADER_VIA",
        default_value = "camo-rs asset proxy (+https://github.com/denschub/camo-rs)"
    )]
    pub header_via: String,

    /// Randomly generated string used as a key for calculating the HMAC digest
    #[clap(long = "key", env = "CAMO_KEY")]
    pub key: String,

    /// The maximum `content-length`
    #[clap(
        long = "length-limit",
        env = "CAMO_LENGTH_LIMIT",
        default_value_t = 52428800
    )]
    pub length_limit: usize,

    /// IP and Port this instance should listen on
    #[clap(long = "listen", env = "CAMO_LISTEN", default_value = "[::]:8081")]
    pub listen: String,

    /// URL, including a trailing slash, relative to the domain Camo is running
    /// on. For example, if Camo is available on `example.com/camo/`, set this
    /// to `/camo/`. For installations that do not run in a subdirectory, set
    /// this to `/`.
    #[clap(long = "root-url", env = "CAMO_ROOT_URL")]
    pub root_url: String,

    /// The number of seconds to wait for an upstream response
    #[clap(
        long = "upstream-timeout",
        env = "CAMO_UPSTREAM_TIMEOUT",
        default_value_t = 10
    )]
    pub upstream_timeout: usize,
}
