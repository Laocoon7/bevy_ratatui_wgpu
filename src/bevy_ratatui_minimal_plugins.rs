use bevy::app::plugin_group;

plugin_group! {
    /// A customized Bevy `plugin_group` that includes only the essential plugins
    /// required for a terminal application.
    ///
    /// This crate is meant to use Bevy as a front end window manager only: NOT a renderer.
    /// While adding further bevy plugins (e.g. AssetPlugin, StatePlugin) is possible
    /// realize the terminal assumes it has full control of the window/rendering.
    pub struct BevyRatatuiMinimalPlugins {
        bevy::log:::LogPlugin,
        bevy::app:::TaskPoolPlugin,
        bevy::diagnostic:::FrameCountPlugin,
        bevy::time:::TimePlugin,
        bevy::input:::InputPlugin,
        bevy::window:::WindowPlugin,
        bevy::a11y:::AccessibilityPlugin,
        bevy::winit:::WinitPlugin,
    }
}
