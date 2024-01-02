use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

// use crate::enemy::components::*;
use super::ENEMY_SIZE;
use super::NUMBER_OF_ENEMIES;
use super::ENEMY_SPEED;
use super::components::*;
use super::resources::*;

use crate::game::player::components::Player;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let rand_x = random::<f32>() * (window.width()  - ENEMY_SIZE) + (0.5 * ENEMY_SIZE);
        let rand_y = random::<f32>() * (window.height() - ENEMY_SIZE) + (0.5 * ENEMY_SIZE);
        
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize_or_zero(),
                speed: ENEMY_SPEED
            },
        ));
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * enemy.speed * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let mut direction_changed: bool = false;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        if transform.translation.x <= (ENEMY_SIZE / 2.0) || transform.translation.x >= window.width() - (ENEMY_SIZE / 2.0){
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if transform.translation.y <= (ENEMY_SIZE / 2.0) || transform.translation.y >= window.height() - (ENEMY_SIZE / 2.0){
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }
    }

    if direction_changed {
        let sound_effect = asset_server.load(match random::<bool>() {
            true => "audio/impactSoft_heavy_001.ogg",
            false => "audio/impactSoft_heavy_003.ogg"
        });
        
        commands.spawn(AudioBundle {
            source: sound_effect,
            settings: PlaybackSettings::DESPAWN,
            ..default()
        });
    }
}

pub fn confine_enemy_position(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    for mut enemy_transform in enemy_query.iter_mut(){
        let window = window_query.get_single().unwrap();

        let half_enemy_size = ENEMY_SIZE / 2.0;

        let min_x = 0.0 + half_enemy_size;
        let min_y = 0.0 + half_enemy_size;

        let max_x = window.width() - half_enemy_size;
        let max_y = window.height() - half_enemy_size;

        let mut translation = enemy_transform.translation;

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

        enemy_transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>
) {
    if player_query.get_single().is_ok() {
        enemy_spawn_timer.timer.tick(time.delta());
    }
}

pub fn spawn_enemies_over_time(
    spawn_timer: Res<EnemySpawnTimer>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    if spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap(); 
        for _ in 0..NUMBER_OF_ENEMIES {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    transform: Transform::from_xyz(
                        rand::random::<f32>() * (window.width()  - ENEMY_SIZE) + (0.5 * ENEMY_SIZE), 
                        rand::random::<f32>() * (window.width()  - ENEMY_SIZE) + (0.5 * ENEMY_SIZE), 
                        0.0
                    ),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(
                        rand::random::<f32>(), 
                        rand::random::<f32>()
                    ).normalize_or_zero(),
                    speed: ENEMY_SPEED
                }
            ));
        }
    }
}