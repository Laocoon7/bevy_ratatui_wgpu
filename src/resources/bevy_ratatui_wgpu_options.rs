use bevy::prelude::*;
use ratatui::prelude::Color;
use ratatui_wgpu::{Font, wgpu::PresentMode};

/// Configuration options resource for the Ratatui WGPU rendering backend in Bevy.
///
/// This resource configures WGPU presentation behavior, default colors for
/// terminal cells, and timing intervals for blinking effects.
#[derive(Resource)]
pub struct BevyRatatuiWgpuOptions {
    /// The default font to use for new windows.
    ///
    /// Defaults to embedded `JuliaMono-Regular`.
    pub default_font: Option<Font<'static>>,
    /// Use the specified font size in pixels.
    ///
    /// Defaults to `24px`.
    pub font_size_px: u32,
    /// Use the specified list of fonts for rendering. This will
    /// automatically organize fonts by relative width in order
    /// to optimize fallback rendering quality
    pub additional_fonts: Vec<Font<'static>>,
    /// Use the specified list of regular fonts for rendering.
    pub additional_regular_fonts: Vec<Font<'static>>,
    /// Use the specified list of bold fonts for rendering.
    pub additional_bold_fonts: Vec<Font<'static>>,
    /// Use the specified list of italic fonts for rendering.
    pub additional_italic_fonts: Vec<Font<'static>>,
    /// Use the specified list of bold italic fonts for rendering.
    pub additional_bold_italic_fonts: Vec<Font<'static>>,
    /// The WGPU presentation mode (e.g., VSync/`Fifo`, `Immediate`, or `Mailbox`).
    ///
    /// If set to `None`, the backend defers to the surface default configuration.
    pub present_mode: Option<PresentMode>,
    /// The default foreground color used when resetting cell attributes or rendering
    /// unstyled text.
    ///
    /// Defaults to [`Color::White`].
    pub reset_fg: Color,
    /// The default background color used when resetting cell attributes or rendering
    /// unstyled text.
    ///
    /// Defaults to [`Color::Black`].
    pub reset_bg: Color,
    /// Cycle milliseconds elements set to fast-blink.
    ///
    /// Defaults to `200ms`.
    pub fast_blink: u64,
    /// Cycle milliseconds elements set to slow-blink.
    ///
    /// Defaults to `1000ms` (1 second).
    pub slow_blink: u64,
}

impl Default for BevyRatatuiWgpuOptions {
    fn default() -> Self {
        Self {
            default_font: None,
            font_size_px: 24,
            additional_fonts: Vec::new(),
            additional_regular_fonts: Vec::new(),
            additional_bold_fonts: Vec::new(),
            additional_italic_fonts: Vec::new(),
            additional_bold_italic_fonts: Vec::new(),
            present_mode: None,
            reset_fg: Color::White,
            reset_bg: Color::Black,
            fast_blink: 200,
            slow_blink: 1000,
        }
    }
}
