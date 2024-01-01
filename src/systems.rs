use bevy::{
    prelude::*, 
    window::PrimaryWindow, 
    app::AppExit
};
use rand::random;
use colored::*;

use crate::components::*;
use crate::resources::*;
use crate::events::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 450.0;
pub const NUMBER_OF_ENEMIES: usize = 1;

pub const STAR_SIZE: f32 = 30.0;
pub const NUMBER_OF_STARS: usize = 4;

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

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

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
//? MOVEMENT SYSTEMS

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

//? INTERACTION SYSTEMS

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

pub fn update_score(
    score: Res<Score>
) {
    if score.is_changed() {
        println!("{} {}", "new Score:".blue() ,score.value.to_string().red());
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

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>
) {
    if player_query.get_single().is_ok() {
        enemy_spawn_timer.timer.tick(time.delta());
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
                        rand::random::<f32>() * (window.width()  - STAR_SIZE) + (0.5 * ENEMY_SIZE), 
                        rand::random::<f32>() * (window.width()  - STAR_SIZE) + (0.5 * ENEMY_SIZE), 
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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(&mut Enemy, &Transform)>,
    score: Res<Score>
){
    for _ in event_reader.read() {
        let window = window_query.get_single().unwrap();
        println!("{} {}","Game Over! Final score:".red() ,score.value.to_string().green().bold());
        let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
        
        commands.spawn(
            AudioBundle {
                source: sound_effect,
                settings: PlaybackSettings::DESPAWN,
                ..default()
            }
        );
        
        commands.spawn(
            SpriteBundle {
                texture: asset_server.load("sprites/game_over.png"),
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                ..default()
            }
        );
        
        if let Ok((player_entity, _)) = player_query.get_single_mut() {
            commands.entity(player_entity).despawn();
        }

        for (mut enemy, _) in enemy_query.iter_mut() {
            enemy.speed = 0.0;
        }
    }
}

pub fn update_highscores(
    mut game_over_event: EventReader<GameOver>,
    mut highscores: ResMut<HighScores>,
    score: Res<Score>
) {
    for _ in game_over_event.read() {
        highscores.scores.push(("Player1".to_string(), score.value))
    }
}

pub fn handle_highscore_updates(
    highscores: Res<HighScores>
) {
    if highscores.is_changed() {
        println!("{} {:?}", "High Scores:".red(), highscores.scores);
    }
}