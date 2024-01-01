use bevy::prelude::*;

pub const ENEMY_RESPAWN_TIMER: f32 = 1.0;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_RESPAWN_TIMER, TimerMode::Repeating)
        }
    }
}