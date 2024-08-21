mod plugin;

use bevy::{
    app::ScheduleRunnerPlugin,
    diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use plugin::{io::IoPlugin, log::LogPlugin};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_micros(16670))),
            LogPlugin,
            DiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
            HierarchyPlugin,
            IoPlugin,
        ))
        .run();
}
