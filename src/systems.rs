use bevy::{
    prelude::*, 
    window::PrimaryWindow, 
    app::AppExit
};
use colored::*;

use crate::game::enemy::components::*;
use crate::game::player::components::*;
use crate::game::score::resources::Score;
use crate::events::GameOver;

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
