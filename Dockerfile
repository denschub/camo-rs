FROM debian:bullseye as builder
RUN apt-get update && \
    apt-get install -y build-essential curl git && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain 1.70
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
COPY . .
RUN cargo install --locked --path . --root ./out

FROM debian:bullseye-slim
WORKDIR /app
RUN \
    groupadd --gid 10001 app && \
    useradd --uid 10001 --gid 10001 --home /app --create-home app && \
    apt-get update && apt-get -y dist-upgrade && \
    apt-get install -y curl && apt-get clean && \
    rm -rf /var/lib/apt/lists/*

USER app:app
COPY --from=builder /app/out/bin/camo /app

EXPOSE 8081
HEALTHCHECK CMD curl -f http://localhost:8081/__heartbeat__ || exit 1
CMD ["/app/camo"]
