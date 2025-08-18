use crate::prelude::*;

use super::components::{FallAffected, FallVelocity};

pub fn apply_fall_system(mut query: Query<(&FallAffected, &mut FallVelocity)>) {
    for (fall_affected, mut velocity) in query.iter_mut() {
        velocity.y = fall_affected.fall_force;
    }
}

pub fn apply_velocity_system(time: Res<Time>, mut query: Query<(&mut Transform, &FallVelocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}
