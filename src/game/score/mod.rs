use bevy::prelude::*;

use systems::*;
use resources::*;

pub mod resources;
pub mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(Update, (
                update_score,
                update_highscores,
                handle_highscore_updates
            ));
    }
}