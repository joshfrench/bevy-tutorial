use bevy::prelude::*;

pub const ENEMY_SPAWN_TIMER: f32 = 5.;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(ENEMY_SPAWN_TIMER, TimerMode::Repeating))
    }
}
