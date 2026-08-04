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

- Resizing
  - BevyRatatuiWGPU automatically handles resizing the terminal when the window resizes.

- Font Management

  - Default font is the monospaced: `JuliaMono-Regular`

  - Sending a `ChangeFont` message will cause the terminal to be automatically rebuilt with the new font.

  - Fonts are automatically cached by file path.
