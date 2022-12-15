use crate::camera::CameraPlugin;
use bevy::app::{App, Plugin};

pub mod camera;

pub struct BevyRubiksCubePlugin;

impl Plugin for BevyRubiksCubePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin);
    }
}
