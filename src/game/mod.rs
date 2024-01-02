use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use star::StarPlugin;
use score::ScorePlugin;

use crate::events::GameOver;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
        .add_plugins((
            EnemyPlugin,
            PlayerPlugin,
            StarPlugin,
            ScorePlugin
        ));
    }
}