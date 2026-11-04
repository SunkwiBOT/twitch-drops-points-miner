# syntax=docker/dockerfile:1.7

FROM --platform=$BUILDPLATFORM docker.io/tonistiigi/xx:1.10.0@sha256:40fe21f44bb8cddf1eb1226db190d0a984118f62ee02a91b77d99448d28264a6 AS xx

FROM --platform=$BUILDPLATFORM docker.io/library/node:24-alpine@sha256:8a32c8cf649a49915a66e81e7a609223c708d0adabf288ee3f4077d765542f85 AS dashboard

WORKDIR /build

COPY package.json package-lock.json ./

RUN npm ci --no-audit --no-fund

COPY vite.config.js ./
COPY project.inlang ./project.inlang
COPY messages ./messages
COPY ui ./ui

RUN npm run build

FROM --platform=$BUILDPLATFORM docker.io/library/rust:1.99-alpine@sha256:27cd9befd4a05d6f6eb407c3e28365f5092f008bfc5946207cca76ae5179ef23 AS build

COPY --from=xx / /

RUN apk add --no-cache \
    clang \
    lld \
    cmake \
    make \
    perl

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY messages ./messages
COPY miner-core ./miner-core
COPY miner-server ./miner-server
COPY src-tauri/Cargo.toml ./src-tauri/Cargo.toml

# ! The desktop crate is a workspace member: an empty stand-in is enough here.
RUN mkdir -p src-tauri/src \
    && echo 'fn main() {}' > src-tauri/src/main.rs \
    && echo 'fn main() {}' > src-tauri/build.rs

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch --locked

COPY --from=dashboard /build/ui/dist ./ui/dist

ARG TARGETPLATFORM

RUN xx-apk add --no-cache \
    musl-dev \
    gcc

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target,id=target-$TARGETPLATFORM \
    xx-cargo build --release --locked -p miner-server \
    && xx-verify --static "target/$(xx-cargo --print-target-triple)/release/twitch-miner" \
    && cp "target/$(xx-cargo --print-target-triple)/release/twitch-miner" /twitch-miner

FROM scratch AS runner

LABEL org.opencontainers.image.source="https://github.com/SunkwiBOT/twitch-drops-points-miner" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.title="Twitch Drops & Points Miner"

COPY --from=build /twitch-miner /twitch-miner

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

COPY --from=build --chown=65532:65532 /build/miner-server/empty /data

ENV HOST=0.0.0.0 \
    PORT=8080 \
    DATA_DIR=/data

USER 65532:65532

EXPOSE 8080

VOLUME ["/data"]

HEALTHCHECK --interval=60s --timeout=5s --start-period=15s --retries=3 \
    CMD ["/twitch-miner", "healthcheck"]

ENTRYPOINT ["/twitch-miner"]
