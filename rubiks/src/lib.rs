use bevy::app::{App, Plugin};

use crate::camera::CameraPlugin;
use crate::viewer::ViewerPlugin;

pub mod camera;
pub mod core;
pub mod viewer;

/// 魔方插件
pub struct BevyRubiksCubePlugin;

impl Plugin for BevyRubiksCubePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin).add_plugin(ViewerPlugin);
    }
}
