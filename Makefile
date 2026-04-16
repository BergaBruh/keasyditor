BIN_DIR     := $(HOME)/.local/bin
APP_DIR     := $(HOME)/.local/share/applications
ICON_DIR    := $(HOME)/.local/share/icons/hicolor/scalable/apps
BINARY      := target/release/keasyditor

.PHONY: build install uninstall package package-debian package-ubuntu package-fedora package-archlinux package-appimage

build:
	cargo build --release -p keasyditor

install: build
	install -Dm755 $(BINARY) $(BIN_DIR)/keasyditor
	install -Dm644 keasyditor.desktop $(APP_DIR)/keasyditor.desktop
	install -Dm644 crates/keasyditor/assets/icon.svg $(ICON_DIR)/keasyditor.svg
	@gtk-update-icon-cache -f -t $(HOME)/.local/share/icons/hicolor 2>/dev/null || true
	@echo "Installed to $(BIN_DIR)/keasyditor"
	@echo "Make sure $(BIN_DIR) is in your PATH"

uninstall:
	rm -f $(BIN_DIR)/keasyditor
	rm -f $(APP_DIR)/keasyditor.desktop
	rm -f $(ICON_DIR)/keasyditor.svg
	@gtk-update-icon-cache -f -t $(HOME)/.local/share/icons/hicolor 2>/dev/null || true

# ── Docker packaging ──────────────────────────────────────────────────────────

package:
	bash packaging/build.sh

package-debian:
	bash packaging/build.sh debian

package-ubuntu:
	bash packaging/build.sh ubuntu

package-fedora:
	bash packaging/build.sh fedora

package-archlinux:
	bash packaging/build.sh archlinux

package-appimage:
	bash packaging/build.sh appimage
