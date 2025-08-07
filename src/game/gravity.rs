use bevy::prelude::*;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_gravity_system, apply_velocity_system).chain(),
        );
    }
}

#[derive(Component, Debug)]
pub struct GravityAffected {
    pub gravity: f32,
}

impl Default for GravityAffected {
    fn default() -> Self {
        Self { gravity: -9.8 }
    }
}

impl GravityAffected {
    pub fn set_gravity(gravity: f32) -> Self {
        Self { gravity }
    }
}

#[derive(Component, Debug)]
pub struct GravityVelocity {
    pub x: f32,
    pub y: f32,
}

impl Default for GravityVelocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl GravityVelocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub fn apply_gravity_system(
    time: Res<Time>,
    mut query: Query<(&GravityAffected, &mut GravityVelocity)>,
) {
    for (gv_affected, mut velocity) in query.iter_mut() {
        velocity.y += gv_affected.gravity * time.delta_secs();
        println!("velocity: {:#?}", velocity.y);
    }
}

pub fn apply_velocity_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &GravityVelocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}
