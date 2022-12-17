use crate::core::{random_command, Command, Piece, Surface, BaseMove, MyRaycastSet};
use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingEvent, PickingCameraBundle, DefaultPickingPlugins, DebugCursorPickingPlugin, DebugEventsPickingPlugin};
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastSource, RaycastMesh, Intersection, RaycastMethod, RaycastSystem};
use std::collections::VecDeque;
use std::f32::consts::{FRAC_PI_2, PI};

/// 可视化插件
pub struct ViewerPlugin;

impl Plugin for ViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateCube>()
            .add_event::<UpdateSurface>()
            .add_event::<RandomPuzzle>()
            .init_resource::<CubeSettings>()
            .init_resource::<MoveSequence>()
            .init_resource::<ExecutingCommand>()
            .insert_resource(MouseDraggingRecorder { start_pos: None, piece: None })
            .register_type::<Piece>()
            .add_startup_system(setup)
            .add_system(create_cube_event)
            .add_system(move_piece)
            .add_system(random_puzzle)
            .add_system(mouse_dragging)
            .add_system_to_stage(CoreStage::PostUpdate, update_surface);
    }
}

/// 魔方设置
#[derive(Debug, Resource)]
pub struct CubeSettings {
    /// 是几阶的魔方
    pub cube_order: u8,
    /// 块的大小
    pub piece_size: f32,
    /// 前面的颜色
    pub front_color: Color,
    /// 后面的颜色
    pub back_color: Color,
    /// 左面的颜色
    pub left_color: Color,
    /// 右面的颜色
    pub right_color: Color,
    /// 上面的颜色
    pub top_color: Color,
    /// 下面的颜色
    pub bottom_color: Color,
    /// 旋转速度
    pub rotate_speed: f32,
}

impl Default for CubeSettings {
    fn default() -> Self {
        Self {
            cube_order: 3,
            piece_size: 1.0,
            front_color: Color::GREEN,
            back_color: Color::BLUE,
            left_color: Color::ORANGE,
            right_color: Color::RED,
            top_color: Color::WHITE,
            bottom_color: Color::YELLOW,
            rotate_speed: 1.0
        }
    }
}

/// 旋转魔方的队列
#[derive(Deref, DerefMut, Resource, Default)]
pub struct MoveSequence(pub VecDeque<Command>);

/// 正在执行的command
#[derive(Resource)]
pub struct ExecutingCommand {
    pub command: Command,
    // 剩余待旋转的弧度
    pub left_angle: f32,
}

impl Default for ExecutingCommand {
    fn default() -> Self {
        Self { command: Command(BaseMove::U, 0), left_angle: 0.0 }
    }
}

/// 生成魔方事件
#[derive(Debug)]
pub struct CreateCube {
    number: u8,
}

impl Default for CreateCube {
    fn default() -> Self {
        Self { number: 3 }
    }
}

impl CreateCube {
    pub fn new(number: u8) -> Self {
        assert!((2..=10).contains(&number));
        Self { number }
    }
}

/// 随机打乱魔方
pub struct RandomPuzzle;

/// 更新索引
pub struct UpdateSurface;

#[derive(Debug, Resource)]
pub struct MouseDraggingRecorder {
    pub start_pos: Option<Vec3>,
    pub piece: Option<Entity>,
}

impl MouseDraggingRecorder {
    pub fn clear(&mut self) {
        self.start_pos = None;
        self.piece = None;
    }
}

