//! A basic example demonstrating how to integrate Ratatui with Bevy using `bevy_ratatui_wgpu`.
//!
//! This example shows how to set up the minimal plugins, add the WGPU plugin,
//! and render a simple bordered paragraph widget to the terminal.

use bevy::prelude::*;
use bevy_ratatui_wgpu::prelude::*;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

fn main() {
    let mut app = App::new();

    app.add_plugins(BevyRatatuiMinimalPlugins);
    app.add_plugins(BevyRatatuiWgpuPlugin);

    app.add_systems(PostUpdate, draw_windows);

    app.run();
}

fn draw_windows(mut q_terminals: Query<&mut RatatuiTerminal>) {
    for mut window in q_terminals.iter_mut() {
        window
            .terminal
            .draw(|f| {
                f.render_widget(
                    Paragraph::new("Test").red().block(Block::bordered().title("Title").black()),
                    f.area(),
                );
            })
            .unwrap();
    }
}
