use std::num::NonZeroU32;

use bevy::{
    ecs::system::NonSendMarker,
    prelude::*,
    window::{RawHandleWrapper, WindowCreated},
    winit::WINIT_WINDOWS,
};
use ratatui::prelude::Terminal;
use ratatui_wgpu::{Builder, Dimensions, Font};

use crate::{DEFAULT_FONT, components::RatatuiTerminal, resources::BevyRatatuiWgpuOptions};

/// Listens for `WindowCreated` messages, fetches the window's raw handle and size,
/// initializes the WGPU terminal backend with the default font, and inserts
/// the `RatatuiTerminal` component into the window's entity.
pub fn on_window_created(
    mut m_window_created: MessageReader<WindowCreated>,
    mut commands: Commands,
    // winit_windows: NonSend<WinitWindows>,
    options: Res<BevyRatatuiWgpuOptions>,
    q_windows: Query<&RawHandleWrapper>,
    _non_send_marker: NonSendMarker, // no NonSend<> resources in 0.19, must build backend on main thread
) {
    WINIT_WINDOWS.with_borrow(|winit_windows| {
        for message in m_window_created.read() {
            // Get the surface target for the window
            let Ok(raw_handle_wrapper) = q_windows.get(message.window) else {
                warn!("WindowCreated event for Window without RawHandleWrapper");
                continue;
            };
            let surface_target = unsafe { raw_handle_wrapper.get_handle() }; // Safety: we MUST pass the `HasWindowHandle` `HasDisplayHandle` implementation

            // Get the window size
            let Some(winit_window) = winit_windows.get_window(message.window) else {
                warn!("Failed to find winit window for window: {}", message.window);
                continue;
            };
            let size = winit_window.inner_size();

            // Build the font
            let font = if let Some(font) = options.default_font.clone() {
                font
            } else {
                let Some(font) = Font::new(DEFAULT_FONT) else {
                    warn!("Failed to build font.");
                    continue;
                };
                font
            };

            // Wait to get the backend
            let build_result = bevy::tasks::block_on(async {
                let mut builder = Builder::from_font(font)
                    .with_font_size_px(options.font_size_px)
                    .with_fonts(options.additional_fonts.clone())
                    .with_regular_fonts(options.additional_regular_fonts.clone())
                    .with_bold_fonts(options.additional_bold_fonts.clone())
                    .with_italic_fonts(options.additional_italic_fonts.clone())
                    .with_bold_italic_fonts(options.additional_bold_italic_fonts.clone())
                    .with_width_and_height(Dimensions {
                        width: NonZeroU32::new(size.width.max(1)).unwrap(), // Safety: we use .max(1) to guarantee non-zero
                        height: NonZeroU32::new(size.height.max(1)).unwrap(), // Safety: we use .max(1) to guarantee non-zero
                    })
                    .with_fg_color(options.reset_fg)
                    .with_bg_color(options.reset_bg)
                    .with_rapid_blink_millis(options.fast_blink)
                    .with_slow_blink_millis(options.slow_blink);

                if let Some(mode) = options.present_mode {
                    builder = builder.with_present_mode(mode);
                }

                builder.build_with_target(surface_target).await
            });

            // Build the terminal
            match build_result {
                Ok(backend) => match Terminal::new(backend) {
                    Ok(terminal) => {
                        commands.entity(message.window).insert(RatatuiTerminal { terminal });
                    },
                    Err(e) => error!("{e}"),
                },
                Err(e) => error!("{e}"),
            }
        }
    });
}
