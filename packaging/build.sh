#!/usr/bin/env bash
# Build packages for all supported distributions using Docker.
# Run from the project root: bash packaging/build.sh [debian|ubuntu|fedora|archlinux|appimage]
#
# Requires Docker with BuildKit support (Docker 23+ has it by default).
# Output packages are placed in build/packages/.
set -euo pipefail

VERSION="0.1.0"
OUT_DIR="build/packages"

DISTROS=(debian ubuntu fedora archlinux appimage)

# If an argument is given, build only that distro
if [[ $# -gt 0 ]]; then
    DISTROS=("$@")
fi

command -v docker >/dev/null || { echo "error: docker not found"; exit 1; }

mkdir -p "$OUT_DIR"

build_distro() {
    local distro="$1"
    local dockerfile="packaging/${distro}/Dockerfile"

    if [[ ! -f "$dockerfile" ]]; then
        echo "error: $dockerfile not found"
        return 1
    fi

    echo ""
    echo "══ Building for ${distro} ══"

    DOCKER_BUILDKIT=1 docker build \
        --build-arg "VERSION=${VERSION}" \
        --output "type=local,dest=${OUT_DIR}" \
        -f "$dockerfile" \
        .

    echo "✓ ${distro} done"
}

for distro in "${DISTROS[@]}"; do
    build_distro "$distro"
done

echo ""
echo "All packages written to ${OUT_DIR}/"
ls -lh "${OUT_DIR}/"