/// 先清除之前的魔方， 再生成新的魔方
fn create_cube_event(
    mut commands: Commands,
    mut create_ev: EventReader<CreateCube>,
    mut update_ev: EventWriter<UpdateSurface>,
    q_old_cubes: Query<Entity, With<Piece>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cube_settings: ResMut<CubeSettings>,
) {
    for ev in create_ev.iter() {
        for entity in q_old_cubes.iter() {
            commands.entity(entity).despawn_recursive();
        }
        cube_settings.cube_order = ev.number;
        let order = cube_settings.cube_order;
        let size = cube_settings.piece_size;
        let border = (order as f32 * size) / 2.0 - 0.5;

        // 生成魔方
        for x in 0..order {
            for z in 0..order {
                for y in 0..order {
                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size })),
                            material: materials.add(StandardMaterial {
                                base_color: Color::BLACK,
                                alpha_mode: AlphaMode::Blend,
                                unlit: true,
                                ..default()
                            }),
                            transform: Transform::from_xyz(
                                x as f32 - border,
                                y as f32 - border,
                                z as f32 - border,
                            ),
                            ..Default::default()
                        })
                        .insert(Piece::new(order, x, y, z))
                        .insert(PickableBundle::default())
                        .insert(RaycastMesh::<MyRaycastSet>::default())
                        .with_children(|parent| {
                            // 生成顶部
                            if y == (order - 1) {
                                spawn_piece(
                                    parent,
                                    &mut meshes,
                                    &mut materials,
                                    &cube_settings,
                                    Surface::U,
                                );
                            }

                            // 生成底部
                            if y == 0 {
                                spawn_piece(
                                    parent,
                                    &mut meshes,
                                    &mut materials,
                                    &cube_settings,
                                    Surface::D,
                                );
                            }

                            // 生成左侧
                            if x == 0 {
                                spawn_piece(
                                    parent,
                                    &mut meshes,
                                    &mut materials,
                                    &cube_settings,
                                    Surface::L,
                                );
                            }

                            // 生成右侧
                            if x == (order - 1) {
                                spawn_piece(
                                    parent,
                                    &mut meshes,
                                    &mut materials,
                                    &cube_settings,
                                    Surface::R,
                                );
                            }

                            // 生成前部
                            if z == (order - 1) {
                                spawn_piece(
                                    parent,
                                    &mut meshes,
                                    &mut materials,
                                    &cube_settings,
                                    Surface::F,
                                );
                            }

                            // 生成后部
                            if z == 0 {
                                spawn_piece(
                                    parent,
                                    &mut meshes,
                                    &mut materials,
                                    &cube_settings,
                                    Surface::B,
                                );
                            }
                        });
                }
            }
        }

        update_ev.send(UpdateSurface);
    }
}

/// 创建块的辅助方法
fn spawn_piece(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cube_settings: &CubeSettings,
    surface: Surface,
) -> Entity {
    let size = cube_settings.piece_size;
    // 贴纸颜色的大小， 比块小一点
    let square_size = cube_settings.piece_size * 0.9;
    let (color, pos) = match surface {
        Surface::U => {
            // 上面
            (
                cube_settings.top_color,
                Transform::from_xyz(0.0, size * 0.5 + 0.01, 0.0),
            )
        }
        Surface::D => {
            // 下面
            let mut pos = Transform::from_xyz(0.0, -size * 0.5 - 0.01, 0.0);
            pos.rotate_x(PI);
            (cube_settings.bottom_color, pos)
        }
        Surface::L => {
            // 左面
            let mut pos = Transform::from_xyz(-size * 0.5 - 0.01, 0.0, 0.0);
            pos.rotate_z(FRAC_PI_2);
            (cube_settings.left_color, pos)
        }
        Surface::R => {
            // 右面
            let mut pos = Transform::from_xyz(size * 0.5 + 0.01, 0.0, 0.0);
            pos.rotate_z(-FRAC_PI_2);
            (cube_settings.right_color, pos)
        }
        Surface::F => {
            // 前面
            let mut pos = Transform::from_xyz(0.0, 0.0, size * 0.5 + 0.01);
            pos.rotate_x(FRAC_PI_2);
            (cube_settings.front_color, pos)
        }
        Surface::B => {
            // 后面
            let mut pos = Transform::from_xyz(0.0, 0.0, -size * 0.5 - 0.01);
            pos.rotate_x(-FRAC_PI_2);
            (cube_settings.back_color, pos)
        }
    };
    parent
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: square_size })),
            material: materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                ..default()
            }),
            transform: pos,
            ..Default::default()
        })
        .insert(surface)
        .id()
}

