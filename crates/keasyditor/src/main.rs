mod i18n;
mod message;
mod theme;
mod ui;

use std::collections::{HashMap, HashSet};

use iced::{Element, Task, Theme};
use message::{KlassyMessage, KvantumMessage, Message, Page, SettingsMessage, SvgMessage};

use keasyditor_core::models::klassy::KlassyConfig;
use keasyditor_core::models::kvantum::KvantumConfig;
use keasyditor_core::undo_redo::UndoRedoStack;

fn vec_to_map(v: &Option<Vec<(String, String)>>) -> HashMap<String, String> {
    v.as_ref()
        .map(|list| list.iter().cloned().collect())
        .unwrap_or_default()
}

fn app_theme(_state: &App) -> Theme {
    Theme::custom(
        "KEasyDitor Dark".to_string(),
        iced::theme::Palette {
            background: theme::BG,
            text: theme::TEXT_ON,
            primary: theme::AMBER,
            success: theme::GREEN,
            danger: iced::Color::from_rgb(0.8, 0.2, 0.2),
            warning: iced::Color::from_rgb(0.9, 0.7, 0.2),
        },
    )
}

fn main() -> iced::Result {
    i18n::init();
    iced::application(App::new, App::update, App::view)
        .title("KEasyDitor")
        .theme(app_theme)
        .subscription(App::subscription)
        .default_font(theme::MONO)
        .font(include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf").as_slice())
        .font(include_bytes!("../assets/fonts/JetBrainsMono-Bold.ttf").as_slice())
        .window(iced::window::Settings {
            size: iced::Size::new(1440.0, 900.0),
            min_size: Some(iced::Size::new(960.0, 600.0)),
            ..Default::default()
        })
        .run()
}

struct App {
    page: Page,
    // Klassy
    klassy_config: Option<KlassyConfig>,
    klassy_original: Option<KlassyConfig>,
    klassy_undo: Option<UndoRedoStack<KlassyConfig>>,
    klassy_file_path: Option<String>,
    klassy_tab: usize,
    klassy_expanded: HashSet<String>,
    klassy_loading: bool,
    klassy_error: Option<String>,
    // Kvantum
    kvantum_config: Option<KvantumConfig>,
    kvantum_original: Option<KvantumConfig>,
    kvantum_undo: Option<UndoRedoStack<KvantumConfig>>,
    kvantum_dir_path: Option<String>,
    kvantum_theme_name: Option<String>,
    kvantum_svg_content: Option<String>,
    kvantum_tab: usize,
    kvantum_expanded: HashSet<String>,
    kvantum_loading: bool,
    kvantum_error: Option<String>,
    // Settings
    auto_apply: bool,
    reload_status: Option<String>,
    matugen_loading: bool,
    matugen_dark_palette: Option<Vec<(String, String)>>,
    matugen_light_palette: Option<Vec<(String, String)>>,
    prefer_dark_palette: bool,
    show_apply_to_system_confirm: bool,
    apply_to_system_status: Option<(bool, Vec<String>)>,
    // Availability
    klassy_installed: bool,
    kvantum_installed: bool,
    matugen_installed: bool,
    // Home
    recent_files: Vec<String>,
    installed_themes: Vec<(String, bool)>,
    active_kvantum_theme: Option<String>,
    // Pickers
    show_klassy_presets: bool,
    klassy_preset_names: Vec<String>,
    show_kvantum_themes: bool,
    kvantum_discovered_themes: Vec<(String, String, bool)>,
    // UI state for widgets
    slider_values: HashMap<String, f32>,
    toggle_values: HashMap<String, bool>,
    text_input_values: HashMap<String, String>,
    // Search field for pickers
    search_text: String,
    // UI feedback
    save_flash: bool,
}

impl App {
    fn klassy_dirty(&self) -> bool {
        match (&self.klassy_config, &self.klassy_original) {
            (Some(c), Some(o)) => c != o,
            _ => false,
        }
    }

    fn kvantum_dirty(&self) -> bool {
        match (&self.kvantum_config, &self.kvantum_original) {
            (Some(c), Some(o)) => c != o,
            _ => false,
        }
    }

    fn klassy_can_undo(&self) -> bool {
        self.klassy_undo.as_ref().is_some_and(|u| u.can_undo())
    }

    fn klassy_can_redo(&self) -> bool {
        self.klassy_undo.as_ref().is_some_and(|u| u.can_redo())
    }

    fn kvantum_can_undo(&self) -> bool {
        self.kvantum_undo.as_ref().is_some_and(|u| u.can_undo())
    }

    fn kvantum_can_redo(&self) -> bool {
        self.kvantum_undo.as_ref().is_some_and(|u| u.can_redo())
    }
}

impl App {
    fn new() -> (Self, Task<Message>) {
        // Pre-expand some sections so the UI isn't all collapsed on first view
        let mut klassy_expanded = HashSet::new();
        klassy_expanded.insert("klassy.buttons.shape".to_string());
        klassy_expanded.insert("klassy.titlebar.opacity".to_string());
        klassy_expanded.insert("klassy.window.corners".to_string());
        klassy_expanded.insert("klassy.shadows.style".to_string());
        klassy_expanded.insert("klassy.animations.settings".to_string());
        klassy_expanded.insert("klassy.advanced.general".to_string());

        let mut kvantum_expanded = HashSet::new();
        kvantum_expanded.insert("kvantum.colors.window".to_string());
        kvantum_expanded.insert("kvantum.general.appearance".to_string());
        kvantum_expanded.insert("kvantum.hacks.compat".to_string());
        kvantum_expanded.insert("kvantum.widgets.pushbutton".to_string());

        // Default slider values
        let mut slider_values = HashMap::new();
        slider_values.insert("klassy.corner_radius".to_string(), 10.0);
        slider_values.insert("klassy.shadow_strength".to_string(), 128.0);
        slider_values.insert("klassy.titlebar_opacity".to_string(), 100.0);
        slider_values.insert("klassy.titlebar_opacity_inactive".to_string(), 100.0);
        slider_values.insert("klassy.bg_opacity_active".to_string(), 100.0);
        slider_values.insert("klassy.bg_opacity_inactive".to_string(), 100.0);

        (
            Self {
                page: Page::Home,
                // Klassy
                klassy_config: None,
                klassy_original: None,
                klassy_undo: None,
                klassy_file_path: None,
                klassy_tab: 0,
                klassy_expanded,
                klassy_loading: false,
                klassy_error: None,
                // Kvantum
                kvantum_config: None,
                kvantum_original: None,
                kvantum_undo: None,
                kvantum_dir_path: None,
                kvantum_theme_name: None,
                kvantum_svg_content: None,
                kvantum_tab: 0,
                kvantum_expanded,
                kvantum_loading: false,
                kvantum_error: None,
                // Settings
                auto_apply: false,
                reload_status: None,
                matugen_loading: false,
                matugen_dark_palette: None,
                matugen_light_palette: None,
                prefer_dark_palette: true,
                show_apply_to_system_confirm: false,
                apply_to_system_status: None,
                // Availability — detect synchronously (fast `which` calls)
                klassy_installed: std::process::Command::new("which")
                    .arg("klassy-settings")
                    .output()
                    .is_ok_and(|o| o.status.success()),
                kvantum_installed: std::process::Command::new("which")
                    .arg("kvantummanager")
                    .output()
                    .is_ok_and(|o| o.status.success()),
                matugen_installed: std::process::Command::new("which")
                    .arg("matugen")
                    .output()
                    .is_ok_and(|o| o.status.success()),
                // Home
                recent_files: Vec::new(),
                installed_themes: Vec::new(),
                active_kvantum_theme: {
                    let svc = keasyditor_core::services::KvantumService::new(
                        keasyditor_core::services::FileService::new(),
                    );
                    svc.get_active_theme()
                },
                // Pickers
                show_klassy_presets: false,
                klassy_preset_names: Vec::new(),
                show_kvantum_themes: false,
                kvantum_discovered_themes: Vec::new(),
                // UI
                slider_values,
                toggle_values: HashMap::new(),
                text_input_values: HashMap::new(),
                search_text: String::new(),
                save_flash: false,
            },
            Task::batch([
                // Discover installed themes on startup
                Task::perform(
                    async {
                        let file_svc = keasyditor_core::services::FileService::new();
                        let discovery = keasyditor_core::services::ThemeDiscoveryService::new(file_svc);
                        let themes = discovery.discover_kvantum_themes();
                        themes
                            .into_iter()
                            .map(|t| (t.name, t.path, t.is_system))
                            .collect::<Vec<_>>()
                    },
                    |themes| Message::Kvantum(KvantumMessage::ThemesDiscovered(themes)),
                ),
                // Load persisted app settings
                Task::perform(
                    async {
                        let path = keasyditor_core::constants::keasyditor_settings_path();
                        tokio::fs::read_to_string(&path)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    |result| Message::Settings(SettingsMessage::AppSettingsLoaded(result)),
                ),
                // Load recent files from cache
                Task::perform(
                    async {
                        let path = keasyditor_core::constants::keasyditor_recent_files_path();
                        match tokio::fs::read_to_string(&path).await {
                            Ok(content) => content
                                .lines()
                                .filter(|l| !l.is_empty())
                                .map(String::from)
                                .collect::<Vec<_>>(),
                            Err(_) => Vec::new(),
                        }
                    },
                    |files| Message::Settings(SettingsMessage::RecentFilesLoaded(files)),
                ),
                // Auto-extract wallpaper palette on startup
                Task::perform(
                    async {
                        use keasyditor_core::services::{MatugenService, MatugenPalette};
                        let wallpaper = MatugenService::detect_wallpaper();
                        let palette = wallpaper.and_then(|w| MatugenService::extract_palette(&w));
                        let is_dark = MatugenService::is_system_dark();
                        let extract = |colors: &std::collections::HashMap<String, String>| -> Vec<(String, String)> {
                            MatugenPalette::KEY_COLORS
                                .iter()
                                .filter_map(|key| colors.get(*key).map(|hex| (key.to_string(), hex.clone())))
                                .collect()
                        };
                        let (dark, light) = match &palette {
                            Some(p) => (Some(extract(&p.dark)), Some(extract(&p.light))),
                            None => (None, None),
                        };
                        (dark, light, is_dark)
                    },
                    |(dark, light, is_dark)| Message::Settings(SettingsMessage::WallpaperPaletteResult { dark, light, is_dark }),
                ),
            ]),
        )
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::listen().map(|event| {
            if let iced::keyboard::Event::KeyPressed { key, modifiers, .. } = event
                && modifiers.command() {
                    match key {
                        iced::keyboard::Key::Character(ref c) if c == "z" && !modifiers.shift() => {
                            return Message::UndoCurrentPage;
                        }
                        iced::keyboard::Key::Character(ref c) if c == "z" && modifiers.shift() => {
                            return Message::RedoCurrentPage;
                        }
                        iced::keyboard::Key::Character(ref c) if c == "y" => {
                            return Message::RedoCurrentPage;
                        }
                        iced::keyboard::Key::Character(ref c) if c == "s" => {
                            return Message::SaveCurrentPage;
                        }
                        _ => {}
                    }
                }
            Message::Noop
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavigateTo(page) => {
                self.page = page;
                match page {
                    Page::Klassy if self.klassy_config.is_none() && !self.klassy_loading => {
                        return self.update(Message::Klassy(KlassyMessage::LoadDefault));
                    }
                    Page::Kvantum if self.kvantum_config.is_none() && !self.kvantum_loading => {
                        return self.update(Message::Kvantum(KvantumMessage::LoadActiveTheme));
                    }
                    _ => {}
                }
            }

            // -- Keyboard shortcut dispatchers --
            Message::UndoCurrentPage => match self.page {
                Page::Klassy => return self.update(Message::Klassy(KlassyMessage::Undo)),
                Page::Kvantum => return self.update(Message::Kvantum(KvantumMessage::Undo)),
                _ => {}
            },
            Message::RedoCurrentPage => match self.page {
                Page::Klassy => return self.update(Message::Klassy(KlassyMessage::Redo)),
                Page::Kvantum => return self.update(Message::Kvantum(KvantumMessage::Redo)),
                _ => {}
            },
            Message::SaveCurrentPage => match self.page {
                Page::Klassy => return self.update(Message::Klassy(KlassyMessage::Save)),
                Page::Kvantum => return self.update(Message::Kvantum(KvantumMessage::Save)),
                _ => {}
            },

            // -- Klassy messages --
            Message::Klassy(msg) => match msg {
                KlassyMessage::TabChanged(tab) => {
                    self.klassy_tab = tab;
                }
                KlassyMessage::LoadDefault => {
                    let path = keasyditor_core::constants::klassy_config_path();
                    if path.exists() {
                        self.klassy_loading = true;
                        self.klassy_error = None;
                        let path_str = path.to_string_lossy().to_string();
                        return Task::perform(
                            async move {
                                std::fs::read_to_string(&path_str)
                                    .map_err(|e| e.to_string())
                            },
                            |result| Message::Klassy(KlassyMessage::ConfigLoaded(result)),
                        );
                    } else {
                        self.klassy_error =
                            Some("Klassy config file not found at default path.".to_string());
                    }
                }
                KlassyMessage::LoadFile(path) => {
                    self.klassy_loading = true;
                    self.klassy_error = None;
                    let path_str = path.to_string_lossy().to_string();
                    self.klassy_file_path = Some(path_str.clone());
                    return Task::perform(
                        async move {
                            std::fs::read_to_string(&path_str).map_err(|e| e.to_string())
                        },
                        |result| Message::Klassy(KlassyMessage::ConfigLoaded(result)),
                    );
                }
                KlassyMessage::ConfigLoaded(Ok(content)) => {
                    let doc = keasyditor_core::ini::parse_ini(&content);
                    let config = KlassyConfig::from_ini(&doc);
                    self.klassy_original = Some(config.clone());
                    self.klassy_undo = Some(UndoRedoStack::new(config.clone()));
                    self.klassy_config = Some(config);
                    self.klassy_loading = false;
                    self.klassy_error = None;
                    // Set file path if loading from default
                    if self.klassy_file_path.is_none() {
                        self.klassy_file_path = Some(
                            keasyditor_core::constants::klassy_config_path()
                                .to_string_lossy()
                                .to_string(),
                        );
                    }
                    // Sync slider/toggle values from loaded config
                    self.sync_klassy_ui_from_config();
                    // Track as recent file and persist
                    if let Some(ref path) = self.klassy_file_path {
                        self.add_recent_file(path.clone());
                        return self.save_recent_files();
                    }
                }
                KlassyMessage::ConfigLoaded(Err(err)) => {
                    self.klassy_loading = false;
                    self.klassy_error = Some(err);
                }
                KlassyMessage::Save | KlassyMessage::SaveAndApply => {
                    if let (Some(config), Some(path)) =
                        (&self.klassy_config, &self.klassy_file_path)
                    {
                        let doc = config.to_ini();
                        let content = keasyditor_core::ini::serialize_ini(&doc);
                        let path = path.clone();
                        let apply_after = matches!(msg, KlassyMessage::SaveAndApply);
                        return Task::perform(
                            async move {
                                std::fs::write(&path, &content).map_err(|e| e.to_string())
                            },
                            move |result| {
                                if apply_after {
                                    Message::Klassy(KlassyMessage::SavedThenApply(result))
                                } else {
                                    Message::Klassy(KlassyMessage::Saved(result))
                                }
                            },
                        );
                    }
                }
                KlassyMessage::Saved(Ok(())) => {
                    self.klassy_original = self.klassy_config.clone();
                    self.save_flash = true;
                    let flash = Task::perform(
                        tokio::time::sleep(std::time::Duration::from_secs(1)),
                        |_| Message::SaveFlashDone,
                    );
                    if self.auto_apply {
                        return Task::batch([self.update(Message::Klassy(KlassyMessage::Apply)), flash]);
                    }
                    return flash;
                }
                KlassyMessage::SavedThenApply(Ok(())) => {
                    self.klassy_original = self.klassy_config.clone();
                    self.save_flash = true;
                    let flash = Task::perform(
                        tokio::time::sleep(std::time::Duration::from_secs(1)),
                        |_| Message::SaveFlashDone,
                    );
                    return Task::batch([self.update(Message::Klassy(KlassyMessage::Apply)), flash]);
                }
                KlassyMessage::SavedThenApply(Err(err)) => {
                    self.klassy_error = Some(format!("Save failed: {}", err));
                }
                KlassyMessage::Saved(Err(err)) => {
                    self.klassy_error = Some(format!("Save failed: {}", err));
                }
                KlassyMessage::UpdateKey {
                    section,
                    key,
                    value,
                } => {
                    if let Some(config) = &mut self.klassy_config {
                        // Push current state to undo stack before modifying
                        if let Some(undo) = &mut self.klassy_undo {
                            undo.push(config.clone());
                        }
                        config.set_value(&section, &key, value);
                    }
                }
                KlassyMessage::Undo => {
                    if let Some(undo) = &mut self.klassy_undo
                        && let Some(prev) = undo.undo() {
                            self.klassy_config = Some(prev.clone());
                            self.sync_klassy_ui_from_config();
                        }
                }
                KlassyMessage::Redo => {
                    if let Some(undo) = &mut self.klassy_undo
                        && let Some(next) = undo.redo() {
                            self.klassy_config = Some(next.clone());
                            self.sync_klassy_ui_from_config();
                        }
                }
                KlassyMessage::Apply => {
                    return Task::perform(
                        async {
                            let svc = keasyditor_core::services::ProcessService::new();
                            match svc.reconfigure_kwin() {
                                Ok(result) => {
                                    if result.is_success() {
                                        Ok(result.stdout)
                                    } else {
                                        Err(result.stderr)
                                    }
                                }
                                Err(e) => Err(e.to_string()),
                            }
                        },
                        |result| Message::Klassy(KlassyMessage::Applied(result)),
                    );
                }
                KlassyMessage::Applied(Ok(_)) => {}
                KlassyMessage::Applied(Err(err)) => {
                    self.klassy_error = Some(format!("Apply failed: {}", err));
                }
                KlassyMessage::OpenFilePicker => {
                    return Task::perform(
                        async {
                            rfd::AsyncFileDialog::new()
                                .set_title("Open Klassy config")
                                .pick_file()
                                .await
                                .map(|f| f.path().to_path_buf())
                        },
                        |result| match result {
                            Some(path) => Message::Klassy(KlassyMessage::LoadFile(path)),
                            None => Message::Noop,
                        },
                    );
                }
                KlassyMessage::ShowPresetPicker => {
                    self.show_klassy_presets = true;
                    // Load preset names if not already loaded
                    if self.klassy_preset_names.is_empty() {
                        return Task::perform(
                            async {
                                let file_svc =
                                    keasyditor_core::services::FileService::new();
                                let discovery =
                                    keasyditor_core::services::ThemeDiscoveryService::new(
                                        file_svc,
                                    );
                                discovery.discover_klassy_presets()
                            },
                            |presets| Message::Klassy(KlassyMessage::PresetsLoaded(presets)),
                        );
                    }
                }
                KlassyMessage::HidePresetPicker => {
                    self.show_klassy_presets = false;
                }
                KlassyMessage::ApplyPreset(name) => {
                    self.show_klassy_presets = false;
                    let preset_name = name.clone();
                    return Task::perform(
                        async move {
                            let svc = keasyditor_core::services::ProcessService::new();
                            match svc.apply_klassy_preset(&preset_name) {
                                Ok(result) => {
                                    if result.is_success() {
                                        Ok(result.stdout)
                                    } else {
                                        Err(result.stderr)
                                    }
                                }
                                Err(e) => Err(e.to_string()),
                            }
                        },
                        |result| Message::Klassy(KlassyMessage::Applied(result)),
                    );
                }
                KlassyMessage::PresetsLoaded(presets) => {
                    self.klassy_preset_names = presets;
                }
            },

            // -- Kvantum messages --
            Message::Kvantum(msg) => match msg {
                KvantumMessage::TabChanged(tab) => {
                    self.kvantum_tab = tab;
                }
                KvantumMessage::LoadActiveTheme => {
                    self.kvantum_loading = true;
                    self.kvantum_error = None;
                    return Task::perform(
                        async {
                            let file_svc = keasyditor_core::services::FileService::new();
                            let kv_svc =
                                keasyditor_core::services::KvantumService::new(file_svc);
                            let active_name = kv_svc.get_active_theme();
                            match active_name {
                                Some(name) => {
                                    // Try user dir first, then system dir
                                    let user_dir = format!(
                                        "{}/{}",
                                        keasyditor_core::constants::kvantum_config_dir()
                                            .to_string_lossy(),
                                        name
                                    );
                                    let system_dir = format!(
                                        "{}/{}",
                                        keasyditor_core::constants::kvantum_system_dir()
                                            .to_string_lossy(),
                                        name
                                    );
                                    let dir = if std::path::Path::new(&user_dir).exists() {
                                        user_dir
                                    } else {
                                        system_dir
                                    };
                                    // Return the dir path and theme name for sync loading
                                    Ok(format!("{}|||{}", dir, name))
                                }
                                None => Err("No active Kvantum theme found.".to_string()),
                            }
                        },
                        |result| Message::Kvantum(KvantumMessage::ThemeLoaded(result)),
                    );
                }
                KvantumMessage::LoadTheme(path) => {
                    self.kvantum_loading = true;
                    self.kvantum_error = None;
                    let dir_str = path.to_string_lossy().to_string();
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("theme")
                        .to_string();
                    return Task::perform(
                        async move { Ok(format!("{}|||{}", dir_str, name)) },
                        |result: Result<String, String>| {
                            Message::Kvantum(KvantumMessage::ThemeLoaded(result))
                        },
                    );
                }
                KvantumMessage::ThemeLoaded(Ok(info)) => {
                    let parts: Vec<&str> = info.splitn(2, "|||").collect();
                    if parts.len() == 2 {
                        let dir_path = parts[0].to_string();
                        let theme_name = parts[1].to_string();
                        let file_svc = keasyditor_core::services::FileService::new();
                        let kv_svc = keasyditor_core::services::KvantumService::new(file_svc);
                        match kv_svc.load_theme(&dir_path) {
                            Ok(data) => {
                                self.kvantum_original = Some(data.config.clone());
                                self.kvantum_undo =
                                    Some(UndoRedoStack::new(data.config.clone()));
                                self.kvantum_config = Some(data.config);
                                self.kvantum_svg_content = data.svg_content;
                                self.kvantum_dir_path = Some(dir_path.clone());
                                self.kvantum_theme_name = Some(theme_name);
                                self.kvantum_loading = false;
                                self.kvantum_error = None;
                                self.sync_kvantum_ui_from_config();
                                // Track as recent file and persist
                                self.add_recent_file(dir_path);
                                return self.save_recent_files();
                            }
                            Err(e) => {
                                self.kvantum_loading = false;
                                self.kvantum_error = Some(e.to_string());
                            }
                        }
                    } else {
                        self.kvantum_loading = false;
                        self.kvantum_error = Some("Failed to parse theme info".to_string());
                    }
                }
                KvantumMessage::ThemeLoaded(Err(err)) => {
                    self.kvantum_loading = false;
                    self.kvantum_error = Some(err);
                }
                KvantumMessage::Save | KvantumMessage::SaveAndApply => {
                    if let (Some(config), Some(dir_path)) =
                        (&self.kvantum_config, &self.kvantum_dir_path)
                    {
                        let theme_name = self
                            .kvantum_theme_name
                            .clone()
                            .unwrap_or_else(|| "theme".to_string());
                        let doc = config.to_ini();
                        let content = keasyditor_core::ini::serialize_ini(&doc);
                        let kvconfig_path =
                            format!("{}/{}.kvconfig", dir_path, theme_name);
                        let svg_path = format!("{}/{}.svg", dir_path, theme_name);
                        let svg_data = self.kvantum_svg_content.clone();
                        let apply_after = matches!(msg, KvantumMessage::SaveAndApply);
                        return Task::perform(
                            async move {
                                std::fs::write(&kvconfig_path, &content)
                                    .map_err(|e| e.to_string())?;
                                if let Some(svg) = svg_data {
                                    std::fs::write(&svg_path, &svg)
                                        .map_err(|e| e.to_string())?;
                                }
                                Ok(())
                            },
                            move |result| {
                                if apply_after {
                                    Message::Kvantum(KvantumMessage::SavedThenApply(result))
                                } else {
                                    Message::Kvantum(KvantumMessage::Saved(result))
                                }
                            },
                        );
                    }
                }
                KvantumMessage::Saved(Ok(())) => {
                    self.kvantum_original = self.kvantum_config.clone();
                    self.save_flash = true;
                    let flash = Task::perform(
                        tokio::time::sleep(std::time::Duration::from_secs(1)),
                        |_| Message::SaveFlashDone,
                    );
                    if self.auto_apply {
                        return Task::batch([self.update(Message::Kvantum(KvantumMessage::Apply)), flash]);
                    }
                    return flash;
                }
                KvantumMessage::SavedThenApply(Ok(())) => {
                    self.kvantum_original = self.kvantum_config.clone();
                    self.save_flash = true;
                    let flash = Task::perform(
                        tokio::time::sleep(std::time::Duration::from_secs(1)),
                        |_| Message::SaveFlashDone,
                    );
                    return Task::batch([self.update(Message::Kvantum(KvantumMessage::Apply)), flash]);
                }
                KvantumMessage::SavedThenApply(Err(err)) => {
                    self.kvantum_error = Some(format!("Save failed: {}", err));
                }
                KvantumMessage::Saved(Err(err)) => {
                    self.kvantum_error = Some(format!("Save failed: {}", err));
                }
                KvantumMessage::UpdateColor { key, value } => {
                    if let Some(config) = self.kvantum_config.take() {
                        if let Some(undo) = &mut self.kvantum_undo {
                            undo.push(config.clone());
                        }
                        let mut new_colors = config.colors.clone();
                        new_colors.set_value(&key, value);
                        self.kvantum_config = Some(config.copy_with_colors(new_colors));
                    }
                }
                KvantumMessage::UpdateGeneral { key, value } => {
                    if let Some(config) = self.kvantum_config.take() {
                        if let Some(undo) = &mut self.kvantum_undo {
                            undo.push(config.clone());
                        }
                        let mut new_general = config.general.clone();
                        new_general.set_value(&key, value);
                        self.kvantum_config = Some(config.copy_with_general(new_general));
                    }
                }
                KvantumMessage::UpdateSection {
                    section,
                    key,
                    value,
                } => {
                    if let Some(config) = self.kvantum_config.take() {
                        if let Some(undo) = &mut self.kvantum_undo {
                            undo.push(config.clone());
                        }
                        if section == "Hacks" {
                            let mut new_hacks = config.hacks.clone();
                            new_hacks.insert(key, value);
                            self.kvantum_config = Some(config.copy_with_hacks(new_hacks));
                        } else if let Some(sec) = config.get_section(&section) {
                            let mut new_sec = sec.clone();
                            new_sec.set_value(&key, value);
                            self.kvantum_config =
                                Some(config.copy_with_section(&section, new_sec));
                        } else {
                            // Put config back if section not found
                            self.kvantum_config = Some(config);
                        }
                    }
                }
                KvantumMessage::Undo => {
                    if let Some(undo) = &mut self.kvantum_undo
                        && let Some(prev) = undo.undo() {
                            self.kvantum_config = Some(prev.clone());
                            self.sync_kvantum_ui_from_config();
                        }
                }
                KvantumMessage::Redo => {
                    if let Some(undo) = &mut self.kvantum_undo
                        && let Some(next) = undo.redo() {
                            self.kvantum_config = Some(next.clone());
                            self.sync_kvantum_ui_from_config();
                        }
                }
                KvantumMessage::Apply => {
                    let theme_name = self.kvantum_theme_name.clone();
                    return Task::perform(
                        async move {
                            let svc = keasyditor_core::services::ProcessService::new();
                            if let Some(name) = theme_name {
                                match svc.apply_kvantum_theme(&name) {
                                    Ok(result) => {
                                        if result.is_success() {
                                            Ok(result.stdout)
                                        } else {
                                            Err(result.stderr)
                                        }
                                    }
                                    Err(e) => Err(e.to_string()),
                                }
                            } else {
                                Err("No theme name to apply.".to_string())
                            }
                        },
                        |result| Message::Kvantum(KvantumMessage::Applied(result)),
                    );
                }
                KvantumMessage::Applied(Ok(_)) => {}
                KvantumMessage::Applied(Err(err)) => {
                    self.kvantum_error = Some(format!("Apply failed: {}", err));
                }
                KvantumMessage::OpenThemePicker => {
                    return Task::perform(
                        async {
                            rfd::AsyncFileDialog::new()
                                .set_title("Open Kvantum theme directory")
                                .pick_folder()
                                .await
                                .map(|f| f.path().to_path_buf())
                        },
                        |result| match result {
                            Some(path) => Message::Kvantum(KvantumMessage::LoadTheme(path)),
                            None => Message::Noop,
                        },
                    );
                }
                KvantumMessage::ShowThemePicker => {
                    self.show_kvantum_themes = true;
                    // Refresh discovered themes when opening picker
                    return Task::perform(
                        async {
                            let file_svc = keasyditor_core::services::FileService::new();
                            let discovery =
                                keasyditor_core::services::ThemeDiscoveryService::new(file_svc);
                            let themes = discovery.discover_kvantum_themes();
                            themes
                                .into_iter()
                                .map(|t| (t.name, t.path, t.is_system))
                                .collect::<Vec<_>>()
                        },
                        |themes| Message::Kvantum(KvantumMessage::ThemesDiscovered(themes)),
                    );
                }
                KvantumMessage::HideThemePicker => {
                    self.show_kvantum_themes = false;
                }
                KvantumMessage::SelectTheme(name) => {
                    self.show_kvantum_themes = false;
                    // Look up the path from discovered themes
                    if let Some((_, path, _)) = self
                        .kvantum_discovered_themes
                        .iter()
                        .find(|(n, _, _)| n == &name)
                    {
                        let dir = path.clone();
                        return self.update(Message::Kvantum(KvantumMessage::LoadTheme(
                            std::path::PathBuf::from(dir),
                        )));
                    }
                }
                KvantumMessage::NewTheme => {
                    return Task::perform(
                        async {
                            let kvantum_dir = keasyditor_core::constants::kvantum_config_dir();
                            let _ = std::fs::create_dir_all(&kvantum_dir);
                            let file = rfd::AsyncFileDialog::new()
                                .set_title("Save new Kvantum theme")
                                .set_file_name("MyTheme.kvconfig")
                                .set_directory(&kvantum_dir)
                                .add_filter("Kvantum config", &["kvconfig"])
                                .save_file()
                                .await;
                            file.map(|f| f.path().to_string_lossy().to_string())
                        },
                        |path| Message::Kvantum(KvantumMessage::NewThemeCreated(path)),
                    );
                }
                KvantumMessage::NewThemeCreated(Some(path)) => {
                    // Always create themes under ~/.config/Kvantum/<name>/<name>.kvconfig
                    // regardless of where the dialog returned — only the file stem matters.
                    let name = std::path::Path::new(&path)
                        .file_stem()
                        .and_then(|n| n.to_str())
                        .unwrap_or("MyTheme")
                        .to_string();

                    let file_svc = keasyditor_core::services::FileService::new();
                    let kv_svc = keasyditor_core::services::KvantumService::new(file_svc);

                    match kv_svc.create_theme(&name) {
                        Ok(theme_dir_path) => {
                            self.page = Page::Kvantum;
                            let load_task = self.update(Message::Kvantum(
                                KvantumMessage::LoadTheme(std::path::PathBuf::from(
                                    theme_dir_path,
                                )),
                            ));
                            // Re-discover so the new theme appears in the picker
                            let discover_task = Task::perform(
                                async {
                                    let file_svc =
                                        keasyditor_core::services::FileService::new();
                                    let discovery =
                                        keasyditor_core::services::ThemeDiscoveryService::new(
                                            file_svc,
                                        );
                                    discovery
                                        .discover_kvantum_themes()
                                        .into_iter()
                                        .map(|t| (t.name, t.path, t.is_system))
                                        .collect::<Vec<_>>()
                                },
                                |themes| {
                                    Message::Kvantum(KvantumMessage::ThemesDiscovered(themes))
                                },
                            );
                            return Task::batch([load_task, discover_task]);
                        }
                        Err(e) => {
                            self.kvantum_error =
                                Some(format!("Failed to create theme: {}", e));
                        }
                    }
                }
                KvantumMessage::NewThemeCreated(None) => {
                    // Dialog cancelled
                }
                KvantumMessage::ApplySystemTheme(name) => {
                    let theme_name = name.clone();
                    return Task::perform(
                        async move {
                            let svc = keasyditor_core::services::ProcessService::new();
                            match svc.apply_kvantum_theme(&theme_name) {
                                Ok(r) if r.is_success() => Ok(theme_name),
                                Ok(r) => Err(format!("Failed: {}", r.stderr)),
                                Err(e) => Err(format!("Error: {}", e)),
                            }
                        },
                        |result| Message::Kvantum(KvantumMessage::SystemThemeApplied(result)),
                    );
                }
                KvantumMessage::SystemThemeApplied(result) => {
                    match result {
                        Ok(name) => {
                            self.reload_status = Some(format!("Applied: {}", name));
                            self.active_kvantum_theme = Some(name);
                            // Reload the editor with the newly applied theme
                            self.kvantum_config = None;
                            return self.update(Message::Kvantum(KvantumMessage::LoadActiveTheme));
                        }
                        Err(msg) => {
                            self.reload_status = Some(msg);
                        }
                    }
                }
                KvantumMessage::ThemesDiscovered(themes) => {
                    // Update installed_themes for home page (name, is_system)
                    self.installed_themes = themes
                        .iter()
                        .map(|(name, _, is_system)| (name.clone(), *is_system))
                        .collect();
                    self.kvantum_discovered_themes = themes;
                }
            },

            // -- Settings messages --
            Message::Settings(msg) => match msg {
                SettingsMessage::AppSettingsLoaded(Ok(content)) => {
                    for line in content.lines() {
                        if let Some(val) = line.strip_prefix("auto_apply=") {
                            self.auto_apply = val == "true";
                        }
                    }
                }
                SettingsMessage::AppSettingsLoaded(Err(_)) => {
                    // File doesn't exist yet — keep defaults
                }
                SettingsMessage::AppSettingsSaved => {}
                SettingsMessage::RecentFilesLoaded(files) => {
                    self.recent_files = files;
                }
                SettingsMessage::RecentFilesSaved => {}
                SettingsMessage::ToggleAutoApply(v) => {
                    self.auto_apply = v;
                    return self.save_app_settings();
                }
                SettingsMessage::ReloadKlassy => {
                    self.reload_status = Some("Reloading Klassy...".to_string());
                    return Task::perform(
                        async {
                            let svc = keasyditor_core::services::ProcessService::new();
                            match svc.reconfigure_kwin() {
                                Ok(result) => {
                                    if result.is_success() {
                                        Ok("Klassy reloaded successfully.".to_string())
                                    } else {
                                        Err(format!("Error: {}", result.stderr))
                                    }
                                }
                                Err(e) => Err(format!("Error: {}", e)),
                            }
                        },
                        |result| {
                            Message::Settings(SettingsMessage::ReloadKlassyResult(result))
                        },
                    );
                }
                SettingsMessage::ReloadKvantum => {
                    self.reload_status = Some("Reloading Kvantum...".to_string());
                    let theme_name = self.kvantum_theme_name.clone();
                    return Task::perform(
                        async move {
                            let svc = keasyditor_core::services::ProcessService::new();
                            if let Some(name) = theme_name {
                                match svc.apply_kvantum_theme(&name) {
                                    Ok(result) => {
                                        if result.is_success() {
                                            Ok("Kvantum reloaded successfully.".to_string())
                                        } else {
                                            Err(format!("Error: {}", result.stderr))
                                        }
                                    }
                                    Err(e) => Err(format!("Error: {}", e)),
                                }
                            } else {
                                Err("Error: No active Kvantum theme to reload.".to_string())
                            }
                        },
                        |result| {
                            Message::Settings(SettingsMessage::ReloadKvantumResult(result))
                        },
                    );
                }
                SettingsMessage::ReloadKlassyResult(result) => {
                    self.reload_status = Some(match result {
                        Ok(msg) => msg,
                        Err(msg) => msg,
                    });
                }
                SettingsMessage::ReloadKvantumResult(result) => {
                    self.reload_status = Some(match result {
                        Ok(msg) => msg,
                        Err(msg) => msg,
                    });
                }
                SettingsMessage::DetectWallpaper | SettingsMessage::PickWallpaperImage => {
                    // Both handled via SaveAndApply button now; kept for compatibility
                }
                SettingsMessage::WallpaperPaletteResult { dark, light, is_dark } => {
                    self.matugen_loading = false;
                    self.matugen_dark_palette = dark;
                    self.matugen_light_palette = light;
                    self.prefer_dark_palette = is_dark;
                }
                SettingsMessage::ToggleDarkPalette(prefer_dark) => {
                    self.prefer_dark_palette = prefer_dark;
                }
                SettingsMessage::ShowApplyToSystemConfirm => {
                    self.show_apply_to_system_confirm = true;
                    self.apply_to_system_status = None;
                }
                SettingsMessage::HideApplyToSystemConfirm => {
                    self.show_apply_to_system_confirm = false;
                }
                SettingsMessage::ApplyToSystem => {
                    self.show_apply_to_system_confirm = false;
                    let dark = vec_to_map(&self.matugen_dark_palette);
                    let light = vec_to_map(&self.matugen_light_palette);
                    if dark.is_empty() && light.is_empty() {
                        self.apply_to_system_status = Some((
                            false,
                            vec![i18n::t("home.palette.apply_no_palette")],
                        ));
                        return Task::none();
                    }
                    let prefer_dark = self.prefer_dark_palette;
                    return Task::perform(
                        async move {
                            use keasyditor_core::services::{
                                MatugenPalette, WallpaperApplyService,
                            };
                            let palette = MatugenPalette {
                                image_path: String::new(),
                                dark,
                                light,
                            };
                            let svc = WallpaperApplyService::new();
                            let outcome = svc.apply(&palette, prefer_dark);
                            (outcome.ok, outcome.steps)
                        },
                        |(ok, steps)| {
                            Message::Settings(SettingsMessage::ApplyToSystemResult { ok, steps })
                        },
                    );
                }
                SettingsMessage::ApplyToSystemResult { ok, steps } => {
                    self.apply_to_system_status = Some((ok, steps));
                    // Re-discover themes in case Kvantum picked up the rewritten config.
                    return Task::perform(
                        async {
                            let file_svc = keasyditor_core::services::FileService::new();
                            let discovery =
                                keasyditor_core::services::ThemeDiscoveryService::new(file_svc);
                            discovery
                                .discover_kvantum_themes()
                                .into_iter()
                                .map(|t| (t.name, t.path, t.is_system))
                                .collect::<Vec<_>>()
                        },
                        |themes| Message::Kvantum(KvantumMessage::ThemesDiscovered(themes)),
                    );
                }
                _ => {}
            },

            // -- UI widget state messages --
            Message::SliderChanged { key, value } => {
                self.slider_values.insert(key.clone(), value);
                // Keep text_input_values in sync for display only — no undo push yet
                let text_val = if value.fract().abs() < 0.01 {
                    format!("{}", value as i32)
                } else {
                    format!("{:.1}", value)
                };
                self.text_input_values.insert(key, text_val);
            }
            Message::SliderReleased(key) => {
                // Color channel slider (e.g. "color.klassy.shadow_color.r")?
                // Compute hex from current R/G/B and commit to config.
                let color_prefix = key
                    .strip_suffix(".r")
                    .or_else(|| key.strip_suffix(".g"))
                    .or_else(|| key.strip_suffix(".b"));

                if let Some(prefix) = color_prefix {
                    let r = self.slider_values.get(&format!("{}.r", prefix)).copied().unwrap_or(0.0) as u8;
                    let g = self.slider_values.get(&format!("{}.g", prefix)).copied().unwrap_or(0.0) as u8;
                    let b = self.slider_values.get(&format!("{}.b", prefix)).copied().unwrap_or(0.0) as u8;
                    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
                    let hex_key = format!("{}.hex", prefix);
                    self.text_input_values.insert(hex_key.clone(), hex.clone());
                    self.sync_ui_value_to_config(&hex_key, &hex);
                } else if let Some(text_val) = self.text_input_values.get(&key).cloned() {
                    self.sync_ui_value_to_config(&key, &text_val);
                }
            }
            Message::ColorPickerChanged { key_prefix, r, g, b } => {
                // UI update only — no undo push, no config write (like SliderChanged)
                let prefix = format!("color.{}", key_prefix);
                self.slider_values.insert(format!("{}.r", prefix), r as f32);
                self.slider_values.insert(format!("{}.g", prefix), g as f32);
                self.slider_values.insert(format!("{}.b", prefix), b as f32);
                let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
                self.text_input_values.insert(format!("{}.hex", prefix), hex);
            }
            Message::ColorPickerReleased(key_prefix) => {
                // Commit to config + push undo (like SliderReleased)
                let prefix = format!("color.{}", key_prefix);
                let hex_key = format!("{}.hex", prefix);
                if let Some(hex) = self.text_input_values.get(&hex_key).cloned() {
                    self.sync_ui_value_to_config(&hex_key, &hex);
                }
            }
            Message::ToggleChanged { key, value } => {
                self.toggle_values.insert(key.clone(), value);
                // Propagate to config model
                self.sync_toggle_to_config(&key, value);
            }
            Message::TextInputChanged { key, value } => {
                self.text_input_values.insert(key.clone(), value.clone());
                // Try to parse as f32 and update slider
                if let Ok(v) = value.parse::<f32>() {
                    self.slider_values.insert(key.clone(), v);
                }
                // Propagate to config model
                self.sync_ui_value_to_config(&key, &value);
            }
            Message::DropdownChanged { key, value } => {
                self.text_input_values.insert(key.clone(), value.clone());
                // Propagate to config model
                self.sync_ui_value_to_config(&key, &value);
            }
            Message::ToggleSection(section_id) => {
                // Toggle in the appropriate expanded set based on prefix
                let set = if section_id.starts_with("kvantum.") || section_id.contains("kvantum.") {
                    &mut self.kvantum_expanded
                } else {
                    &mut self.klassy_expanded
                };
                if set.contains(&section_id) {
                    set.remove(&section_id);
                } else {
                    set.insert(section_id);
                }
            }
            Message::Svg(msg) => match msg {
                SvgMessage::ImportSvg => {
                    return Task::perform(
                        async {
                            let file = rfd::AsyncFileDialog::new()
                                .set_title("Import SVG file")
                                .add_filter("SVG", &["svg"])
                                .pick_file()
                                .await;
                            match file {
                                Some(handle) => {
                                    let path = handle.path().to_string_lossy().to_string();
                                    tokio::fs::read_to_string(&path).await.ok()
                                }
                                None => None,
                            }
                        },
                        |content| Message::Svg(SvgMessage::SvgImported(content)),
                    );
                }
                SvgMessage::SvgImported(Some(content)) => {
                    self.kvantum_svg_content = Some(content);
                }
                SvgMessage::SvgImported(None) => {}
                SvgMessage::UpdateFill { element_id, color } => {
                    if let Some(svg) = &self.kvantum_svg_content {
                        self.kvantum_svg_content = Some(
                            keasyditor_core::svg::set_fill_color(svg, &element_id, &color),
                        );
                    }
                }
                SvgMessage::UpdateStroke { element_id, color } => {
                    if let Some(svg) = &self.kvantum_svg_content {
                        self.kvantum_svg_content = Some(
                            keasyditor_core::svg::set_stroke_color(svg, &element_id, &color),
                        );
                    }
                }
                _ => {}
            },
            Message::FontLoaded(_) => {}
            Message::SaveFlashDone => {
                self.save_flash = false;
            }
            Message::SearchChanged(text) => {
                self.search_text = text;
            }
            Message::Noop => {
                // no-op, used when file picker is cancelled
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        use iced::widget::row;

        let nav = ui::shell::nav_rail(self.page, self.klassy_installed, self.kvantum_installed);

        let content: Element<Message> = match self.page {
            Page::Home => {
                let active_palette = if self.prefer_dark_palette {
                    &self.matugen_dark_palette
                } else {
                    &self.matugen_light_palette
                };
                ui::home::home_page(
                    &self.installed_themes,
                    self.matugen_loading,
                    active_palette,
                    self.prefer_dark_palette,
                    self.klassy_installed,
                    self.kvantum_installed,
                    self.matugen_installed,
                    self.active_kvantum_theme.as_deref(),
                    self.show_apply_to_system_confirm,
                    self.apply_to_system_status.as_ref(),
                )
            }
            Page::Klassy => ui::klassy::editor::klassy_editor(
                self.klassy_tab,
                self.klassy_dirty(),
                self.klassy_file_path.as_deref(),
                &self.klassy_expanded,
                &self.slider_values,
                &self.toggle_values,
                &self.text_input_values,
                self.klassy_can_undo(),
                self.klassy_can_redo(),
                self.klassy_loading,
                self.klassy_error.as_deref(),
                self.save_flash,
            ),
            Page::Kvantum => ui::kvantum::editor::kvantum_editor(
                self.kvantum_tab,
                self.kvantum_dirty(),
                self.kvantum_dir_path.as_deref(),
                &self.kvantum_expanded,
                &self.slider_values,
                &self.toggle_values,
                &self.text_input_values,
                self.kvantum_svg_content.as_deref(),
                self.kvantum_can_undo(),
                self.kvantum_can_redo(),
                self.kvantum_loading,
                self.kvantum_error.as_deref(),
                self.save_flash,
            ),
            Page::Settings => ui::settings::settings_page(
                &self.reload_status,
            ),
        };

        let content_area = iced::widget::container(content)
            .width(iced::Fill)
            .height(iced::Fill);

        row![nav, content_area].into()
    }

    // --------------------------------------------------------------------------
    // Config <-> UI synchronization helpers
    // --------------------------------------------------------------------------

    /// Sync UI slider/toggle values from the loaded Klassy config.
    fn sync_klassy_ui_from_config(&mut self) {
        if let Some(config) = &self.klassy_config {
            // Corner radius
            if let Some(v) = config.window_corner_radius() {
                self.slider_values
                    .insert("klassy.corner_radius".to_string(), v as f32);
            }
            // Shadow strength
            if let Some(v) = config.shadow_strength() {
                self.slider_values
                    .insert("klassy.shadow_strength".to_string(), v as f32);
            }
            // Titlebar opacity
            if let Some(v) = config.active_title_bar_opacity() {
                self.slider_values
                    .insert("klassy.titlebar_opacity".to_string(), v as f32);
            }
            if let Some(v) = config.inactive_title_bar_opacity() {
                self.slider_values
                    .insert("klassy.titlebar_opacity_inactive".to_string(), v as f32);
            }
            // Button background opacity
            if let Some(v) = config.button_background_opacity_active() {
                self.slider_values
                    .insert("klassy.bg_opacity_active".to_string(), v as f32);
            }
            if let Some(v) = config.button_background_opacity_inactive() {
                self.slider_values
                    .insert("klassy.bg_opacity_inactive".to_string(), v as f32);
            }
            // Titlebar spacing
            if let Some(v) = config.title_bar_top_margin() {
                self.slider_values
                    .insert("klassy.titlebar_top_margin".to_string(), v as f32);
            }
            if let Some(v) = config.title_bar_bottom_margin() {
                self.slider_values
                    .insert("klassy.titlebar_bottom_margin".to_string(), v as f32);
            }
            // Button spacing
            if let Some(v) = config.get_value("Windeco", "ButtonSpacingLeft")
                && let Ok(n) = v.parse::<f32>() {
                    self.slider_values
                        .insert("klassy.button_spacing_left".to_string(), n);
                }
            if let Some(v) = config.get_value("Windeco", "ButtonSpacingRight")
                && let Ok(n) = v.parse::<f32>() {
                    self.slider_values
                        .insert("klassy.button_spacing_right".to_string(), n);
                }
            // Shadow size
            if let Some(v) = config.get_value("ShadowStyle", "ShadowSize")
                && let Ok(n) = v.parse::<f32>() {
                    self.slider_values
                        .insert("klassy.shadow_size".to_string(), n);
                }
            // Side padding
            if let Some(v) = config.get_value("TitleBarSpacing", "TitleBarSidePadding")
                && let Ok(n) = v.parse::<f32>() {
                    self.slider_values
                        .insert("klassy.titlebar_side_padding".to_string(), n);
                }
            // Animation duration
            if let Some(v) = config.animations_speed_relative_system() {
                self.slider_values
                    .insert("klassy.anim_duration".to_string(), v as f32);
            }
            // Toggles
            if let Some(v) = config.draw_title_bar_separator() {
                self.toggle_values
                    .insert("klassy.titlebar_separator".to_string(), v);
            }
            if let Some(v) = config.get_value("Windeco", "DrawBorderOnMaximizedWindows") {
                self.toggle_values
                    .insert("klassy.border_maximized".to_string(), v == "true");
            }
            if let Some(v) = config.get_value("Windeco", "DrawHighlightOnActiveWindow") {
                self.toggle_values
                    .insert("klassy.active_highlight".to_string(), v == "true");
            }
            // Dropdowns — strip "Shape"/"Style" prefix where needed
            if let Some(v) = config.get_value("Windeco", "ButtonShape") {
                let display = v.strip_prefix("Shape").unwrap_or(v);
                self.text_input_values
                    .insert("klassy.button_shape".to_string(), display.to_string());
            }
            if let Some(v) = config.get_value("Windeco", "ButtonIconStyle") {
                let display = v.strip_prefix("Style").unwrap_or(v);
                self.text_input_values
                    .insert("klassy.button_icon_style".to_string(), display.to_string());
            }
            if let Some(v) = config.get_value("ButtonColors", "ButtonBackgroundColorsActive") {
                self.text_input_values
                    .insert("klassy.bg_colors_active".to_string(), v.to_string());
            }
            if let Some(v) = config.get_value("ButtonColors", "ButtonBackgroundColorsInactive") {
                self.text_input_values
                    .insert("klassy.bg_colors_inactive".to_string(), v.to_string());
            }
            if let Some(v) = config.get_value("Windeco", "WindowOutlineStyleActive") {
                self.text_input_values
                    .insert("klassy.outline_active".to_string(), v.to_string());
            }
            if let Some(v) = config.get_value("Windeco", "WindowOutlineStyleInactive") {
                self.text_input_values
                    .insert("klassy.outline_inactive".to_string(), v.to_string());
            }
            if let Some(v) = config.get_value("Windeco", "TitleAlignment") {
                self.text_input_values
                    .insert("klassy.title_alignment".to_string(), v.to_string());
            }
            if let Some(v) = config.get_value("Windeco", "BorderSize") {
                self.text_input_values
                    .insert("klassy.kwin_border_size".to_string(), v.to_string());
            }
            // Sync color values back to slider_values + text_input_values
            let color_mappings: &[(&str, &str, &str)] = &[
                ("klassy.shadow_color", "ShadowStyle", "ShadowColor"),
                ("klassy.close_icon_color", "ButtonColors", "CloseButtonCustomIconColorActive"),
                ("klassy.minimize_icon_color", "ButtonColors", "MinimizeButtonCustomIconColorActive"),
                ("klassy.maximize_icon_color", "ButtonColors", "MaximizeButtonCustomIconColorActive"),
                ("klassy.outline_color_active", "Windeco", "WindowOutlineCustomColorActive"),
                ("klassy.outline_color_inactive", "Windeco", "WindowOutlineCustomColorInactive"),
            ];
            for &(key_prefix, section, ini_key) in color_mappings {
                if let Some(v) = config.get_value(section, ini_key)
                    && let Some(rgba) = keasyditor_core::color::try_parse(v) {
                        let prefix = format!("color.{}", key_prefix);
                        self.slider_values.insert(format!("{}.r", prefix), rgba.r as f32);
                        self.slider_values.insert(format!("{}.g", prefix), rgba.g as f32);
                        self.slider_values.insert(format!("{}.b", prefix), rgba.b as f32);
                        self.text_input_values.insert(
                            format!("{}.hex", prefix),
                            format!("#{:02X}{:02X}{:02X}", rgba.r, rgba.g, rgba.b),
                        );
                    }
            }
        }
    }

    /// Sync UI slider/toggle values from the loaded Kvantum config.
    fn sync_kvantum_ui_from_config(&mut self) {
        if let Some(config) = &self.kvantum_config {
            // General sliders
            self.slider_values.insert("kvantum.window_opacity_reduction".into(), config.general.reduce_window_opacity() as f32);
            self.slider_values.insert("kvantum.reduce_opacity".into(), config.general.reduce_menu_opacity() as f32);
            self.slider_values.insert("kvantum.scrollbar_width".into(), config.general.slider_width() as f32);
            self.slider_values.insert("kvantum.scroll_arrow_size".into(), config.general.scroll_min_extent() as f32);
            self.slider_values.insert("kvantum.general.small_icon_size".into(), config.general.small_icon_size() as f32);
            self.slider_values.insert("kvantum.general.large_icon_size".into(), config.general.large_icon_size() as f32);
            self.slider_values.insert("kvantum.general.slider_width".into(), config.general.slider_width() as f32);
            self.slider_values.insert("kvantum.general.slider_handle_width".into(), config.general.slider_handle_width() as f32);
            self.slider_values.insert("kvantum.general.slider_handle_length".into(), config.general.slider_handle_length() as f32);
            self.slider_values.insert("kvantum.general.layout_spacing".into(), config.general.layout_spacing() as f32);
            self.slider_values.insert("kvantum.general.layout_margin".into(), config.general.layout_margin() as f32);
            self.slider_values.insert("kvantum.general.submenu_overlap".into(), 0.0);
            // Visual sliders
            self.slider_values.insert("kvantum.general.contrast".into(), (config.general.contrast() * 10.0) as f32);
            self.slider_values.insert("kvantum.general.intensity".into(), (config.general.intensity() * 10.0) as f32);
            self.slider_values.insert("kvantum.general.saturation".into(), (config.general.saturation() * 10.0) as f32);
            self.slider_values.insert("kvantum.general.menu_shadow_depth".into(), config.general.menu_shadow_depth() as f32);
            self.slider_values.insert("kvantum.general.tooltip_shadow_depth".into(), config.general.tooltip_shadow_depth() as f32);
            self.slider_values.insert("kvantum.general.splitter_width".into(), 7.0); // no typed getter
            self.slider_values.insert("kvantum.general.scroll_width".into(), config.general.slider_width() as f32);
            self.slider_values.insert("kvantum.general.arrow_size".into(), 9.0); // no typed getter

            // General toggles — all using kvantum.general.* keys matching tabs.rs
            self.toggle_values.insert("kvantum.general.composite".into(), config.general.composite());
            self.toggle_values.insert("kvantum.general.translucent_windows".into(), config.general.translucent_windows());
            self.toggle_values.insert("kvantum.general.blurring".into(), config.general.blurring());
            self.toggle_values.insert("kvantum.general.popup_blurring".into(), config.general.popup_blurring());
            self.toggle_values.insert("kvantum.general.animate_states".into(), config.general.animate_states());
            self.toggle_values.insert("kvantum.general.fill_rubberband".into(), config.general.fill_rubberband());
            self.toggle_values.insert("kvantum.general.no_window_pattern".into(), config.general.no_window_pattern());
            self.toggle_values.insert("kvantum.general.shadowless_popup".into(), false); // no typed getter
            self.toggle_values.insert("kvantum.general.left_handed_mouse".into(), false); // no typed getter
            self.toggle_values.insert("kvantum.general.scrollbar_in_view".into(), true); // no typed getter
            self.toggle_values.insert("kvantum.general.scroll_arrows".into(), config.general.scroll_arrows());
            self.toggle_values.insert("kvantum.general.scrollable_menu".into(), config.general.scrollable_menu());
            self.toggle_values.insert("kvantum.general.alt_mnemonic".into(), config.general.alt_mnemonic());
            self.toggle_values.insert("kvantum.general.drag_from_buttons".into(), false); // no typed getter
            self.toggle_values.insert("kvantum.general.double_click".into(), false); // no typed getter

            // Legacy toggle keys (for backward compat with old handlers)
            self.toggle_values.insert("kvantum.animate_states".into(), config.general.animate_states());
            self.toggle_values.insert("kvantum.composite".into(), config.general.composite());

            // General dropdowns
            self.text_input_values.insert("kvantum.general.x11drag".into(), config.general.x11drag());
            self.text_input_values.insert("kvantum.general.click_behavior".into(), config.general.click_behavior().to_string());

            // Hacks — using kvantum.hacks.* keys matching tabs.rs
            for &(hack_key, _) in &[
                ("transparent_dolphin_view", ""), ("transparent_ktitle_label", ""),
                ("transparent_menu_title", ""), ("blur_translucent", ""),
                ("respect_DE", ""), ("force_size_grip", ""),
                ("middle_click_scroll", ""), ("normal_default", ""),
                ("iconless_pushbutton", ""), ("iconless_menu", ""),
                ("single_top_toolbar", ""), ("no_inactive_tab_separator", ""),
                ("transparent_arrow_button", ""), ("tint_current_tab", ""),
                ("center_toolbar_handle", ""), ("joined_inactive_tabs", ""),
            ] {
                let full_key = format!("kvantum.hacks.{}", hack_key);
                let val = config.hacks.get(hack_key).map(|v| v == "true").unwrap_or(false);
                self.toggle_values.insert(full_key, val);
            }

            // Legacy hack keys
            self.toggle_values.insert("kvantum.transparent_dolphin".into(),
                config.hacks.get("transparent_dolphin_view").map(|v| v == "true").unwrap_or(false));
            self.toggle_values.insert("kvantum.transparent_ktitle".into(),
                config.hacks.get("transparent_ktitle_label").map(|v| v == "true").unwrap_or(false));
            self.toggle_values.insert("kvantum.blur_translucent".into(),
                config.hacks.get("blur_translucent").map(|v| v == "true").unwrap_or(true));
            self.toggle_values.insert("kvantum.respect_de_colors".into(),
                config.hacks.get("respect_darkness").map(|v| v == "true").unwrap_or(false));

            // Hack sliders
            self.slider_values.insert("kvantum.hacks.kcapacitybar_width".into(),
                config.hacks.get("kcapacitybar_width").and_then(|v| v.parse().ok()).unwrap_or(0.0));
            self.slider_values.insert("kvantum.hacks.lxqtmainmenu_iconsize".into(),
                config.hacks.get("lxqtmainmenu_iconsize").and_then(|v| v.parse().ok()).unwrap_or(22.0));
            self.slider_values.insert("kvantum.hacks.disabled_icon_opacity".into(),
                config.hacks.get("disabled_icon_opacity").and_then(|v| v.parse().ok()).unwrap_or(100.0));
            // Widget sections — store ALL key-value pairs as text_input_values
            for (sec_name, section) in &config.widget_sections {
                let prefix = format!("kvantum.widget.{}", sec_name);
                for (key, value) in section.to_map() {
                    self.text_input_values.insert(format!("{}.{}", prefix, key), value);
                }
            }

            // Colors — populate both text_input_values and slider_values for color picker
            for (key, value) in config.colors.to_map() {
                let key_prefix = format!("kvantum.color.{}", key);
                self.text_input_values.insert(key_prefix.clone(), value.clone());
                // Also populate color picker fields (slider R/G/B + hex)
                if let Some(rgba) = keasyditor_core::color::try_parse(&value) {
                    let prefix = format!("color.{}", key_prefix);
                    self.slider_values.insert(format!("{}.r", prefix), rgba.r as f32);
                    self.slider_values.insert(format!("{}.g", prefix), rgba.g as f32);
                    self.slider_values.insert(format!("{}.b", prefix), rgba.b as f32);
                    self.text_input_values.insert(
                        format!("{}.hex", prefix),
                        format!("#{:02X}{:02X}{:02X}", rgba.r, rgba.g, rgba.b),
                    );
                }
            }
        }
    }

    /// Propagate a slider/text/dropdown value change to the underlying config model.
    fn sync_ui_value_to_config(&mut self, key: &str, value: &str) {
        match key {
            // Klassy sliders -> config
            "klassy.corner_radius" => {
                if let (Some(config), Ok(v)) = (&mut self.klassy_config, value.parse::<i64>()) {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_window_corner_radius(v);
                }
            }
            "klassy.shadow_strength" => {
                if let (Some(config), Ok(v)) = (&mut self.klassy_config, value.parse::<i64>()) {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_shadow_strength(v);
                }
            }
            "klassy.titlebar_opacity" => {
                if let (Some(config), Ok(v)) = (&mut self.klassy_config, value.parse::<i64>()) {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_active_title_bar_opacity(v);
                }
            }
            "klassy.titlebar_opacity_inactive" => {
                if let (Some(config), Ok(v)) = (&mut self.klassy_config, value.parse::<i64>()) {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_inactive_title_bar_opacity(v);
                }
            }
            "klassy.bg_opacity_active" => {
                if let (Some(config), Ok(v)) = (&mut self.klassy_config, value.parse::<i64>()) {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_button_background_opacity_active(v);
                }
            }
            "klassy.bg_opacity_inactive" => {
                if let (Some(config), Ok(v)) = (&mut self.klassy_config, value.parse::<i64>()) {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_button_background_opacity_inactive(v);
                }
            }
            "klassy.titlebar_side_padding" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value("TitleBarSpacing", "TitleBarSidePadding", value.to_string());
                }
            }
            "klassy.titlebar_top_margin" => {
                if let (Some(config), Ok(v)) = (&mut self.klassy_config, value.parse::<f64>()) {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_title_bar_top_margin(v);
                }
            }
            "klassy.titlebar_bottom_margin" => {
                if let (Some(config), Ok(v)) = (&mut self.klassy_config, value.parse::<f64>()) {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_title_bar_bottom_margin(v);
                }
            }
            "klassy.anim_duration" => {
                if let (Some(config), Ok(v)) = (&mut self.klassy_config, value.parse::<i64>()) {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_animations_speed_relative_system(v);
                }
            }
            "klassy.shadow_size" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value("ShadowStyle", "ShadowSize", value.to_string());
                }
            }
            "klassy.button_spacing_left" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value("Windeco", "ButtonSpacingLeft", value.to_string());
                }
            }
            "klassy.button_spacing_right" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value("Windeco", "ButtonSpacingRight", value.to_string());
                }
            }
            "klassy.hover_opacity" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value("Windeco", "HoverHighlightOpacity", value.to_string());
                }
            }
            // Klassy dropdowns
            "klassy.button_shape" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value("Windeco", "ButtonShape", format!("Shape{}", value));
                }
            }
            "klassy.button_icon_style" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value("Windeco", "ButtonIconStyle", format!("Style{}", value));
                }
            }
            "klassy.bg_colors_active" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value(
                        "ButtonColors",
                        "ButtonBackgroundColorsActive",
                        value.to_string(),
                    );
                }
            }
            "klassy.bg_colors_inactive" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value(
                        "ButtonColors",
                        "ButtonBackgroundColorsInactive",
                        value.to_string(),
                    );
                }
            }
            "klassy.outline_active" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value(
                        "Windeco",
                        "WindowOutlineStyleActive",
                        value.to_string(),
                    );
                }
            }
            "klassy.outline_inactive" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value(
                        "Windeco",
                        "WindowOutlineStyleInactive",
                        value.to_string(),
                    );
                }
            }
            // Kvantum legacy sliders — use new_config pattern for correct undo
            "kvantum.window_opacity_reduction" => {
                if let Ok(v) = value.parse::<i64>()
                    && let Some(config) = self.kvantum_config.take() {
                        let mut g = config.general.clone();
                        g.set_reduce_window_opacity(v);
                        let nc = config.copy_with_general(g);
                        if let Some(undo) = &mut self.kvantum_undo { undo.push(nc.clone()); }
                        self.kvantum_config = Some(nc);
                    }
            }
            "kvantum.reduce_opacity" => {
                if let Ok(v) = value.parse::<i64>()
                    && let Some(config) = self.kvantum_config.take() {
                        let mut g = config.general.clone();
                        g.set_reduce_menu_opacity(v);
                        let nc = config.copy_with_general(g);
                        if let Some(undo) = &mut self.kvantum_undo { undo.push(nc.clone()); }
                        self.kvantum_config = Some(nc);
                    }
            }
            "kvantum.scrollbar_width" => {
                if let Ok(v) = value.parse::<i64>()
                    && let Some(config) = self.kvantum_config.take() {
                        let mut g = config.general.clone();
                        g.set_slider_width(v);
                        let nc = config.copy_with_general(g);
                        if let Some(undo) = &mut self.kvantum_undo { undo.push(nc.clone()); }
                        self.kvantum_config = Some(nc);
                    }
            }
            "kvantum.scroll_arrow_size" => {
                if let Ok(v) = value.parse::<i64>()
                    && let Some(config) = self.kvantum_config.take() {
                        let mut g = config.general.clone();
                        g.set_scroll_min_extent(v);
                        let nc = config.copy_with_general(g);
                        if let Some(undo) = &mut self.kvantum_undo { undo.push(nc.clone()); }
                        self.kvantum_config = Some(nc);
                    }
            }
            "kvantum.highlight_opacity" | "kvantum.pushbutton_min_width"
            | "kvantum.pushbutton_min_height" | "kvantum.combobox_min_width" => {
                // stubs for future
            }
            // Klassy color fields (hex "#RRGGBB" → Klassy "R,G,B")
            "color.klassy.shadow_color.hex" => {
                if let Some(rgba) = keasyditor_core::color::try_parse(value) {
                    let c = keasyditor_core::color::to_klassy_color(&rgba);
                    if let Some(config) = &mut self.klassy_config {
                        if let Some(undo) = &mut self.klassy_undo { undo.push(config.clone()); }
                        config.set_value("ShadowStyle", "ShadowColor", c);
                    }
                }
            }
            "color.klassy.outline_color_active.hex" => {
                if let Some(rgba) = keasyditor_core::color::try_parse(value) {
                    let c = keasyditor_core::color::to_klassy_color(&rgba);
                    if let Some(config) = &mut self.klassy_config {
                        if let Some(undo) = &mut self.klassy_undo { undo.push(config.clone()); }
                        config.set_value("Windeco", "WindowOutlineCustomColorActive", c.clone());
                        config.set_value("WindowOutlineStyle", "WindowOutlineCustomColorActive", c);
                    }
                }
            }
            "color.klassy.outline_color_inactive.hex" => {
                if let Some(rgba) = keasyditor_core::color::try_parse(value) {
                    let c = keasyditor_core::color::to_klassy_color(&rgba);
                    if let Some(config) = &mut self.klassy_config {
                        if let Some(undo) = &mut self.klassy_undo { undo.push(config.clone()); }
                        config.set_value("Windeco", "WindowOutlineCustomColorInactive", c.clone());
                        config.set_value("WindowOutlineStyle", "WindowOutlineCustomColorInactive", c);
                    }
                }
            }
            "color.klassy.close_icon_color.hex" => {
                if let Some(rgba) = keasyditor_core::color::try_parse(value) {
                    let c = keasyditor_core::color::to_klassy_color(&rgba);
                    if let Some(config) = &mut self.klassy_config {
                        if let Some(undo) = &mut self.klassy_undo { undo.push(config.clone()); }
                        config.set_value("ButtonColors", "CloseButtonCustomIconColorActive", c.clone());
                        config.set_value("ButtonColors", "CloseButtonCustomIconColorInactive", c.clone());
                        config.set_value("Windeco", "CloseButtonCustomIconColorActive", c.clone());
                        config.set_value("Windeco", "CloseButtonCustomIconColorInactive", c);
                    }
                }
            }
            "color.klassy.minimize_icon_color.hex" => {
                if let Some(rgba) = keasyditor_core::color::try_parse(value) {
                    let c = keasyditor_core::color::to_klassy_color(&rgba);
                    if let Some(config) = &mut self.klassy_config {
                        if let Some(undo) = &mut self.klassy_undo { undo.push(config.clone()); }
                        config.set_value("ButtonColors", "MinimizeButtonCustomIconColorActive", c.clone());
                        config.set_value("ButtonColors", "MinimizeButtonCustomIconColorInactive", c);
                    }
                }
            }
            "color.klassy.maximize_icon_color.hex" => {
                if let Some(rgba) = keasyditor_core::color::try_parse(value) {
                    let c = keasyditor_core::color::to_klassy_color(&rgba);
                    if let Some(config) = &mut self.klassy_config {
                        if let Some(undo) = &mut self.klassy_undo { undo.push(config.clone()); }
                        config.set_value("ButtonColors", "MaximizeButtonCustomIconColorActive", c.clone());
                        config.set_value("ButtonColors", "MaximizeButtonCustomIconColorInactive", c);
                    }
                }
            }
            // Kvantum widget section properties: "kvantum.widget.<Section>.<property>"
            other if other.starts_with("kvantum.widget.") => {
                // Parse: "kvantum.widget.PushButton.frame.top" → section="PushButton", prop="frame.top"
                let rest = other.strip_prefix("kvantum.widget.").unwrap_or("");
                // Find the section name (first segment before the property)
                // Section names are PascalCase, properties start with lowercase
                if let Some(dot_pos) = rest.find('.') {
                    let section_name = &rest[..dot_pos];
                    let property = &rest[dot_pos + 1..];
                    if !section_name.is_empty() && !property.is_empty() {
                        // Special case: inherits dropdown
                        let (ini_key, ini_val) = if property == "inherits" {
                            if value == "(none)" {
                                // Remove inherits
                                ("inherits", String::new())
                            } else {
                                ("inherits", value.to_string())
                            }
                        } else {
                            (property, value.to_string())
                        };

                        if let Some(config) = self.kvantum_config.take() {
                            let mut section = config
                                .get_section(section_name)
                                .cloned()
                                .unwrap_or_else(|| keasyditor_core::models::kvantum::KvantumSection::empty(section_name));

                            if ini_key == "inherits" && ini_val.is_empty() {
                                section.remove_key("inherits");
                            } else {
                                section.set_value(ini_key, ini_val);
                            }

                            let new_config = config.copy_with_section(section_name, section);
                            if let Some(undo) = &mut self.kvantum_undo {
                                undo.push(new_config.clone());
                            }
                            self.kvantum_config = Some(new_config);
                        }
                    }
                }
            }
            // Kvantum general dropdowns/text: "kvantum.general.*"
            other if other.starts_with("kvantum.general.") => {
                let general_key = other.strip_prefix("kvantum.general.").unwrap_or("");
                if let Some(config) = self.kvantum_config.take() {
                    let mut new_general = config.general.clone();
                    new_general.set_value(general_key, value.to_string());
                    let new_config = config.copy_with_general(new_general);
                    if let Some(undo) = &mut self.kvantum_undo {
                        undo.push(new_config.clone());
                    }
                    self.kvantum_config = Some(new_config);
                }
            }
            // Kvantum hack sliders: "kvantum.hacks.*"
            other if other.starts_with("kvantum.hacks.") && !other.ends_with(".search") => {
                let hack_key = other.strip_prefix("kvantum.hacks.").unwrap_or("");
                if !hack_key.is_empty()
                    && let Some(config) = self.kvantum_config.take() {
                        let mut new_hacks = config.hacks.clone();
                        new_hacks.insert(hack_key.to_string(), value.to_string());
                        let new_config = config.copy_with_hacks(new_hacks);
                        if let Some(undo) = &mut self.kvantum_undo {
                            undo.push(new_config.clone());
                        }
                        self.kvantum_config = Some(new_config);
                    }
            }
            // Kvantum color fields: "color.kvantum.color.<key>.hex"
            other if other.starts_with("color.kvantum.color.") && other.ends_with(".hex") => {
                if let Some(rgba) = keasyditor_core::color::try_parse(value) {
                    let color_key = other
                        .strip_prefix("color.kvantum.color.")
                        .and_then(|s| s.strip_suffix(".hex"))
                        .unwrap_or("");
                    if !color_key.is_empty() {
                        let hex = keasyditor_core::color::to_kvantum_color(&rgba);
                        if let Some(config) = self.kvantum_config.take() {
                            let mut new_colors = config.colors.clone();
                            new_colors.set_value(color_key, hex);
                            let new_config = config.copy_with_colors(new_colors);
                            if let Some(undo) = &mut self.kvantum_undo {
                                undo.push(new_config.clone());
                            }
                            self.kvantum_config = Some(new_config);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Propagate a toggle value change to the underlying config model.
    fn sync_toggle_to_config(&mut self, key: &str, value: bool) {
        let value_str = value.to_string();
        match key {
            "klassy.titlebar_separator" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_draw_title_bar_separator(value);
                }
            }
            "klassy.border_maximized" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value(
                        "Windeco",
                        "DrawBorderOnMaximizedWindows",
                        value_str,
                    );
                }
            }
            "klassy.active_highlight" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_value(
                        "Windeco",
                        "DrawHighlightOnActiveWindow",
                        value_str,
                    );
                }
            }
            "klassy.custom_outline" => {
                if let Some(config) = &mut self.klassy_config {
                    if let Some(undo) = &mut self.klassy_undo {
                        undo.push(config.clone());
                    }
                    config.set_colorize_window_outline_with_button(value);
                }
            }
            // Generic handler for kvantum.general.* toggles (includes legacy keys)
            other if other.starts_with("kvantum.general.") || other == "kvantum.animate_states" || other == "kvantum.composite" => {
                let general_key = other.strip_prefix("kvantum.general.")
                    .or_else(|| other.strip_prefix("kvantum."))
                    .unwrap_or("");
                if let Some(config) = self.kvantum_config.take() {
                    let mut new_general = config.general.clone();
                    new_general.set_value(general_key, value_str.clone());
                    let new_config = config.copy_with_general(new_general);
                    if let Some(undo) = &mut self.kvantum_undo {
                        undo.push(new_config.clone());
                    }
                    self.kvantum_config = Some(new_config);
                }
            }
            // Generic handler for kvantum.hacks.* toggles (includes legacy keys)
            other if other.starts_with("kvantum.hacks.") || other == "kvantum.transparent_dolphin"
                || other == "kvantum.transparent_ktitle" || other == "kvantum.blur_translucent"
                || other == "kvantum.respect_de_colors" => {
                let hack_key = if let Some(k) = other.strip_prefix("kvantum.hacks.") {
                    k.to_string()
                } else {
                    // Legacy key mapping
                    match other {
                        "kvantum.transparent_dolphin" => "transparent_dolphin_view".to_string(),
                        "kvantum.transparent_ktitle" => "transparent_ktitle_label".to_string(),
                        "kvantum.blur_translucent" => "blur_translucent".to_string(),
                        "kvantum.respect_de_colors" => "respect_darkness".to_string(),
                        _ => return,
                    }
                };
                if let Some(config) = self.kvantum_config.take() {
                    let mut new_hacks = config.hacks.clone();
                    new_hacks.insert(hack_key, value_str.clone());
                    let new_config = config.copy_with_hacks(new_hacks);
                    if let Some(undo) = &mut self.kvantum_undo {
                        undo.push(new_config.clone());
                    }
                    self.kvantum_config = Some(new_config);
                }
            }
            _ => {}
        }
    }

    /// Add a file path to the recent files list, keeping it deduplicated and capped.
    fn add_recent_file(&mut self, path: String) {
        self.recent_files.retain(|p| p != &path);
        self.recent_files.insert(0, path);
        self.recent_files.truncate(10);
    }

    /// Persist auto_apply to ~/.config/keasyditor/settings.ini.
    fn save_app_settings(&self) -> Task<Message> {
        let auto_apply = self.auto_apply;
        Task::perform(
            async move {
                let path = keasyditor_core::constants::keasyditor_settings_path();
                if let Some(parent) = path.parent() {
                    let _ = tokio::fs::create_dir_all(parent).await;
                }
                let content = format!("[settings]\nauto_apply={}\n", auto_apply);
                let _ = tokio::fs::write(&path, content).await;
            },
            |_| Message::Settings(SettingsMessage::AppSettingsSaved),
        )
    }

    /// Persist recent files list to ~/.cache/keasyditor/recent_files.
    fn save_recent_files(&self) -> Task<Message> {
        let recent = self.recent_files.clone();
        Task::perform(
            async move {
                let path = keasyditor_core::constants::keasyditor_recent_files_path();
                if let Some(parent) = path.parent() {
                    let _ = tokio::fs::create_dir_all(parent).await;
                }
                let _ = tokio::fs::write(&path, recent.join("\n")).await;
            },
            |_| Message::Settings(SettingsMessage::RecentFilesSaved),
        )
    }
}
