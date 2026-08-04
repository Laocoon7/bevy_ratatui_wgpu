//! A basic example demonstrating how to integrate Ratatui with Bevy using `bevy_ratatui_wgpu`.
//!
//! This example shows how to set up the minimal plugins, add the WGPU plugin,
//! and spawn and draw to multiple windows.

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ratatui_wgpu::prelude::*;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

#[derive(Component)]
#[require(Window)]
struct SecondaryWindow;

fn main() {
    let mut app = App::new();

    app.add_plugins(BevyRatatuiMinimalPlugins);
    app.add_plugins(BevyRatatuiWgpuPlugin);

    app.add_systems(Startup, spawn_second_window);
    app.add_systems(PostUpdate, draw_primary_window);
    app.add_systems(PostUpdate, draw_secondary_window);

    app.run();
}

fn spawn_second_window(mut commands: Commands) {
    commands.spawn(SecondaryWindow);
}

fn draw_primary_window(
    primary_window: Single<Entity, With<PrimaryWindow>>,
    mut q_terminals: Query<&mut RatatuiTerminal>,
) {
    let primary_window = primary_window.into_inner();

    if let Ok(mut ratatui_terminal) = q_terminals.get_mut(primary_window) {
        ratatui_terminal
            .terminal
            .draw(|f| {
                f.render_widget(
                    Paragraph::new("Primary Window").red().block(Block::bordered().title("Primary").black()),
                    f.area(),
                )
            })
            .unwrap();
    }
}

fn draw_secondary_window(
    secondary_window: Single<Entity, With<SecondaryWindow>>,
    mut q_terminals: Query<&mut RatatuiTerminal>,
) {
    let secondary_window = secondary_window.into_inner();

    if let Ok(mut ratatui_terminal) = q_terminals.get_mut(secondary_window) {
        ratatui_terminal
            .terminal
            .draw(|f| {
                f.render_widget(
                    Paragraph::new("Secondary Window")
                        .green()
                        .block(Block::bordered().title("Secondary").black()),
                    f.area(),
                )
            })
            .unwrap();
    }
}
