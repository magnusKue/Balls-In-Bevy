use bevy::{prelude::*, window::PrimaryWindow};

use super::components::*;
use crate::events::GameOver;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::enemy::components::*;
use crate::game::star::{components::*, systems::*};
use crate::game::score::resources::Score;
use crate::game::player::*;

pub fn spawn_player(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player { },
        )
    );
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;

        let min_x = 0.0 + half_player_size;
        let min_y = 0.0 + half_player_size;

        let max_x = window.width() - half_player_size;
        let max_y = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x > max_x {
            translation.x = max_x;
        }
        else if translation.x < min_x {
            translation.x = min_x;
        }

        if translation.y > max_y {
            translation.y = max_y;
        }
        else if translation.y < min_y {
            translation.y = min_y;
        }

        player_transform.translation = translation;

    }
}

pub fn enemy_hit_player(
    mut event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(&mut Enemy, &Transform)>,
    score: Res<Score>
) {
    if let Ok((_, player_transform)) = player_query.get_single_mut() {
        let mut collision: bool = false;

        for (_, enemy_transform) in enemy_query.iter_mut() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE * 0.5;
            let enemy_radius = ENEMY_SIZE * 0.5;


            if distance < player_radius + enemy_radius {
                collision = true;
                break;
            }
        }
        if collision {
            event_writer.send(GameOver {score: score.value});
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter_mut() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            let player_radius = PLAYER_SIZE * 0.5;
            let star_radius = STAR_SIZE * 0.5;


            if distance < player_radius + star_radius {
                commands.entity(star_entity).despawn();
                score.value += 1; 
                commands.spawn(
                    AudioBundle {
                        source: asset_server.load("audio/jingles_NES09.ogg"),
                        settings: PlaybackSettings::DESPAWN,
                        ..default()
                    }
                );

            }
        };
    }
}  