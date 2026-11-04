# syntax=docker/dockerfile:1.7

ARG NODE_VERSION=24
ARG RUST_VERSION=1.99
ARG XX_VERSION=1.10.0

FROM --platform=$BUILDPLATFORM docker.io/tonistiigi/xx:${XX_VERSION} AS xx

FROM --platform=$BUILDPLATFORM docker.io/library/node:${NODE_VERSION}-alpine AS dashboard

WORKDIR /build

COPY package.json package-lock.json ./

RUN npm ci --no-audit --no-fund

COPY vite.config.js ./
COPY project.inlang ./project.inlang
COPY messages ./messages
COPY ui ./ui

RUN npm run build

FROM --platform=$BUILDPLATFORM docker.io/library/rust:${RUST_VERSION}-alpine AS build

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
