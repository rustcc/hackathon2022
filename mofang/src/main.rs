use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_rubikscube::BevyRubiksCubePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Rubiks's Cube".to_string(),
            ..default()
        },
        ..default()
    }))
    .add_plugin(EguiPlugin)
    .add_plugin(BevyRubiksCubePlugin);

    #[cfg(feature = "debug")]
    {
        app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());
    }

    app.run();
}
