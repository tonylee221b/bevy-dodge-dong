use crate::prelude::*;

#[derive(Event)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}
