use bevy::prelude::*;

pub struct FallPlugin;

impl Plugin for FallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_fall_system, apply_velocity_system).chain());
    }
}

#[derive(Component, Debug)]
pub struct FallAffected {
    pub fall_force: f32,
}

impl Default for FallAffected {
    fn default() -> Self {
        Self { fall_force: -9.8 }
    }
}

impl FallAffected {
    pub fn set_fall_force(fall_force: f32) -> Self {
        Self {
            fall_force: -fall_force,
        }
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub fn apply_fall_system(mut query: Query<(&FallAffected, &mut Velocity)>) {
    for (gv_affected, mut velocity) in query.iter_mut() {
        velocity.y = gv_affected.fall_force;
    }
}

pub fn apply_velocity_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}
