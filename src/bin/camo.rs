use std::{net::SocketAddr, str::FromStr};

use clap::Parser;
use tokio::net::TcpListener;

use camo_rs::{Settings, server, settings::LogFormat};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

fn main() {
    let settings = Settings::parse();

    let mut rt = tokio::runtime::Builder::new_multi_thread();
    if let Some(threads) = settings.threads {
        rt.worker_threads(threads);
    }

    rt.enable_all()
        .build()
        .expect("building runtime not to fail")
        .block_on(async { run(settings).await })
}

async fn run(settings: Settings) {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(settings.log_level.tracing_level())
        .with_target(false);
    match settings.log_format {
        LogFormat::Text => subscriber.with_ansi(false).init(),
        LogFormat::TextColor => subscriber.with_ansi(true).init(),
        LogFormat::Json => subscriber.json().with_span_list(false).init(),
    }

    if !(settings.allow_all_types
        || settings.allow_audio
        || settings.allow_image
        || settings.allow_video)
    {
        println!(
            "ERROR: The configuration does not allow any content-type, and it \
            would block all requests. This isn't useful. Exiting."
        );
        std::process::exit(1);
    }

    let listen_addr = SocketAddr::from_str(&settings.listen).unwrap();
    let listener = TcpListener::bind(&listen_addr).await.unwrap();

    let server = server::build(settings);
    axum::serve(listener, server.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}
