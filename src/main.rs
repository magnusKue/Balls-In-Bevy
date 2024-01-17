use bevy::prelude::*;

mod events;
mod systems;
mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use crate::systems::*;

fn main() {
    App::new()
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins((
            GamePlugin,
            MainMenuPlugin
        ))
        // Systems
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (
            exit_game,
            handle_game_over.run_if(in_state(AppState::Game)),
            transition_to_main_menu_state,
            transition_to_game_state
        ))
        .run()
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}