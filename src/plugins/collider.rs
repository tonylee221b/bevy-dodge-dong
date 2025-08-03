use bevy::prelude::*;

pub struct CollisionPlugin;

#[derive(Component, Default)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;
