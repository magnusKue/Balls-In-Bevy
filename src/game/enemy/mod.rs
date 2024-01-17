use bevy::prelude::*;

use systems::*;
use resources::*;

use crate::AppState;

use super::SimulationState;

pub mod components;
pub mod resources;
pub mod systems;

pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 450.0;
pub const NUMBER_OF_ENEMIES: usize = 1;
pub const ENEMY_RESPAWN_TIMER: f32 = 1.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(OnExit(AppState::Game), despawn_enemies)
            .add_systems(Update, (
                enemy_movement.before(confine_enemy_position)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
                confine_enemy_position,
                update_enemy_direction
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
                tick_enemy_spawn_timer
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
                spawn_enemies_over_time
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
        ));
    }
}