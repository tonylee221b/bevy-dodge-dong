use crate::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct CollisionBehaviour {
    pub entity_name: String,
}

#[derive(Component)]
pub struct CollisionLayer {
    pub layer: u8, // 지금 속한 레이어
    pub mask: u8,  // 어떤 레이어와 충돌?
}

pub mod layers {
    pub const PLAYER: u8 = 1 << 0; // 0001
    pub const DONG: u8 = 1 << 1; // 0010
}
