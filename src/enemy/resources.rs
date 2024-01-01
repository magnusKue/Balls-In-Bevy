use bevy::prelude::*;

use super::ENEMY_RESPAWN_TIMER;

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