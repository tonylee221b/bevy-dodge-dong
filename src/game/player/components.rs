use crate::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerHealth {
    pub cnt: u8,
}
