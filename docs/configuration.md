# Configuration

CLI options and environment vars are equal in their usage. If both a CLI flag and an environment variable is set, the CLI flag takes precedence. For boolean CLI flags, they have no value. For environment vars, they evaluate as `true` as soon as they're set (yes, even a `CAMO_ALLOW_AUDIO=` will be `true`)

| CLI option           | Environment var         | Description                                                                                                                                                                                                                       | Default                                                      |
| -------------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `--allow-audio`      | `CAMO_ALLOW_AUDIO`      | Whether `audio/*` MIME types should be allowed.                                                                                                                                                                                   | `false`                                                      |
| `--allow-image`      | `CAMO_ALLOW_IMAGE`      | Whether `image/*` MIME types should be allowed.                                                                                                                                                                                   | `false`                                                      |
| `--allow-video`      | `CAMO_ALLOW_VIDEO`      | Whether `video/*` MIME types should be allowed.                                                                                                                                                                                   | `false`                                                      |
| `--header-via`       | `CAMO_HEADER_VIA`       | The string used to identify this `camo-rs` instance in upstream requests.                                                                                                                                                         | `camo-rs asset proxy (+https://github.com/denschub/camo-rs)` |
| `--key`              | `CAMO_KEY`              | Randomly generated string used as a key for calculating the HMAC digest.                                                                                                                                                          | unset                                                        |
| `--length-limit`     | `CAMO_LENGTH_LIMIT`     | The maximum `content-length` proxied by `camo-rs`.                                                                                                                                                                                | `52428800` (50 MiB)                                          |
| `--listen`           | `CAMO_LISTEN`           | IP and Port this application should listen on.                                                                                                                                                                                    | `[::]:8081`                                                  |
| `--root-url`         | `CAMO_ROOT_URL`         | URL, including a trailing slash, relative to the domain Camo is running on. For example, if Camo is available on `example.com/camo/`, set this to `/camo/`. For installations that do not run in a subdirectory, set this to `/`. | unset                                                        |
| `--upstream-timeout` | `CAMO_UPSTREAM_TIMEOUT` | The number of seconds to wait for an upstream response.                                                                                                                                                                           | `10`                                                         |

`CAMO_KEY` and `CAMO_ROOT_URL` are required. At least one `content-type` needs to be allowed, or Camo will refuse to start. All other settings are optional.

## Logging

By default, `camo-rs` is very quiet. It will only ever say anything if something goes wrong. You can enable some optional logs with a `RUST_LOG` environment var:

- `RUST_LOG=warn` will log when a valid request (i.e. valid URL and valid HMAC) could not be processed, for example due to an upstream failure or a rejected content type.
- `RUST_LOG=info` will, in addition, also log any attempts with an invalid Camo URL or invalid HMACs. This can be useful for debugging a fresh setup.
