use crate::{prelude::*, shared::timer::components::SpawnTimer};

pub fn setup_spawn_timer(mut commands: Commands) {
    commands.insert_resource(SpawnTimer {
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
    });

    println!("SpawnTimer set");
}
