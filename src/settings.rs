//! The Application Settings Module(tm)

use tracing::Level;

/// Specifies the log's output format
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LogFormat {
    /// Logs in a human-readable format.
    Text,

    /// Logs in a human-readable format, but with colors.
    TextColor,

    /// Logs into a machine-readable JSONL format.
    Json,
}

/// Specifies how much log output Camo generates
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LogLevel {
    /// Does not log anything at all
    Quiet,

    /// Logs upstream errors and blocked upstream requests
    Warn,

    /// Everything that `warn` includes, but also logs invalid URLs/HMACs
    Info,
}

impl LogLevel {
    pub fn tracing_level(&self) -> Level {
        match self {
            LogLevel::Quiet => Level::ERROR,
            LogLevel::Warn => Level::WARN,
            LogLevel::Info => Level::INFO,
        }
    }
}

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

    /// If present, `json/*` MIME types will be allowed
    #[clap(long = "allow-json", env = "CAMO_ALLOW_JSON")]
    pub allow_json: bool,

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

    /// Defines how the log output will be formatted
    #[clap(value_enum, long = "log-format", env = "CAMO_LOG_FORMAT", default_value_t = LogFormat::TextColor)]
    pub log_format: LogFormat,

    /// The amount of log output. See the documentation for details
    #[clap(value_enum, long = "log-level", env = "CAMO_LOG_LEVEL", default_value_t = LogLevel::Quiet)]
    pub log_level: LogLevel,

    /// URL, including a trailing slash, relative to the domain Camo is running
    /// on
    ///
    /// For example, if Camo is available on `example.com/camo/`, set this
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
