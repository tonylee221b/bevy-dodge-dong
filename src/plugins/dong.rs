use crate::game::fall::{FallAffected, Velocity};
use crate::game::timer;
use bevy::prelude::*;
use rand::Rng;

use super::collider::{Collider, CollisionBehaviour, CollisionLayer, layers};

pub struct DongPlugin;

impl Plugin for DongPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_dong, despawn_on_lifetime_end));
    }
}

const DONG_COLOR: Color = Color::srgb(0.36, 0.25, 0.2);
const DONG_SIZE: Vec2 = Vec2::new(50.0, 50.0);

#[derive(Component)]
struct Dong;

#[derive(Component)]
struct Lifetime {
    timer: Timer,
}

fn spawn_dong(
    time: Res<Time>,
    mut spawn_timer: ResMut<timer::SpawnTimer>,
    mut commands: Commands,
    window: Single<&Window>,
) {
    if spawn_timer.timer.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();

        let window_right = window.width() / 2.0;
        let window_left = -window_right;
        let window_top = window.height() / 2.0;

        let random_x: f32 = rng.random_range(window_left..window_right);
        let random_fall_speed: f32 = rng.random_range(700.0..1700.0);

        commands.spawn((
            Sprite::from_color(DONG_COLOR, Vec2::ONE),
            Transform {
                translation: Vec3::new(random_x, window_top + DONG_SIZE.y / 2.0, 0.0),
                scale: DONG_SIZE.extend(1.0),
                ..default()
            },
            Dong,
            Collider {
                size: Vec2 {
                    x: DONG_SIZE.x,
                    y: DONG_SIZE.y,
                },
            },
            CollisionBehaviour {
                entity_name: String::from("Dong"),
            },
            CollisionLayer {
                layer: layers::DONG,
                mask: layers::PLAYER,
            },
            FallAffected::set_fall_force(random_fall_speed),
            Velocity::new(0.0, -1.0),
            Lifetime {
                timer: Timer::from_seconds(3.5, TimerMode::Once),
            },
        ));
    }
}

fn despawn_on_lifetime_end(
    time: Res<Time>,
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Lifetime), With<Dong>>,
) {
    for (entity, mut lifetime) in entities.iter_mut() {
        if lifetime.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
