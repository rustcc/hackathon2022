use crate::core::MyRaycastSet;
use bevy::prelude::*;
use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::{Camera3dBundle, Commands, Transform};
use bevy_mod_picking::PickingCameraBundle;
use bevy_mod_raycast::{RaycastSource, RaycastMethod, RaycastSystem, Intersection};

/// 相机插件
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup)
        // .add_system(intersection)
        .add_system_to_stage(
            CoreStage::First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.5, 5.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(PickingCameraBundle::default())
    .insert(RaycastSource::<MyRaycastSet>::new());
}

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<MyRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

fn intersection(query: Query<&Intersection<MyRaycastSet>>) {
    for intersection in &query {
        info!(
            "Distance {:?}, Position {:?}",
            intersection.distance(),
            intersection.position()
        );
    }
}