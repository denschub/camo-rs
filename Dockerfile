FROM rust:1.66-bullseye as builder
WORKDIR /app
COPY . .
RUN cargo install --locked --path . --root ./out

FROM debian:bullseye-slim
WORKDIR /app
RUN \
    groupadd --gid 10001 app && \
    useradd --uid 10001 --gid 10001 --home /app --create-home app && \
    apt-get update && apt-get -y dist-upgrade && \
    apt-get install -y curl libjemalloc2 && apt-get clean && \
    rm -rf /var/lib/apt/lists/*

USER app:app
COPY --from=builder /app/out/bin/camo /app
COPY ./docker-entrypoint.sh /app

EXPOSE 8081
HEALTHCHECK CMD curl -f http://localhost:8081/__heartbeat__ || exit 1
ENTRYPOINT ["/app/docker-entrypoint.sh"]
CMD ["/app/camo"]
