/*
# TODO
 */

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::time::FixedTimestep;

use component::*;

mod component;

const GENERAL_SIZE: f32 = 40.;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::hex("93a7ae").unwrap()),
        },
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            ..Default::default()
        }))
        .add_startup_system(setup)
        .insert_resource(Follow::init_follow())
        .add_startup_system(init_image_resource)
        .add_system(start_game)
        .add_system(keyboard)
        .add_system(flush)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(check_pass),
        )
        .run();
}

fn start_game(
    mut commands: Commands,
    mut follow: ResMut<Follow>,
    image: Res<ImageResource>,
    query_player: Query<Entity, With<Player>>,
    query_wall: Query<Entity, With<Wall>>,
    query_crab_pot: Query<Entity, With<CrabPot>>,
    query_stone: Query<Entity, With<Stone>>,
    text_stone: Query<Entity, With<ShowText>>,
) {
    // 绘制过不再绘制
    if follow.current_episode < follow.legal_episode {
        for entity in query_player.iter() {
            commands.entity(entity).despawn();
        }
        for entity in query_wall.iter() {
            commands.entity(entity).despawn();
        }
        for entity in query_crab_pot.iter() {
            commands.entity(entity).despawn();
        }
        for entity in query_stone.iter() {
            commands.entity(entity).despawn();
        }
        for entity in text_stone.iter() {
            commands.entity(entity).despawn();
        }

        draw(&follow.vec[follow.legal_episode - 1], commands, image);
        follow.current_episode += 1;
    }
}

fn flush(keyboard_input: Res<Input<KeyCode>>, mut follow: ResMut<Follow>) {
    if keyboard_input.just_pressed(KeyCode::F5) && follow.current_episode > 0 {
        follow.current_episode -= 1;
    }
}

/// 检查是否所有蟹笼被推到了石头上则进行下一关
fn check_pass(
    stones: Query<&Transform, (With<Stone>, Without<CrabPot>)>,
    crab_pots: Query<&Transform, (With<CrabPot>, Without<Wall>)>,
    mut follow: ResMut<Follow>,
) {
    // 如果是最后一关则不进行计算
    if let GameStage::End = follow.vec[follow.current_episode - 1] {
        return;
    }

    let mut crab_count = 0;
    let mut break_count = 0;

    crab_pots.iter().for_each(|pot| {
        let crab_pot_position = pot.translation.xy();
        crab_count += 1;
        stones.iter().for_each(|stone| {
            if crab_pot_position == stone.translation.xy() {
                break_count += 1;
            }
        })
    });

    if crab_count == break_count {
        follow.legal_episode += 1;
    }
}

fn keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut positions: Query<&mut Transform, With<Player>>,
    walls: Query<&Transform, (With<Wall>, Without<Player>, Without<CrabPot>)>,
    mut crab_pots: Query<&mut Transform, (With<CrabPot>, Without<Player>, Without<Wall>)>,
) {
    if let Some(mut transform) = positions.iter_mut().next() {
        let input = if keyboard_input.just_pressed(KeyCode::Left)
            || keyboard_input.just_pressed(KeyCode::A)
        {
            Some(KeyCode::Left)
        } else if keyboard_input.just_pressed(KeyCode::Right)
            || keyboard_input.just_pressed(KeyCode::D)
        {
            Some(KeyCode::Right)
        } else if keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::W)
        {
            Some(KeyCode::Up)
        } else if keyboard_input.just_pressed(KeyCode::Down)
            || keyboard_input.just_pressed(KeyCode::S)
        {
            Some(KeyCode::Down)
        } else {
            None
        };

        if let Some(input) = input {
            let (next_position, next_next_position) =
                get_next_position(input, &transform.translation.xy());

            // 有墙
            if walls
                .iter()
                .any(|temp| temp.translation.xy() == next_position)
            {
                return;
            }

            let next_next_wall = walls
                .iter()
                .find(|temp| temp.translation.xy() == next_next_position);
            let next_next_crab_pot = crab_pots
                .iter()
                .any(|temp| temp.translation.xy() == next_next_position);

            let next_crab_pot = crab_pots
                .iter_mut()
                .find(|temp| temp.translation.xy() == next_position);

            // 移动
            if next_crab_pot.is_none() {
                transform.translation.x = next_position.x;
                transform.translation.y = next_position.y;
            }
            // 推动箱子 只有在下一个格子为箱子，并且下两个格子不是箱子和墙壁
            else if next_crab_pot.is_some() && next_next_wall.is_none() && !next_next_crab_pot {
                transform.translation.x = next_position.x;
                transform.translation.y = next_position.y;

                let mut next_crab_pot_position = next_crab_pot.unwrap();
                next_crab_pot_position.translation.x = next_next_position.x;
                next_crab_pot_position.translation.y = next_next_position.y;
            }
        }
    }
}

/// 返回下两个格子的坐标
fn get_next_position(key: KeyCode, now_position: &Vec2) -> (Vec2, Vec2) {
    match key {
        KeyCode::Left => (
            Vec2::new(now_position.x - GENERAL_SIZE, now_position.y),
            Vec2::new(now_position.x - GENERAL_SIZE - GENERAL_SIZE, now_position.y),
        ),
        KeyCode::Up => (
            Vec2::new(now_position.x, now_position.y + GENERAL_SIZE),
            Vec2::new(now_position.x, now_position.y + GENERAL_SIZE + GENERAL_SIZE),
        ),
        KeyCode::Right => (
            Vec2::new(now_position.x + GENERAL_SIZE, now_position.y),
            Vec2::new(now_position.x + GENERAL_SIZE + GENERAL_SIZE, now_position.y),
        ),
        KeyCode::Down => (
            Vec2::new(now_position.x, now_position.y - GENERAL_SIZE),
            Vec2::new(now_position.x, now_position.y - GENERAL_SIZE - GENERAL_SIZE),
        ),
        _ => None.unwrap(),
    }
}
