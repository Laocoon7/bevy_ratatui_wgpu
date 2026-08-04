use bevy::prelude::*;

use crate::systems::{on_window_created, on_window_resized};

/// The core plugin that integrates Ratatui into Bevy via WGPU.
///
/// This plugin ensures the terminal backend is built and scaled
/// automatically alongside Bevy's window lifecycle.
#[derive(Default)]
pub struct BevyRatatuiWgpuPlugin;
impl Plugin for BevyRatatuiWgpuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (on_window_created, on_window_resized));
    }
}
