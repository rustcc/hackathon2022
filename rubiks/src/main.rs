// 在发布模式下 windows系统，关闭命令行窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::prelude::*;
use bevy_mod_picking::{
    DebugCursorPickingPlugin, DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle,
    PickingCameraBundle,
};
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, Intersection, RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem,
};
use bevy_rubikscube::core::{flatten, MyRaycastSet};
use bevy_rubikscube::viewer::{
    CreateCube, CubeSettings, MoveSequence, PlayMode, RandomPuzzle, SolvePuzzle, TimekeepingTimer,
};
use bevy_rubikscube::{parser, BevyRubiksCubePlugin};
use rubiks_solver::Cube;
use std::io::Cursor;
use std::time::Instant;
use winit::window::Icon;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Rubiks's Cube".to_string(),
            canvas: Some("#bevy".to_owned()),
            ..default()
        },
        ..default()
    }))
    .add_plugin(EguiPlugin)
    .add_plugin(BevyRubiksCubePlugin)
    // .add_plugin(WorldInspectorPlugin::new())
    .add_plugins(DefaultPickingPlugins)
    // .add_plugin(DebugCursorPickingPlugin)
    // .add_plugin(DebugEventsPickingPlugin)
    .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
    .add_startup_system(set_window_icon)
    .add_system(dashboard_ui);

    #[cfg(feature = "dev")]
    {
        app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());
    }

    app.run();
}

/// 调试UI
fn dashboard_ui(
    mut egui_context: ResMut<EguiContext>,
    mut create_ev: EventWriter<CreateCube>,
    mut rand_ev: EventWriter<RandomPuzzle>,
    mut solve_ev: EventWriter<SolvePuzzle>,
    mut cube_settings: ResMut<CubeSettings>,
    mut move_seq: ResMut<MoveSequence>,
    mut timekeeping_timer: ResMut<TimekeepingTimer>,
) {
    egui::Window::new("Dashboard").show(egui_context.ctx_mut(), |ui| {
        egui::ComboBox::from_label("Cubes")
            .selected_text(format!("{0}x{0}", cube_settings.cube.size()))
            .show_ui(ui, |ui| {
                for i in 2..=10i32 {
                    if ui
                        .selectable_value(&mut cube_settings.cube.size(), i, format!("{i}x{i}"))
                        .clicked()
                    {
                        create_ev.send(CreateCube::new(i));
                    }
                }
            });

        // ui.separator();
        egui::Grid::new("commands_grid")
            .striped(true)
            .spacing([20.0, 4.0])
            .show(ui, |ui| {
                ui.add(egui::Label::new("Commands:"));
                ui.end_row();
                for l in [
                    ["U", "L", "F", "R", "B", "D"],
                    ["U'", "L'", "F'", "R'", "B'", "D'"],
                    ["U2", "L2", "F2", "R2", "B2", "D2"],
                ] {
                    for c in l {
                        if ui.button(c).clicked() {
                            for c in flatten(parser::parse(c).unwrap().1).into_iter() {
                                move_seq.push_back(c)
                            }
                        }
                    }
                    ui.end_row();
                }
            });

        ui.separator();
        egui::Grid::new("ui_grid")
            .num_columns(2)
            .spacing([10.0, 20.0])
            .striped(true)
            .show(ui, |ui| {
                ui.add(egui::Label::new("Rotate Speed"));
                ui.add(egui::Slider::new(
                    &mut cube_settings.rotate_speed,
                    0.1..=10.0,
                ));
                ui.end_row();

                ui.add(egui::Label::new("Play Mode"));
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut cube_settings.play_mode,
                        PlayMode::Practice,
                        "Practice",
                    );
                    if ui
                        .selectable_value(
                            &mut cube_settings.play_mode,
                            PlayMode::Timekeeping,
                            "Timekeeping",
                        )
                        .clicked()
                    {
                        // 重置计时器
                        timekeeping_timer.0 = Instant::now();
                    }
                });
                if cube_settings.play_mode == PlayMode::Timekeeping {
                    ui.add(egui::Label::new(format!(
                        "{}s",
                        timekeeping_timer.0.elapsed().as_secs()
                    )));
                }
                ui.end_row();

                if ui
                    .add_sized([100.0, 30.0], egui::Button::new("Scramble"))
                    .clicked()
                {
                    rand_ev.send(RandomPuzzle);
                }

                if cube_settings.cube.size() == 3 {
                    if ui
                        .add_sized([100.0, 30.0], egui::Button::new("Solver"))
                        .clicked()
                    {
                        solve_ev.send(SolvePuzzle);
                    }
                }

                ui.end_row();
            });
    });
}

/// 设置windows图标
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
