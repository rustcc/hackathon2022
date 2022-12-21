use crate::GENERAL_SIZE;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone)]
pub struct Wall;

#[derive(Component, Clone)]
pub struct CrabPot;

#[derive(Component, Clone)]
pub struct Stone;

#[derive(Component)]
pub struct ShowText;

#[derive(Resource)]
pub struct Follow {
    pub vec: Vec<GameStage>,
    // 当前渲染关卡
    pub current_episode: usize,
    // 当前应渲染关卡
    pub legal_episode: usize,
}

#[derive(Resource)]
pub struct ImageResource {
    player: Handle<Image>,
    wall: Handle<Image>,
    stone: Handle<Image>,
    crab_pot: Handle<Image>,
    pre: Handle<Image>,
    end: Handle<Image>,
}

pub fn init_image_resource(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_textures = ImageResource {
        player: asset_server.load("picture/ferris.png"),
        wall: asset_server.load("picture/coral.png"),
        stone: asset_server.load("picture/stone.png"),
        crab_pot: asset_server.load("picture/crab_pot.png"),
        pre: asset_server.load("picture/pre.png"),
        end: asset_server.load("picture/end.png"),
    };
    commands.insert_resource(game_textures);
}

impl Follow {
    pub fn init_follow() -> Self {
        Self {
            vec: vec![
                GameStage::Pre(Episode {
                    player_position: Vec3::new(-0., 0., 0.),
                    wall_vec: coordinate_to_vec3(vec![
                        (-3, 1),
                        (-2, 1),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                        (2, 1),
                        (3, 1),
                        (-3, 0),
                        (-3, -1),
                        (-2, -1),
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                        (2, -1),
                        (3, -1),
                        (3, 0),
                    ]),
                    crab_pot_vec: coordinate_to_vec3(vec![(1, 0)]),
                    stone_vec: coordinate_to_vec3(vec![(2, 0)]),
                }),
                GameStage::Gaming(Episode {
                    player_position: Vec3::new(0., 0., 0.),
                    wall_vec: coordinate_to_vec3(vec![
                        (-4, -2),
                        (-4, -3),
                        (-5, -4),
                        (-4, -5),
                        (-3, -2),
                        (-3, -5),
                        (-3, -6),
                        (-2, 0),
                        (-2, -1),
                        (-2, -6),
                        (-1, 1),
                        (-1, 0),
                        (-1, -5),
                        (-1, -6),
                        (0, 1),
                        (0, -4),
                        (0, -5),
                        (1, 1),
                        (1, -3),
                        (1, -4),
                        (2, 1),
                        (2, -1),
                        (2, -2),
                        (2, -3),
                        (3, 0),
                    ]),
                    crab_pot_vec: coordinate_to_vec3(vec![
                        (1, 0),
                        (1, -1),
                        (0, -1),
                        (-1, -2),
                        (0, -2),
                        (-2, -3),
                        (-1, -3),
                        (-3, -4),
                        (-2, -4),
                    ]),
                    stone_vec: coordinate_to_vec3(vec![
                        (2, 0),
                        (-1, -1),
                        (-2, -2),
                        (1, -2),
                        (-3, -3),
                        (0, -3),
                        (-4, -4),
                        (-1, -4),
                        (-2, -5),
                    ]),
                }),
                GameStage::Gaming(Episode {
                    player_position: Vec3::new(0., 0., 0.),
                    wall_vec: coordinate_to_vec3(vec![
                        (-1, 4),
                        (0, 4),
                        (1, 4),
                        (1, 3),
                        (1, 2),
                        (2, 1),
                        (3, 0),
                        (3, -1),
                        (2, -2),
                        (1, -2),
                        (0, -2),
                        (-1, -2),
                        (-1, -1),
                        (-2, -1),
                        (-3, 0),
                        (-3, 1),
                        (-2, 2),
                        (-2, 3),
                    ]),
                    crab_pot_vec: coordinate_to_vec3(vec![(0, 2), (-1, 0), (1, -1)]),
                    stone_vec: coordinate_to_vec3(vec![(1, 0), (2, 0), (2, -1)]),
                }),
                GameStage::Gaming(Episode {
                    player_position: Vec3::new(0., 0., 0.),
                    wall_vec: coordinate_to_vec3(vec![
                        (-2, -1),
                        (-2, 0),
                        (-2, 1),
                        (-2, 2),
                        (-1, -1),
                        (-1, 2),
                        (0, -1),
                        (0, 2),
                        (0, 3),
                        (0, 4),
                        (1, -1),
                        (1, 4),
                        (1, 5),
                        (2, -1),
                        (2, 5),
                        (3, -1),
                        (3, 5),
                        (4, -1),
                        (4, 1),
                        (4, 3),
                        (4, 4),
                        (4, 5),
                        (5, -1),
                        (5, 3),
                        (6, -1),
                        (6, 0),
                        (6, 1),
                        (6, 2),
                        (6, 3),
                    ]),
                    crab_pot_vec: coordinate_to_vec3(vec![(1, 0), (1, 1), (3, 1), (3, 3), (4, 2)]),
                    stone_vec: coordinate_to_vec3(vec![(2, 0), (2, 2), (3, 1), (4, 0), (4, 2)]),
                }),
                GameStage::End,
            ],
            current_episode: 0,
            legal_episode: 1,
        }
    }
}

