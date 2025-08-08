FROM alpine:latest AS builder
RUN apk upgrade -U && apk add alpine-sdk bash curl git && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
COPY . .
RUN cargo install --locked --path . --root ./out

FROM alpine:latest
WORKDIR /app
RUN apk upgrade --no-cache && apk add --no-cache jemalloc && \
    addgroup -g 10001 app && adduser -u 10001 -G app -h /app -D app

USER app:app
COPY --from=builder /app/out/bin/camo /app

EXPOSE 8081
HEALTHCHECK CMD wget -qO - http://localhost:8081/__heartbeat__ || exit 1
CMD ["/app/camo"]
