# syntax=docker/dockerfile:1
# Produces: keasyditor-<version>-1.fc40.x86_64.rpm  (Fedora 40 and newer)
# Usage:
#   DOCKER_BUILDKIT=1 docker build \
#     --output type=local,dest=build/packages \
#     -f packaging/fedora/Dockerfile .

ARG VERSION=0.1.0

# ── Stage 1: compile ─────────────────────────────────────────────────────────
FROM fedora:40 AS builder

RUN dnf install -y \
        gcc \
        pkg-config \
        libxkbcommon-devel \
        wayland-devel \
        fontconfig-devel \
        mesa-libEGL-devel \
        libX11-devel \
        libxcb-devel \
        curl \
    && dnf clean all

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable --profile minimal
ENV PATH="/root/.cargo/bin:$PATH"

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

# ── Stage 2: assemble .rpm ────────────────────────────────────────────────────
FROM fedora:40 AS package

ARG VERSION

RUN dnf install -y rpm-build && dnf clean all

# Place pre-built artifacts where the spec's %install can find them
RUN mkdir -p /root/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS,BUILDROOT} /out

COPY --from=builder /src/target/release/keasyditor          /root/rpmbuild/BUILD/keasyditor
COPY --from=builder /src/keasyditor.desktop                 /root/rpmbuild/BUILD/keasyditor.desktop
COPY --from=builder /src/crates/keasyditor/assets/icon.svg  /root/rpmbuild/BUILD/keasyditor.svg
COPY packaging/fedora/keasyditor.spec                       /root/rpmbuild/SPECS/keasyditor.spec

RUN rpmbuild -bb /root/rpmbuild/SPECS/keasyditor.spec \
    && find /root/rpmbuild/RPMS -name '*.rpm' -exec cp {} /out/ \;

# ── Stage 3: export ───────────────────────────────────────────────────────────
FROM scratch AS export
COPY --from=package /out/ /
