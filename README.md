# KEasyDitor

[![CI](https://github.com/BergaBruh/keasyditor/actions/workflows/ci.yml/badge.svg)](https://github.com/BergaBruh/keasyditor/actions/workflows/ci.yml)
[![Release](https://github.com/BergaBruh/keasyditor/actions/workflows/release.yml/badge.svg)](https://github.com/BergaBruh/keasyditor/releases/latest)

A visual editor for KDE Plasma themes - [Klassy](https://github.com/paulmcauley/klassy) window decorations and [Kvantum](https://github.com/tsujan/Kvantum) Qt widget themes.

> Russian README: [README_ru.md](docs/README_ru.md)

---

## Features

- **Klassy editor** - live preview of window decorations; edit buttons, titlebar, shadows, animations, and window outlines with instant visual feedback
- **Kvantum editor** - edit colour palette, general settings, compatibility hacks, per-widget properties, and SVG assets
- **Live preview** - canvas-rendered preview updates in real time as you adjust sliders and toggles
- **Undo / Redo** - full history for all edits
- **Save & Apply** - writes config and reloads the engine in one click
- **Preset picker** - apply bundled Klassy presets
- **Theme picker** - switch between installed Kvantum themes
- **Wallpaper colours** - extracts an accent palette from your wallpaper via [matugen](https://github.com/InioX/matugen) (optional)
- **Localisation** - UI language follows system locale; drop a `.toml` file to add a new language without recompiling

---

## Requirements

- Linux with KDE Plasma
- [Klassy](https://github.com/paulmcauley/klassy) - for the window decoration editor
- [kvantum](https://github.com/tsujan/Kvantum) - for the widget theme editor
- [matugen](https://github.com/InioX/matugen) *(optional)* - wallpaper colour extraction

---

## Installation

### From a release

Download the package for your distribution from the [latest release](https://github.com/BergaBruh/keasyditor/releases/latest):

| Distribution | Package |
| --- | --- |
| Debian 13 | `keasyditor_*_amd64.deb` |
| Ubuntu 24.04+ | `keasyditor_*_amd64.deb` |
| Fedora 40+ | `keasyditor-*.x86_64.rpm` |
| Arch Linux | `keasyditor-*.pkg.tar.zst` |

```bash
# Debian / Ubuntu
sudo dpkg -i keasyditor_*_amd64.deb

# Fedora
sudo rpm -i keasyditor-*.x86_64.rpm

# Arch Linux
sudo pacman -U keasyditor-*.pkg.tar.zst
```

### From source

```bash
git clone https://github.com/BergaBruh/keasyditor.git
cd keasyditor
make install          # builds release binary and installs to ~/.local
```

`make install` places the binary in `~/.local/bin/keasyditor`. Make sure `~/.local/bin` is in your `PATH`.

---

## Building

```bash
# Debug build
cargo build -p keasyditor

# Release build
cargo build --release -p keasyditor

# Run tests
cargo test -p keasyditor-core

# Build distribution packages via Docker
bash packaging/build.sh                  # all distros
bash packaging/build.sh debian           # Debian only
make package-fedora                      # Fedora only
```

Packages are written to `build/packages/`.  
Docker with BuildKit is required for package builds (Docker 23+ has it by default).

---

## File locations

| Data | Path |
| --- | --- |
| Settings (`auto_apply`) | `~/.config/keasyditor/settings.ini` |
| Recent files | `~/.cache/keasyditor/recent_files` |
| Klassy config | `~/.config/klassy/klassyrc` |
| Kvantum themes | `~/.config/Kvantum/` |

---

## Third-party assets

KEasyDitor bundles a few assets from upstream projects. Each is redistributed unmodified under its original license:

- **[KvFlat.svg](crates/keasyditor-core/assets/KvFlat.svg)** - base Kvantum template used as a fallback when no system Kvantum theme is installed. From [tsujan/Kvantum](https://github.com/tsujan/Kvantum) by Pedram Pourang, licensed under **LGPL-2.1-or-later**. See [KvFlat.LICENSE](crates/keasyditor-core/assets/KvFlat.LICENSE).
- **[JetBrains Mono](crates/keasyditor/assets/fonts/)** - UI font. From [JetBrains/JetBrainsMono](https://github.com/JetBrains/JetBrainsMono), licensed under the **SIL Open Font License 1.1**. See [OFL.txt](crates/keasyditor/assets/fonts/OFL.txt).
