use crate::prelude::*;

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
pub struct FallVelocity {
    pub x: f32,
    pub y: f32,
}

impl Default for FallVelocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl FallVelocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
