use bevy::{prelude::*, window::WindowResized};

use crate::components::RatatuiTerminal;

/// Listens for `WindowResized` messages and scales the underlying WGPU terminal backend
/// appropriately. Clamps dimensions to a minimum of 1 to prevent WGPU crashes.
pub fn on_window_resized(
    mut m_resize: MessageReader<WindowResized>,
    mut q_terminals: Query<&mut RatatuiTerminal>,
) {
    for message in m_resize.read() {
        let Ok(mut window) = q_terminals.get_mut(message.window) else {
            continue;
        };

        let new_width = message.width.max(1.0) as u32;
        let new_height = message.height.max(1.0) as u32;

        window.terminal.backend_mut().resize(new_width, new_height);
    }
}
