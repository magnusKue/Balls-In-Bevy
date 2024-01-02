use bevy::prelude::*;

pub const STAR_RESPAWN_TIMER: f32 = 3.0;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_RESPAWN_TIMER, TimerMode::Repeating)
        }
    }
}