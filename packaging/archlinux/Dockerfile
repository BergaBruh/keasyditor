# syntax=docker/dockerfile:1
# Produces: keasyditor-<version>-1-x86_64.pkg.tar.zst  (Arch Linux)
# Usage:
#   DOCKER_BUILDKIT=1 docker build \
#     --output type=local,dest=build/packages \
#     -f packaging/archlinux/Dockerfile .

ARG VERSION=0.1.0

# ── Stage 1: compile ─────────────────────────────────────────────────────────
FROM archlinux:latest AS builder

RUN pacman -Syu --noconfirm \
    && pacman -S --noconfirm \
        base-devel \
        rust \
        cargo \
        pkg-config \
        libxkbcommon \
        wayland \
        fontconfig \
        mesa \
        libx11 \
        libxcb \
    && pacman -Scc --noconfirm

WORKDIR /src

COPY Cargo.toml Cargo.lock ./
COPY crates/keasyditor/Cargo.toml      crates/keasyditor/Cargo.toml
COPY crates/keasyditor-core/Cargo.toml crates/keasyditor-core/Cargo.toml
RUN mkdir -p crates/keasyditor/src crates/keasyditor-core/src \
    && echo 'fn main(){}' > crates/keasyditor/src/main.rs \
    && touch crates/keasyditor-core/src/lib.rs \
    && cargo fetch

COPY . .
RUN cargo build --release -p keasyditor

# ── Stage 2: assemble .pkg.tar.zst ───────────────────────────────────────────
FROM archlinux:latest AS package

ARG VERSION

# makepkg must run as non-root
RUN pacman -Syu --noconfirm \
    && pacman -S --noconfirm base-devel binutils \
    && pacman -Scc --noconfirm \
    && useradd -m builder

RUN mkdir -p /out && chown builder:builder /out

# Copy PKGBUILD and pre-built artifacts into builder's home
COPY --chown=builder:builder packaging/archlinux/PKGBUILD          /home/builder/PKGBUILD
COPY --from=builder --chown=builder:builder \
     /src/target/release/keasyditor                                 /home/builder/keasyditor
COPY --from=builder --chown=builder:builder \
     /src/keasyditor.desktop                                        /home/builder/keasyditor.desktop
COPY --from=builder --chown=builder:builder \
     /src/crates/keasyditor/assets/icon.svg                        /home/builder/keasyditor.svg

USER builder
WORKDIR /home/builder

# --nodeps: deps already present; --nocheck: no test suite
RUN makepkg --nodeps --nocheck --noconfirm

USER root
RUN cp /home/builder/*.pkg.tar.zst /out/

# ── Stage 3: export ───────────────────────────────────────────────────────────
FROM scratch AS export
COPY --from=package /out/ /
