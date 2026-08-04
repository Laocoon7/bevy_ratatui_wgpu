use std::{num::NonZeroU32, path::PathBuf};

use bevy::{ecs::system::NonSendMarker, prelude::*, window::RawHandleWrapper, winit::WINIT_WINDOWS};
use ratatui::prelude::*;
use ratatui_wgpu::{Builder, Dimensions, Font};

use crate::{components::RatatuiTerminal, get_font};

/// A message sent to dynamically change the font of an existing terminal window.
#[derive(Message)]
pub struct ChangeFont {
    /// The entity of the window to update.
    pub window: Entity,
    /// The path to the new font file to load.
    pub font: PathBuf,
}

/// Intercepts `ChangeFont` messages, rebuilds the WGPU target with the new font,
/// and updates the `RatatuiTerminal` component attached to the specified window.
pub fn handle_change_font(
    mut m_change_font: MessageReader<ChangeFont>,
    // winit_windows: NonSend<WinitWindows>,
    q_windows: Query<&RawHandleWrapper>,
    mut q_terminals: Query<&mut RatatuiTerminal>,
    _non_send_marker: NonSendMarker, // no NonSend<> resources in 0.19, must build backend on main thread
) {
    WINIT_WINDOWS.with_borrow(|winit_windows| {
        for message in m_change_font.read() {
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
            let Some(font) = Font::new(get_font(&message.font)) else {
                warn!("Failed to build font.");
                continue;
            };

            // Wait to get the backend
            let build_result = bevy::tasks::block_on(async {
                Builder::from_font(font)
                    .with_width_and_height(Dimensions {
                        width: NonZeroU32::new(size.width.max(1)).unwrap(), // Safety: we use .max(1) to guarantee non-zero
                        height: NonZeroU32::new(size.height.max(1)).unwrap(), // Safety: we use .max(1) to guarantee non-zero
                    })
                    .build_with_target(surface_target)
                    .await
            });

            // Build the terminal
            match build_result {
                Ok(backend) => match Terminal::new(backend) {
                    Ok(terminal) => {
                        let Ok(mut window) = q_terminals.get_mut(message.window) else {
                            warn!("No RatatuiWindow: {}", message.window);
                            continue;
                        };
                        window.terminal = terminal;
                    },
                    Err(e) => error!("{e}"),
                },
                Err(e) => error!("{e}"),
            }
        }
    })
}
