# syntax=docker/dockerfile:1
# Produces: keasyditor-<version>-x86_64.AppImage
# Usage:
#   DOCKER_BUILDKIT=1 docker build \
#     --output type=local,dest=build/packages \
#     -f packaging/appimage/Dockerfile .

ARG VERSION=0.1.0

# ── Stage 1: compile + assemble AppImage ──────────────────────────────────────
# Single stage: dev packages already contain the runtime libs that linuxdeploy
# needs to discover and bundle, so a separate package stage is unnecessary.
FROM ubuntu:24.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
        curl ca-certificates \
        build-essential \
        pkg-config \
        file \
        libxkbcommon-dev \
        libwayland-dev \
        libfontconfig-dev \
        libegl-dev \
        libx11-dev \
        libxcb1-dev \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# linuxdeploy discovers shared-library deps, bundles them into the AppDir,
# and calls appimagetool to produce the final squashfs image.
# APPIMAGE_EXTRACT_AND_RUN lets every AppImage tool run without FUSE in Docker.
RUN curl -fsSL -o /usr/local/bin/linuxdeploy \
        "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage" \
    && chmod +x /usr/local/bin/linuxdeploy

WORKDIR /src

# Cache dependency downloads separately from source
COPY Cargo.toml Cargo.lock ./
COPY crates/keasyditor/Cargo.toml      crates/keasyditor/Cargo.toml
COPY crates/keasyditor-core/Cargo.toml crates/keasyditor-core/Cargo.toml
RUN mkdir -p crates/keasyditor/src crates/keasyditor-core/src \
    && echo 'fn main(){}' > crates/keasyditor/src/main.rs \
    && touch crates/keasyditor-core/src/lib.rs \
    && cargo fetch

COPY . .
RUN cargo build --release -p keasyditor

ARG VERSION
RUN set -e \
    && mkdir -p /out /tmp/build \
    && cp /src/crates/keasyditor/assets/icon.svg /tmp/keasyditor.svg \
    # appimagetool strictly validates the Desktop Entry "Version" field
    # against the spec versions (1.0/1.1/1.2/1.4/1.5); the project's
    # desktop file uses it for the app version, so strip the line entirely
    # on the copy — the field is optional per the Desktop Entry spec.
    && sed '/^Version=/d' /src/keasyditor.desktop > /tmp/keasyditor.desktop \
    && cd /tmp/build \
    && ARCH=x86_64 \
       APPIMAGE_EXTRACT_AND_RUN=1 \
       linuxdeploy \
           --appdir /tmp/AppDir \
           --executable /src/target/release/keasyditor \
           --desktop-file /tmp/keasyditor.desktop \
           --icon-file /tmp/keasyditor.svg \
           --output appimage \
    && mv /tmp/build/*.AppImage "/out/keasyditor-${VERSION}-x86_64.AppImage"

# ── Stage 2: export ───────────────────────────────────────────────────────────
FROM scratch AS export
COPY --from=builder /out/ /
