//! # Bevy Ratatui WGPU
//!
//! This crate provides a seamless integration between the Bevy game engine and the
//! Ratatui terminal UI library, utilizing a WGPU backend. It automatically manages
//! terminal creation, window resizing, and dynamic font loading within Bevy's
//! Entity Component System (ECS).
//!
//! ## Usage
//!
//! Include `BevyRatatuiMinimalPlugins` and `BevyRatatuiWgpuPlugin` in your Bevy app.
//! The terminal will be automatically attached to your window entities as a
//! `RatatuiTerminal` component.

/// Contains all the user facing components for the crate.
pub mod components;
/// Contains all the user facing messages for the crate.
pub mod messages;
/// Contains all the user facing resources for the crate.
pub mod resources;
pub(crate) mod systems;

mod bevy_ratatui_minimal_plugins;
pub use self::bevy_ratatui_minimal_plugins::*;

mod bevy_ratatui_wgpu_plugin;
pub use self::bevy_ratatui_wgpu_plugin::*;

mod fonts;
pub(crate) use self::fonts::*;

/// The prelude provides easy access to the most commonly used types in this crate.
pub mod prelude {
    pub use crate::components::RatatuiTerminal;

    pub use crate::messages::ChangeFont;

    pub use crate::resources::BevyRatatuiWgpuOptions;

    pub use crate::BevyRatatuiMinimalPlugins;
    pub use crate::BevyRatatuiWgpuPlugin;
}
