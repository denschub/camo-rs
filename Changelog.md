# 1.4.2

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.4.1

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.4.0

This versions introduces a new setting, `--threads`/`CAMO_THREADS` that allows limiting the number of worker threads spawned by `camo-rs`. If this flag is not set, the number of available CPU cores will be used, which matches the current behavior.

# 1.3.5

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.3.4

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.3.3

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.3.2

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.3.1

This version does not contain any functional changes. It updates third-party dependencies and enables more compile-time optimizations. For most users, the only difference should be more consistent binary sizes, but there might be minimal performance improvements in larger deployments.

# 1.3.0

This version includes the following non-breaking changes:

- It adds a response to `/robots.txt`, disallowing all robots from interacting with `camo-rs` in any way. This was inspired by a recent uptick of bots that crawl Camo instances. Not all bots to listen to those instructions, but some do.
- It re-ships `jemalloc` inside the Docker container, but doesn't enable it. If you run the Docker container and want to use `jemalloc`, you can add a `LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2` to the environment variables, and it will automatically load it. Likewise, tuning `jemalloc` with the `MALLOC_CONF` environment variable is posisble,too.

# 1.2.2

This version does not contain any functional changes. It only updates third-party dependencies. Included are two updates that address security issues:

- an update to `h2` to resolve [a "moderate severity" Denial of Service attack](https://github.com/advisories/GHSA-8r5v-vm4m-4g25)
- an update to `mio` to resolve [a "high severity" Windows-only vulnerability](https://github.com/advisories/GHSA-r8w9-5wcg-vfj7)

# 1.2.1

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.2.0

This version adds a new setting, `--allow-all-types` / `CAMO_ALLOW_ALL_TYPES` (default: `false`). This allows you to use `camo-rs` with all upstream `content-type`s, and even passes responses that do not have a `content-type` set.

# 1.1.8

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.1.7

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.1.6

This version does not contain any functional changes.

# 1.1.5

This version does not contain any functional changes. It only updates third-party dependencies. Included in those is an update for `h2` to resolve [a potential Denial of Service attack](https://nvd.nist.gov/vuln/detail/CVE-2023-26964) when dealing with http/2 upstream connections.

# 1.1.4

- Offical Docker image: Drop `libjemalloc`. There have been two reports about increased idle memory usage when using `libjemalloc`, having `camo-rs` sit above the 100 MiB mark. While the allocator would release that memory under memory pressure, this application isn't designed for a usage where allocator performance is important anyway, so let's move back to the default system allocator for the official container image.

# 1.1.3

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.1.2

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.1.1

This version does not contain any functional changes. It only updates third-party dependencies.

# 1.1.0

- Switch from OpenSSL-bindings to Rustls, to avoid incompatibilities between OpenSSL 1.x and 3.x (looking at you, Canonical)...

# 1.0.0

This is the first release version of `camo-rs`, so there are no breaking changes to any previous stable release!

**For people running a 0.x prerelease**, these are the breaking changes between the latest pre-release and this stable release:

- For boolean settings like `CAMO_ALLOW_IMAGE`, you have to explicitly set `true` or `false` as values, and other values will be rejected as invalid. The previous behavior, where any specified value (like `yes` or confusingly even `no`) would be parsed as `true` has been removed. Omitting those fields will, however, still set them to `false` by default.

# 0.3.0

- You can now set the log output level with `--log-level`/`CAMO_LOG_LEVEL`, and the log output format with `--log-format`/`CAMO_LOG_FORMAT`. Please see [the documentation](/docs/configuration.md) for full details!

# 0.2.0

- `camo-rs` now refuses to start if no content-types are allowed. Before that, Camo would start up just fine, but reject everything, which can be confusing.
- When receiving a status code outside the expected range (`[200..399]`), Camo will still reject that request, but will pass the upstream status code to the client.
- Even with `RUST_LOG=warn`, the log will still contain the encoded digest, target, and - if available - the decoded and validated target URL. This makes debugging production setups easier.

# 0.1.0

This is the first public release. While it is not considered "production-ready" yet, it should work, and is currently being tested in experimental rollouts.
