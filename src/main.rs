use bevy::prelude::*;


mod enemy;
mod player;
mod score;
mod star;

mod events;
mod systems;

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use star::StarPlugin;
use score::ScorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugins((
            StarPlugin,
            PlayerPlugin,
            EnemyPlugin,
            ScorePlugin
        ))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (
            exit_game,
            handle_game_over,
        ))
        .run()
}
