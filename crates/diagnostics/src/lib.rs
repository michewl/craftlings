//! Implements diagnostics for the game

use bevy::app::PluginGroup;
use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;

pub struct DiagnosticsPluginGroup;

impl PluginGroup for DiagnosticsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>()
            .add(FrameTimeDiagnosticsPlugin)
            .add(SystemInformationDiagnosticsPlugin);

        // In debug mode, log the diagnostics to the console
        #[cfg(debug_assertions)]
        {
            use bevy::diagnostic::LogDiagnosticsPlugin;
            builder = builder.add(LogDiagnosticsPlugin::default());
        }

        builder
    }
}
