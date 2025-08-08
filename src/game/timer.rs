use bevy::prelude::*;

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

pub fn setup_spawn_timer(mut commands: Commands) {
    commands.insert_resource(SpawnTimer {
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
    });

    println!("SpawnTimer set");
}
