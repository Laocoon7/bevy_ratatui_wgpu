use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

use bevy::{platform::collections::HashMap, prelude::*};

static FONT_LIST: OnceLock<Mutex<HashMap<PathBuf, &'static [u8]>>> = OnceLock::new();

/// The default font bytes bundled with the crate (`JuliaMono-Regular.ttf`).
/// Used as a fallback if custom font loading fails.
pub static DEFAULT_FONT: &[u8] = include_bytes!("../assets/fonts/JuliaMono-Regular.ttf");

/// Loads a font from the specified path, utilizing a thread-safe cache to
/// prevent duplicate file reads and optimize memory usage.
///
/// If the font fails to load, it will log an error and safely return `DEFAULT_FONT`.
pub fn get_font(path: impl AsRef<Path>) -> &'static [u8] {
    let path = path.as_ref();
    let cache = FONT_LIST.get_or_init(|| Mutex::new(HashMap::new()));

    let Ok(mut map) = cache.lock() else {
        error!("Failed to get font list.");
        return DEFAULT_FONT;
    };

    if let Some(font_bytes) = map.get(path) {
        return *font_bytes;
    }

    let Ok(bytes) = fs::read(path) else {
        error!("Failed to load font: {:?}", path);
        return DEFAULT_FONT;
    };

    let static_bytes = bytes.leak();

    map.insert(path.to_path_buf(), static_bytes);

    static_bytes
}
