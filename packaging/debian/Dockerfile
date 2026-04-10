# syntax=docker/dockerfile:1
# Produces: keasyditor_<version>_amd64.deb  (Debian 11 Bullseye and newer)
# Usage:
#   DOCKER_BUILDKIT=1 docker build \
#     --output type=local,dest=build/packages \
#     -f packaging/debian/Dockerfile .

ARG VERSION=0.1.0

# ── Stage 1: compile ─────────────────────────────────────────────────────────
FROM rust:slim-bullseye AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config \
        libxkbcommon-dev \
        libwayland-dev \
        libfontconfig-dev \
        libegl-dev \
        libx11-dev \
        libxcb1-dev \
    && rm -rf /var/lib/apt/lists/*

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

# ── Stage 2: assemble .deb ────────────────────────────────────────────────────
FROM debian:11-slim AS package

ARG VERSION

RUN apt-get update && apt-get install -y --no-install-recommends dpkg-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /src/target/release/keasyditor          /artifacts/keasyditor
COPY --from=builder /src/keasyditor.desktop                 /artifacts/keasyditor.desktop
COPY --from=builder /src/crates/keasyditor/assets/icon.svg  /artifacts/keasyditor.svg

RUN set -e \
    && mkdir -p \
        /pkg/DEBIAN \
        /pkg/usr/bin \
        /pkg/usr/share/applications \
        /pkg/usr/share/icons/hicolor/scalable/apps \
        /out \
    && install -m755 /artifacts/keasyditor       /pkg/usr/bin/keasyditor \
    && install -m644 /artifacts/keasyditor.desktop \
                     /pkg/usr/share/applications/keasyditor.desktop \
    && install -m644 /artifacts/keasyditor.svg   \
                     /pkg/usr/share/icons/hicolor/scalable/apps/keasyditor.svg \
    && INSTALLED_SIZE=$(du -sk /pkg/usr | cut -f1) \
    && printf 'Package: keasyditor\nVersion: %s\nArchitecture: amd64\nInstalled-Size: %s\nMaintainer: KEasyDitor contributors\nSection: kde\nPriority: optional\nHomepage: https://github.com/BergaBruh/keasyditor\nDepends: libxkbcommon0, libwayland-client0, libfontconfig1, libgl1-mesa-glx | libgl1, libegl1-mesa | libegl1\nRecommends: xdg-desktop-portal-kde | xdg-desktop-portal-gnome\nDescription: Visual editor for Klassy and Kvantum KDE themes\n KEasyDitor provides a graphical interface for editing KDE Plasma themes.\n .\n Supports the Klassy window decoration engine (klassyrc) and the Kvantum\n Qt widget theme engine (.kvconfig + SVG). Features live preview, undo/redo,\n and colour palette extraction via matugen.\n' \
        "${VERSION}" "${INSTALLED_SIZE}" > /pkg/DEBIAN/control \
    && printf '#!/bin/sh\nset -e\nif command -v gtk-update-icon-cache >/dev/null 2>&1; then\n    gtk-update-icon-cache -f -t /usr/share/icons/hicolor || true\nfi\n' \
        > /pkg/DEBIAN/postinst \
    && chmod 755 /pkg/DEBIAN/postinst \
    && cp /pkg/DEBIAN/postinst /pkg/DEBIAN/postrm \
    && dpkg-deb --root-owner-group --build /pkg \
        "/out/keasyditor_${VERSION}_amd64.deb"

# ── Stage 3: export ───────────────────────────────────────────────────────────
FROM scratch AS export
COPY --from=package /out/ /
