use bevy::prelude::*;
use ratatui::prelude::Terminal;
use ratatui_wgpu::WgpuBackend;

/// The primary Bevy component that wraps a Ratatui terminal instance.
///
/// This component is automatically inserted into a window's `Entity` when
/// a new window is spawned. Use this to interface with Ratatui's drawing
/// commands in your systems.
#[derive(Component)]
pub struct RatatuiTerminal {
    /// The underlying Ratatui terminal using a static WGPU backend.
    pub terminal: Terminal<WgpuBackend<'static, 'static>>,
}
