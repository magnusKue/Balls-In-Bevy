use bevy::{
    prelude::*, 
    window::PrimaryWindow, 
    app::AppExit
};

use colored::*;

use crate::AppState;
use crate::game::SimulationState;
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

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>
) {
    if keyboard_input.pressed(KeyCode::G) {
        if *app_state != AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game)));
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Entered Appstate::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>
) {
    if keyboard_input.pressed(KeyCode::M) {
        if *app_state != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            println!("Entered Appstate::MainMenu");
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<GameOver>,
    score: Res<Score>,
    mut next_state: ResMut<NextState<AppState>>
){
    for _ in event_reader.read() {
        println!("{} {}","Game Over! Final score:".red() ,score.value.to_string().green().bold());
        let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
        
        commands.spawn(
            AudioBundle {
                source: sound_effect,
                settings: PlaybackSettings::DESPAWN,
                ..default()
            }
        );

        //commands.insert_resource(NextState(Some(AppState::GameOver)));
        next_state.set(AppState::MainMenu); // this is the prefered way but does the same thing
        println!("Entered AppState::MainMenu");
    }
}
