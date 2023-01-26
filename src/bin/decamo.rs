use clap::Parser;

#[derive(clap::Parser, Debug)]
#[clap(
    name = "decamo",
    about = "Helper tool to decode a Camo URL",
    author,
    version
)]
struct Input {
    /// The URL to decamo
    #[clap()]
    url: String,
}

fn main() {
    let input = Input::parse();

    let target = input.url.split('/').last().unwrap();
    let target = hex::decode(target).unwrap();
    let target = String::from_utf8(target).unwrap();

    println!("{target}");
}
