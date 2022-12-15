// 在发布模式下 windows系统，关闭命令行窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_rubikscube::core::flatten;
use bevy_rubikscube::viewer::{CreateCube, CubeSettings, MoveSequence, RandomPuzzle};
use bevy_rubikscube::{parser, BevyRubiksCubePlugin};
use std::io::Cursor;
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
    .add_startup_system(set_window_icon)
    .add_system(dashboard_ui);

    #[cfg(feature = "debug")]
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
    mut cube_settings: ResMut<CubeSettings>,
    mut move_seq: ResMut<MoveSequence>,
) {
    egui::Window::new("Dashboard").show(egui_context.ctx_mut(), |ui| {
        egui::ComboBox::from_label("Cubes")
            .selected_text(format!("{0}x{0}", cube_settings.cube_order))
            .show_ui(ui, |ui| {
                for i in 2..=10u8 {
                    if ui
                        .selectable_value(&mut cube_settings.cube_order, i, format!("{i}x{i}"))
                        .clicked()
                    {
                        create_ev.send(CreateCube::new(i));
                    }
                }
            });

        ui.separator();
        if ui.button("random").clicked() {
            rand_ev.send(RandomPuzzle);
        }
        egui::Grid::new("commands_grid")
            .striped(true)
            .spacing([20.0, 4.0])
            .show(ui, |ui| {
                for l in [
                    ["U", "L", "F", "R", "B", "D"],
                    ["U'", "L'", "F'", "R'", "B'", "D'"],
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
            })
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
