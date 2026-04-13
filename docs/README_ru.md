# KEasyDitor

[![CI](https://github.com/BergaBruh/keasyditor/actions/workflows/ci.yml/badge.svg)](https://github.com/BergaBruh/keasyditor/actions/workflows/ci.yml)
[![Release](https://github.com/BergaBruh/keasyditor/actions/workflows/release.yml/badge.svg)](https://github.com/BergaBruh/keasyditor/releases/latest)

Визуальный редактор тем KDE Plasma - оформления окон [Klassy](https://github.com/paulmcauley/klassy) и тем виджетов [Kvantum](https://github.com/tsujan/Kvantum).

> English README: [README.md](../README.md)

---

## Возможности

- **Редактор Klassy** - живой предпросмотр оформления окон; настройка кнопок, заголовка, теней, анимаций и рамок с мгновенным результатом
- **Редактор Kvantum** - редактирование цветовой палитры, основных параметров, хаков совместимости, свойств виджетов и SVG-ресурсов
- **Живой предпросмотр** - canvas-рендер обновляется в реальном времени при изменении слайдеров и переключателей
- **Отмена / Повтор** - полная история изменений
- **Сохранить и применить** - запись конфига и перезагрузка движка одним кликом
- **Пресеты Klassy** - применение встроенных пресетов оформления
- **Выбор темы Kvantum** - переключение между установленными темами
- **Цвета из обоев** - извлечение акцентной палитры через [matugen](https://github.com/InioX/matugen) (опционально)
- **Локализация** - язык интерфейса определяется системной локалью; чтобы добавить новый язык, достаточно положить `.toml`-файл без перекомпиляции

---

## Требования

- Linux с KDE Plasma
- [Klassy](https://github.com/paulmcauley/klassy) - для редактора оформления окон
- [kvantum](https://github.com/tsujan/Kvantum) - для редактора тем виджетов
- [matugen](https://github.com/InioX/matugen) *(опционально)* - извлечение цветов из обоев

---

## Установка

### Из релиза

Скачайте пакет для своего дистрибутива из [последнего релиза](https://github.com/BergaBruh/keasyditor/releases/latest):

| Дистрибутив | Пакет |
|---|---|
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

### Из исходников

```bash
git clone https://github.com/BergaBruh/keasyditor.git
cd keasyditor
make install          # собирает release-бинарь и устанавливает в ~/.local
```

`make install` помещает бинарь в `~/.local/bin/keasyditor`. Убедитесь, что `~/.local/bin` есть в вашем `PATH`.

---

## Сборка

```bash
# Debug-сборка
cargo build -p keasyditor

# Release-сборка
cargo build --release -p keasyditor

# Запуск тестов
cargo test -p keasyditor-core

# Сборка пакетов дистрибутивов через Docker
bash packaging/build.sh                  # все дистрибутивы
bash packaging/build.sh debian           # только Debian
make package-fedora                      # только Fedora
```

Пакеты сохраняются в `build/packages/`.  
Для сборки пакетов требуется Docker с поддержкой BuildKit (Docker 23+ включает его по умолчанию).

---

## Расположение файлов

| Данные | Путь |
|---|---|
| Настройки (`auto_apply`) | `~/.config/keasyditor/settings.ini` |
| Последние файлы | `~/.cache/keasyditor/recent_files` |
| Конфиг Klassy | `~/.config/klassy/klassyrc` |
| Темы Kvantum | `~/.config/Kvantum/` |