pub enum GameStage {
    Pre(Episode),
    Gaming(Episode),
    End,
}

pub struct Episode {
    pub player_position: Vec3,
    pub wall_vec: Vec<Vec3>,
    pub crab_pot_vec: Vec<Vec3>,
    pub stone_vec: Vec<Vec3>,
}

pub fn draw(stage: &GameStage, mut commands: Commands, image: Res<ImageResource>) {
    match stage {
        GameStage::Pre(episode) => {
            commands
                .spawn(SpriteBundle {
                    texture: image.pre.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(1.)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0., 200., 1.),
                        scale: Vec3::new(255., 144., 1.),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(ShowText);

            draw_component(&mut commands, &episode.wall_vec, Wall, image.wall.clone());
            draw_component(
                &mut commands,
                &episode.crab_pot_vec,
                CrabPot,
                image.crab_pot.clone(),
            );
            draw_component(
                &mut commands,
                &episode.stone_vec,
                Stone,
                image.stone.clone(),
            );
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(1.)),
                        ..Default::default()
                    },
                    texture: image.player.clone(),
                    transform: Transform {
                        translation: episode.player_position,
                        scale: Vec3::new(GENERAL_SIZE, GENERAL_SIZE, 1.),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(Player);
        }
        GameStage::Gaming(episode) => {
            draw_component(&mut commands, &episode.wall_vec, Wall, image.wall.clone());
            draw_component(
                &mut commands,
                &episode.crab_pot_vec,
                CrabPot,
                image.crab_pot.clone(),
            );
            draw_component(
                &mut commands,
                &episode.stone_vec,
                Stone,
                image.stone.clone(),
            );
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(1.)),
                        ..Default::default()
                    },
                    texture: image.player.clone(),
                    transform: Transform {
                        translation: episode.player_position,
                        scale: Vec3::new(GENERAL_SIZE, GENERAL_SIZE, 1.),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(Player);
        }
        GameStage::End => {
            commands.spawn(SpriteBundle {
                texture: image.end.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(1.)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    scale: Vec3::new(232., 50., 1.),
                    ..Default::default()
                },
                ..default()
            });
        }
    }
}

fn draw_component(
    commands: &mut Commands,
    vec3: &Vec<Vec3>,
    bundle: impl Bundle + Clone,
    image: Handle<Image>,
) {
    for vec in vec3 {
        commands
            .spawn(SpriteBundle {
                texture: image.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(1.)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: *vec,
                    scale: Vec3::new(GENERAL_SIZE, GENERAL_SIZE, 1.),
                    ..Default::default()
                },
                ..default()
            })
            .insert(bundle.clone());
    }
}

/// 讲坐标转换映射到bevy画布
fn coordinate_to_vec3(coordinate: Vec<(i32, i32)>) -> Vec<Vec3> {
    coordinate
        .into_iter()
        .map(|(x, y)| (x as f32, y as f32))
        .map(|(x, y)| Vec3::new(x * GENERAL_SIZE, y * GENERAL_SIZE, 1.))
        .collect()
}