/// 旋转魔方
fn move_piece(
    mut move_seq: ResMut<MoveSequence>,
    mut executing_cmd: ResMut<ExecutingCommand>,
    mut q_pieces: Query<(&mut Transform, &Piece)>,
    mut update_ev: EventWriter<UpdateSurface>,
    cube_settings: Res<CubeSettings>,
    time: Res<Time>,
) {
    if executing_cmd.left_angle == 0.0 {
        update_ev.send(UpdateSurface);
        // 读取下一个指令
        if let Some(command) = move_seq.pop_front() {
            info!("command: {}", command);
            executing_cmd.command = command;
            executing_cmd.left_angle = command.angle();
        }
    } else {
        let clockwise = executing_cmd.command.clockwise();
        let mut angle = match clockwise {
            true => { cube_settings.rotate_speed * PI * time.delta_seconds() },
            false => { -cube_settings.rotate_speed * PI * time.delta_seconds() },
        };
        let left_angle = executing_cmd.left_angle;
        let mut new_left_angle = left_angle - angle;
        // 判断left_angle是否足够支持一帧旋转
        if clockwise {
            if new_left_angle < 0.0 {
                angle = left_angle;
                new_left_angle = 0.0;
            }
        } else {
            if new_left_angle > 0.0 {
                angle = left_angle;
                new_left_angle = 0.0;
            }
        }

        let quat = Quat::from_axis_angle(executing_cmd.command.axis(), angle);

        for (mut transform, piece) in q_pieces.iter_mut() {
            if piece.is_selected(&executing_cmd.command) {
                transform.rotate_around(Vec3::ZERO, quat);
            }
        }
        // 更新left_angle
        executing_cmd.left_angle = new_left_angle;
    }
}

fn random_puzzle(
    mut ev: EventReader<RandomPuzzle>,
    mut cmd_ev: ResMut<MoveSequence>,
    cube_settings: Res<CubeSettings>,
) {
    for _ in ev.iter() {
        let cmds = random_command(cube_settings.cube_order as usize);
        for command in cmds {
            cmd_ev.push_back(command);
        }
    }
}

/// 通过检查块的空间坐标，判断块的面
fn update_surface(
    mut update_ev: EventReader<UpdateSurface>,
    mut q_plane: Query<(&Transform, &mut Piece)>,
    cube_settings: Res<CubeSettings>,
) {
    for _ in update_ev.iter() {
        let order = cube_settings.cube_order;
        let size = cube_settings.piece_size;
        let border = (order as f32 * size) / 2.0 - 0.5;
        for (transform, mut piece) in q_plane.iter_mut() {
            piece.x = (transform.translation.x.round() + border) as u8;
            piece.y = (transform.translation.y.round() + border) as u8;
            piece.z = (transform.translation.z.round() + border) as u8;
        }
    }
}

fn setup(mut create_ev: EventWriter<CreateCube>) {
    create_ev.send(CreateCube::new(3));
}

pub fn mouse_dragging(
    mut recorder: ResMut<MouseDraggingRecorder>,
    mouse: Res<Input<MouseButton>>,
    mut picking_events: EventReader<PickingEvent>,
    q_pieces: Query<&Transform, With<Piece>>,
    q_intersection: Query<&Intersection<MyRaycastSet>>,
    mut move_seq: ResMut<MoveSequence>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // recorder开始记录
        if let Some(event) = picking_events
            .iter()
            .filter(|e| match e {
                PickingEvent::Clicked(_) => true,
                _ => false,
            })
            .last()
        {
            let piece_entity = match event {
                PickingEvent::Clicked(entity) => entity,
                _ => {
                    unreachable!();
                }
            };
            recorder.piece = Some(piece_entity.clone());

            if let Some(intersection) = q_intersection.iter().last() {
                recorder.start_pos = Some(intersection.position().unwrap().clone());
            } else {
                panic!("Can not get start pos");
            }

            info!("MouseDraggingRecorder started {:?}", recorder);
        }
    }

    if mouse.pressed(MouseButton::Left) {
        if recorder.start_pos.is_some() && recorder.piece.is_some() {
            if let Some(intersection) = q_intersection.iter().last() {
                // 鼠标拽动距离超过临界值
                if recorder
                    .start_pos
                    .unwrap()
                    .distance(intersection.position().unwrap().clone())
                    > 0.5
                {
                    // 触发旋转
                    info!(
                        "Trigger side move event, end_pos: {:?}",
                        &intersection.position()
                    );
                    let translation = q_pieces.get(recorder.piece.unwrap()).unwrap().translation;
                    let command = generate_command(
                        translation,
                        recorder.start_pos.unwrap(),
                        intersection.position().unwrap().clone(),
                    );
                    info!("generate command: {:?}, piece trans: {}", command, translation);
                    move_seq.0.push_back(command);


                    // 清除recorder
                    recorder.clear();
                }
            } else {
                panic!("Can not get end pos");
            }
        }
    }

    if mouse.just_released(MouseButton::Left) {
        // 清除recorder
        recorder.clear();
    }
}

