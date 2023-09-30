# camo-rs

`camo-rs` is a frontend-compatible Rust-re-implementation of the now archived NodeJS-based [`atmos/camo`](https://github.com/atmos/camo) - an HTTP proxy for assets (mainly images) to route requests through an always-encrypted connection.

While initially designed for use in [diaspora\*](https://github.com/diaspora/diaspora), asset proxies like this are useful for all applications that display, for example, image files from external sources based on user input and want to avoid mixed-content warnings and reduce the number of external hosts the end-user has to connect to.

## URL format

To prevent abuse, only authorized URLs can be proxied through Camo. URLs requested from the application need to have the following format:

```
https://camo.example.org/<digest>/<asset-url>
```

Where

- the `digest` is a 40-character hexadecimal-encoded SHA1 HMAC digest computed with the shared secret key,
- the `asset-url` is a hexadecimal representation of the target URL, for example `687474703a2f2f65786d61706c652e636f6d2f6578616d706c652e6a7067` for `http://exmaple.com/example.jpg`.

## Differences to the original project

There are some differences to the original projects, namely:

- passing the `image-url` via a query parameter is not supported.
- `camo-rs` will not follow redirects. Instead, if a redirect is encountered upstream, the redirect response will be passed to the client, but with the `location` header modified to show a Camo-proxied version of the original location. This allows clients (and server-side logic) to cache permanent redirects.
- In addition to `GET` requests, `camo-rs` also accepts `HEAD` and `OPTIONS` requests and passes them through accordingly. This is useful if you want to verify the availability of URLs through Camo on the server side, or if CORS is relevant.

## Security considerations

Camo allows users to proxy essentially arbitrary files through it. If your application is vulnerable, Camo could be used to bypass cross-origin boundaries for assets. To reduce the risk a bit, `camo-rs` will always set the following headers in all of its proxied responses:

- `content-security-policy: default-src 'none'; img-src data:; style-src 'unsafe-inline'`
- `x-content-type-options: nosniff`
- `x-frame-options: deny`
- `x-xss-protection: 1; mode=block`

Which will reduce the amount of things you can do with the proxied resources significantly. In addition, `camo-rs` filters responses by `content-type`. Administrators can set flags to allow `audio/*`, `image/*`, and `video/*` MIME types in the config. Other content types will be rejected. `camo-rs` will reject to proxy resources without a `content-type` headers set. While providing this header is not required by the spec, real-world observations show that the vast majority of servers do, at least for static files, correctly set the `content-type` header. If this behavior is not desired, a setting to bypass all `content-type` checks is available.

## Changes to request and response headers

In addition to the security-relevant response header changes mentioned above, Camo will make some additional changes to the headers:

- Requests to the upstream will always have the `user-agent` and `via` headers set to the configured value.
- Responses will, in addition to the headers from the upstream, always have a `x-camo-original-url` header, showing the original URL without any encoding.

## Configuration

Configuration can be done via environment variables and CLI flags. The available configuration can be listed by running Camo with `--help`, but they're also documented at [`/docs/configuration.md`](/docs/configuration.md).

## Installation and Usage

Please see the additional documentation in the `docs` folder for details on

- [using released binaries](/docs/binaries.md)
- [deploying `camo-rs` with Docker](/docs/docker.md)
- [building `camo-rs` from source](/docs/from-source.md)
- [running `camo-rs` with `systemd`](/docs/systemd-unit.md)

## License

[MIT](/LICENSE). This projects takes heavy inspiration from [`atmos/camo`](https://github.com/atmos/camo), which also was licensed under [the MIT license](https://github.com/atmos/camo/blob/e59df56a01c023850962fac16905269d264fba50/LICENSE.md).
