use bevy::DefaultPlugins;
use bevy::app::App;
use bevy::log::LogPlugin;
use bevy::log::info;
use bevy::log::warn;
use bevy::prelude::PluginGroup;
use bevy::window::Window;
use bevy::window::WindowPlugin;
use craftlings_core::constants::GAME_VERSION;
use craftlings_diagnostics::DiagnosticsPluginGroup;

fn main() {
    // In debug mode, load the environment variables from `.env` if the file exists
    #[cfg(debug_assertions)]
    if let Err(e) = dotenv::dotenv() {
        warn!("The '.env' file could not be loaded.\n{e}");
    }

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                ..Default::default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: format!("Craftlings v{}", GAME_VERSION),
                    ..Default::default()
                }),
                ..Default::default()
            }),
    )
    .add_plugins(DiagnosticsPluginGroup);

    info!("Running Craftlings v{}", GAME_VERSION);

    app.run();
}
