use std::{net::SocketAddr, str::FromStr};

use clap::Parser;

use camo_rs::{server, Settings};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let settings = Settings::parse();

    if !(settings.allow_audio || settings.allow_image || settings.allow_video) {
        println!(
            "ERROR: The configuration does not allow any content-type, and it \
            would block all requests. This isn't useful. Exiting."
        );
        std::process::exit(1);
    }

    let listen_addr = SocketAddr::from_str(&settings.listen).unwrap();
    let server = server::build(settings);
    axum::Server::bind(&listen_addr)
        .serve(server.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}
