use bevy::prelude::*;

use systems::*;
use resources::*;

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
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, (
                enemy_movement,
                confine_enemy_position,
                update_enemy_direction,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time
        ));
    }
}