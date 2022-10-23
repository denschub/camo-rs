# Deploying `camo-rs` with Docker

`camo-rs` containers are [published on Docker Hub as `denschub/camo-rs`](https://hub.docker.com/r/denschub/camo-rs).

## Available tags

Stable releases are available as tags based on their version number:

- `denschub/camo-rs:latest` always points to the current release version,
- `denschub/camo-rs:1` always points to the latest release in the `1.*.*` major version line,
- `denschub/camo-rs:1.0` always points to the latest release in the `1.0.*` minor version line,
- `denschub/camo-rs:1.0.2` points to a specific version.

For production deployments, it is recommended that you pin a major version, so you don't have to update your setup for updates, but you don't get new major versions that may contain breaking changes.

Additionally, the `denschub/camo-rs:develop` tag always points to the current `develop` version. That maybe useful for testing, but it's not recommended to deploy that into production. :)

## Health check

The Docker container defines a health check:

```
HEALTHCHECK CMD curl -f http://localhost:8081/__heartbeat__ || exit 1
```

That endpoint will always return with a `200` when `camo-rs` is alive. If you change the `CAMO_LISTEN` setting to a different port, keep in mind that you have to redefine the health check.

## Configuration

Ideally, configuration is done via environment variables. See [the configuration documentation](/docs/configuration.md) for all available fields.

By default, `camo-rs` listens to `[::]:8081`, and the Docker container exposes that port, so you don't need to set the listen config. Since Camo has no state, there are no volumes to take care off.

## `docker-compose.yml` example

To use `camo-rs` with Docker Compose, you can use a configuration close to this:

```yml
version: "3"
services:
  camo:
    image: denschub/camo-rs:latest
    restart: always
    environment:
      - CAMO_KEY=supersecretkey
      - CAMO_ROOT_URL=/
      - CAMO_ALLOW_IMAGE=true
    ports:
      - 8081:8081
```
