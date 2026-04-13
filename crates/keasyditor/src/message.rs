use std::path::PathBuf;

/// Top-level page in the navigation rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Klassy,
    Kvantum,
    Settings,
}

/// Top-level application message.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    /// Navigate to a page.
    NavigateTo(Page),

    /// Klassy editor messages.
    Klassy(KlassyMessage),

    /// Kvantum editor messages.
    Kvantum(KvantumMessage),

    /// SVG editor messages.
    Svg(SvgMessage),

    /// Settings messages.
    Settings(SettingsMessage),

    /// Font loaded event.
    FontLoaded(Result<(), iced::font::Error>),

    /// "Save & Apply" success flash — resets after 1 second.
    SaveFlashDone,

    /// Slider value changed (fires on every movement — no undo push).
    SliderChanged { key: String, value: f32 },

    /// Slider released (fires once on mouse-up — pushes undo + updates config).
    SliderReleased(String),

    /// Toggle value changed.
    ToggleChanged { key: String, value: bool },

    /// Text input value changed.
    TextInputChanged { key: String, value: String },

    /// Dropdown (pick_list) value changed.
    DropdownChanged { key: String, value: String },

    /// Toggle a collapsible section open/closed.
    ToggleSection(String),

    /// Undo on whichever page is currently active.
    UndoCurrentPage,

    /// Redo on whichever page is currently active.
    RedoCurrentPage,

    /// Save on whichever page is currently active.
    SaveCurrentPage,

    /// Search field text changed (for pickers).
    SearchChanged(String),

    /// Color picker wheel/square dragging — updates UI only (no undo push).
    ColorPickerChanged {
        key_prefix: String,
        r: u8,
        g: u8,
        b: u8,
    },

    /// Color picker wheel/square released — commits to config + pushes undo.
    ColorPickerReleased(String),

    /// No-op message (used when a dialog is cancelled).
    Noop,
}

/// Messages for the Klassy editor.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum KlassyMessage {
    LoadDefault,
    LoadFile(PathBuf),
    Save,
    SaveAndApply,
    UpdateKey {
        section: String,
        key: String,
        value: String,
    },
    Undo,
    Redo,
    Apply,
    TabChanged(usize),
    OpenFilePicker,
    ShowPresetPicker,
    HidePresetPicker,
    ApplyPreset(String),
    PresetsLoaded(Vec<String>),
    // Async results
    ConfigLoaded(Result<String, String>),
    Saved(Result<(), String>),
    SavedThenApply(Result<(), String>),
    Applied(Result<String, String>),
}

/// Messages for the Kvantum editor.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum KvantumMessage {
    LoadActiveTheme,
    LoadTheme(PathBuf),
    Save,
    SaveAndApply,
    UpdateColor {
        key: String,
        value: String,
    },
    UpdateGeneral {
        key: String,
        value: String,
    },
    UpdateSection {
        section: String,
        key: String,
        value: String,
    },
    Undo,
    Redo,
    Apply,
    TabChanged(usize),
    OpenThemePicker,
    ShowThemePicker,
    HideThemePicker,
    SelectTheme(String),
    NewTheme,
    NewThemeCreated(Option<String>),
    ApplySystemTheme(String),
    SystemThemeApplied(Result<String, String>),
    ThemesDiscovered(Vec<(String, String, bool)>),
    // Async results
    ThemeLoaded(Result<String, String>),
    Saved(Result<(), String>),
    SavedThenApply(Result<(), String>),
    Applied(Result<String, String>),
}

/// Messages for the SVG editor.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SvgMessage {
    SelectElement(String),
    UpdateFill { element_id: String, color: String },
    UpdateStroke { element_id: String, color: String },
    ImportSvg,
    SvgImported(Option<String>),
    ClearSelection,
}

/// Messages for the Settings page.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SettingsMessage {
    AppSettingsLoaded(Result<String, String>),
    AppSettingsSaved,
    RecentFilesLoaded(Vec<String>),
    RecentFilesSaved,
    ToggleAutoApply(bool),
    ExtractMatugenPalette,
    PaletteExtracted(Option<String>),
    ReloadKlassy,
    ReloadKvantum,
    ReloadKlassyResult(Result<String, String>),
    ReloadKvantumResult(Result<String, String>),
    DetectWallpaper,
    PickWallpaperImage,
    WallpaperPaletteResult {
        dark: Option<Vec<(String, String)>>,
        light: Option<Vec<(String, String)>>,
        is_dark: bool,
    },
    ToggleDarkPalette(bool),
    /// User pressed the "Set wallpaper colors to all system" button — show
    /// a confirmation prompt before overwriting anything.
    ShowApplyToSystemConfirm,
    /// User dismissed the confirmation prompt.
    HideApplyToSystemConfirm,
    /// User confirmed — run the full system-wide apply pipeline.
    ApplyToSystem,
    /// Result of the system-wide apply pipeline.
    ApplyToSystemResult {
        ok: bool,
        steps: Vec<String>,
    },
}
