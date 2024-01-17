use bevy::prelude::*;

use systems::*;

use crate::AppState;

use super::SimulationState;

pub mod components;
pub mod systems;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(OnExit(AppState::Game), despawn_player)
            .add_systems(Update, (
                player_movement.before(confine_player_movement)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
                confine_player_movement,
                player_hit_enemy.run_if(in_state(AppState::Game)),
                player_hit_star.run_if(in_state(AppState::Game))
        ));
    }
}