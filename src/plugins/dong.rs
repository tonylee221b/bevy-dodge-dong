use crate::game::fall::{FallAffected, Velocity};
use bevy::prelude::*;
use rand::Rng;

use super::collider::Collider;

pub struct DongPlugin;

impl Plugin for DongPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dong);
    }
}

const DONG_COLOR: Color = Color::srgb(0.36, 0.25, 0.2);
const DONG_SIZE: Vec2 = Vec2::new(50.0, 50.0);

#[derive(Component)]
struct Dong;

fn spawn_dong(mut commands: Commands) {
    let mut rng = rand::rng();
    let random_x: f32 = rng.random_range(-200.0..=200.0);

    commands.spawn((
        Sprite::from_color(DONG_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(random_x, 300.0, 0.0),
            scale: DONG_SIZE.extend(1.0),
            ..default()
        },
        Dong,
        Collider,
        FallAffected::set_fall_force(-300.0),
        Velocity::new(0.0, -1.0),
    ));
}
