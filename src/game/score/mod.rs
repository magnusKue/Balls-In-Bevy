use bevy::prelude::*;

use systems::*;
use resources::*;

use crate::AppState;

pub mod resources;
pub mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_systems(Update, (
                update_score.run_if(in_state(AppState::Game)),
                update_highscores.run_if(in_state(AppState::Game)),
                handle_highscore_updates
            ))
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}