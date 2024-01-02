use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::game::star::{resources::*, components::*};
use crate::game::player::components::Player;

pub const STAR_SIZE: f32 = 30.0;
pub const NUMBER_OF_STARS: usize = 4;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let rand_x = random::<f32>() * (window.width()  - STAR_SIZE) + (0.5 * STAR_SIZE);
        let rand_y = random::<f32>() * (window.height() - STAR_SIZE) + (0.5 * STAR_SIZE);
        
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star { }
        ));
    }
}

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>
) {
    if player_query.get_single().is_ok() {
        star_spawn_timer.timer.tick(time.delta());
    }
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    star_spawn_timer: Res<StarSpawnTimer>
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        for _ in 0..NUMBER_OF_STARS {
            let rand_x = random::<f32>() * (window.width()  - STAR_SIZE) + (0.5 * STAR_SIZE);
            let rand_y = random::<f32>() * (window.height() - STAR_SIZE) + (0.5 * STAR_SIZE);
            
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star { }
            ));
        }
    }
}