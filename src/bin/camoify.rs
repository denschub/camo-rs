use clap::Parser;

use camo_rs::AuthenticatedTarget;

#[derive(clap::Parser, Debug)]
#[clap(
    name = "camoify",
    about = "Helper tool to encode an aribtrary URL for Camo",
    author,
    version
)]
struct Input {
    /// Randomly generated string used as a key for calculating the HMAC digest
    #[clap(short = 'k', long = "key", env = "CAMO_KEY")]
    key: String,

    /// The target URL
    #[clap()]
    target: String,
}

fn main() {
    let input = Input::parse();
    let target = AuthenticatedTarget::from_target(input.key.as_bytes(), &input.target);
    println!("/{}", target.encoded_full_path());
}
