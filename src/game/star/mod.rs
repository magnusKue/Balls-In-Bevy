use bevy::prelude::*;

use systems::*;
use resources::*;

use crate::game::SimulationState;
use crate::AppState;

pub mod components;
pub mod resources;
pub mod systems;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(OnExit(AppState::Game), despawn_stars)
            .add_systems(Update, (
                tick_star_spawn_timer
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
                spawn_stars_over_time
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
        ));
    }
}