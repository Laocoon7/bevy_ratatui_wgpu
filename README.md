# BevyRatatuiWGPU

This project provides a seamless integration between the Bevy game engine and the Ratatui terminal UI library, utilizing a WGPU backend. It automatically manages terminal creation, window resizing, and dynamic font loading within Bevy's Entity Component System (ECS).

## Plugins

This crate provides two plugins:

### BevyRatatuiMinimalPlugins

- A customized Bevy plugin_group that replaces `DefaultPlugins` or `MinimalPlugins`. Add this first.

### BevyRatatuiWgpuPlugin

- The core plugin for BevyRatatuiWGPU.

## Features

- Window Creation

  - When a new window is created, Bevy Ratatui WGPU will create the terminal and insert a `RatatuiTerminal` component which holds the `ratatui::Terminal`.

  - See `BevyRatatuiWgpuOptions` for more options.

- Resizing
  - BevyRatatuiWGPU automatically handles resizing the terminal when the window resizes.

- Font Management

  - Default font is the monospaced: `JuliaMono-Regular`

  - Sending a `ChangeFont` message will cause the terminal to be automatically rebuilt with the new font.

  - Fonts are automatically cached by file path.

## Notes
- Be sure to turn off Bevy's default features and include only necessary features. Other features may work (e.g. `bevy_state`) Here is an example minimal feature set:

```toml
bevy = { version = "0.19", default-features = false, features = [
    "async_executor",
    "bevy_log",
    "bevy_winit",
    "bevy_window",
]}
```