fn generate_command(piece_translation: Vec3, start_pos: Vec3, end_pos: Vec3) -> Command {
    // TODO 1.5 surface的坐标值
    if (start_pos.x.abs() - 1.5).abs() < 0.001 {
        let delta_y = end_pos.y - start_pos.y;
        let delta_z = end_pos.z - start_pos.z;
        if delta_y.abs() > delta_z.abs() {
            // y轴变化大，沿z轴旋转
            let rotate = if delta_y > 0.0 { 1 } else { -1 };
            if piece_translation.z.round() == -1.0 {
                return Command(BaseMove::B, rotate);
            } else if piece_translation.z.round() == 0.0 {
                return Command(BaseMove::S, rotate);
            } else {
                return Command(BaseMove::F, rotate);
            }
        } else {
            // z轴变化大，沿y轴旋转
            let rotate = if delta_z > 0.0 { -1 } else { 1 };
            if piece_translation.y.round() == -1.0 {
                return Command(BaseMove::D, rotate);
            } else if piece_translation.y.round() == 0.0 {
                return Command(BaseMove::E, rotate);
            } else {
                return Command(BaseMove::U, rotate);
            }
        }
    } else if (start_pos.y.abs() - 1.5).abs() < 0.001 {
        let delta_x = end_pos.x - start_pos.x;
        let delta_z = end_pos.z - start_pos.z;
        if delta_x.abs() > delta_z.abs() {
            // x轴变化大，沿z轴旋转
            let rotate = if delta_x > 0.0 { -1 } else { 1 };
            if piece_translation.z.round() == -1.0 {
                return Command(BaseMove::B, rotate);
            } else if piece_translation.z.round() == 0.0 {
                return Command(BaseMove::S, rotate);
            } else {
                return Command(BaseMove::F, rotate);
            }
        } else {
            // z轴变化大，沿x轴旋转
            let rotate = if delta_z > 0.0 { 1 } else { -1 };
            if piece_translation.x.round() == -1.0 {
                return Command(BaseMove::L, rotate);
            } else if piece_translation.x.round() == 0.0 {
                return Command(BaseMove::M, rotate);
            } else {
                return Command(BaseMove::R, rotate);
            }
        }
    } else {
        let delta_x = end_pos.x - start_pos.x;
        let delta_y = end_pos.y - start_pos.y;
        if delta_x.abs() > delta_y.abs() {
            // x轴变化大，沿y轴旋转
            let rotate = if delta_x > 0.0 { 1 } else { -1 };
            if piece_translation.y.round() == -1.0 {
                return Command(BaseMove::D, rotate);
            } else if piece_translation.y.round() == 0.0 {
                return Command(BaseMove::E, rotate);
            } else {
                return Command(BaseMove::U, rotate);
            }
        } else {
            // y轴变化大，沿x轴旋转
            let rotate = if delta_y > 0.0 { -1 } else { 1 };
            if piece_translation.x.round() == -1.0 {
                return Command(BaseMove::L, rotate);
            } else if piece_translation.x.round() == 0.0 {
                return Command(BaseMove::M, rotate);
            } else {
                return Command(BaseMove::R, rotate);
            }
        }
    }
}