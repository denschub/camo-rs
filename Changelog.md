# 1.0.0-dev

- `camo-rs` now refuses to start if no content-types are allowed. Before that, Camo would start up just fine, but reject everything, which can be confusing.
- When receiving a status code outside the expected range (`[200..399]`), Camo will still reject that request, but will pass the upstream status code to the client.
- Even with `RUST_LOG=warn`, the log will still contain the unencoded digest, target, and - if available - the encoded and validated target URL. This makes debugging production setups easier.

# 0.1.0

This is the first public release. While it is not considered "production-ready" yet, it should work, and is currently being tested in experimental rollouts.
