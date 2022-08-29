use std::process::Command;

/// This is just a little build script that attempts to put the commit hash into
/// and rustc-env variable so we can embed that into the application.
fn main() {
    let mut git_hash = "unknown".to_string();

    if let Ok(output) = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        if let Ok(git_str_parse) = String::from_utf8(output.stdout) {
            if !git_str_parse.is_empty() {
                git_hash = git_str_parse.trim().to_string();
            }
        }
    }

    println!(
        "cargo:rustc-env=CAMO_RS_VERSION={} ({})",
        env!("CARGO_PKG_VERSION"),
        git_hash
    );
}
