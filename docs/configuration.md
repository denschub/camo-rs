# Configuration

CLI options and environment vars are equal in their usage. If both a CLI flag and an environment variable is set, the CLI flag takes precedence. For boolean CLI flags, they have no value. For environment vars, set them to either `true` or `false`, other values will not be accepted, and omitting them will set them to `false`.

## Required

- `--key` / `CAMO_KEY` - Randomly generated string used as a key for calculating the HMAC digest.
- `--root-url` / `CAMO_ROOT_URL` - URL, including a trailing slash, relative to the domain Camo is running on. For example, if Camo is available on `example.com/camo/`, set this to `/camo/`. For installations that do not run in a subdirectory, set this to `/`.

## Allowed content-types

At least one `content-type` needs to be allowed, or Camo will refuse to start.

- `--allow-audio` / `CAMO_ALLOW_AUDIO` - Whether `audio/*` MIME types should be allowed. (default: `false`)
- `--allow-image` / `CAMO_ALLOW_IMAGE` - Whether `image/*` MIME types should be allowed. (default: `false`)
- `--allow-video` / `CAMO_ALLOW_VIDEO` - Whether `video/*` MIME types should be allowed. (default: `false`)

Alternatively, you can set `--allow-all-types` / `CAMO_ALLOW_ALL_TYPES` (default: `false`), which will completely bypass any `content-type` checks, and thus allows all responses, even ones with a missing `content-type`.

## Other settings

- `--header-via` / `CAMO_HEADER_VIA` - The string used to identify this `camo-rs` instance in upstream requests. (default: `camo-rs asset proxy (+https://github.com/denschub/camo-rs)`)
- `--length-limit` / `CAMO_LENGTH_LIMIT` - The maximum `content-length` proxied by `camo-rs`. (default: `52428800` (50 MiB))
- `--listen` / `CAMO_LISTEN` - IP and Port this application should listen on. (default: `[::]:8081`)
- `--upstream-timeout` / `CAMO_UPSTREAM_TIMEOUT` - The number of seconds to wait for an upstream response. (default: `10`)

## Logging

By default, `camo-rs` is very quiet. It will only ever say anything if something goes wrong. Optional logging is available.

### Log levels

The `--log-level` flag or `CAMO_LOG_LEVEL` env var can have the following values:

- `quiet` - Doesn't log anything at all, unless something unexpected goes wrong. This is the default.
- `warn` - Logs when valid requests couldn't be processed due to upstream errors, or if requests have been blocked by length limits or content-type restrictions.
- `info` - Logs the same was `warn`, but additionally logs when the request field encoding was wrong, or if the HMAC was invalid.

### Log formats

The `--log-format` flag or `CAMO_LOG_FORMAT` env var can be set to:

- `text` - Does log the information in a human-readable format.
- `text-color` - The same as `text`, but with some color added to improve readability. This is the default.
- `json` - Logs the data in a machine-readable JSON format, with one log entry per line.
