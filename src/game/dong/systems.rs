use crate::{
    engine::{
        physics::{
            collisions::{components::*, events::CollisionEvent},
            fallings::components::{FallAffected, FallVelocity},
        },
        timer::components::*,
    },
    game::{
        global::components::{GameEntity, GameState, Score},
        player::components::Player,
    },
    prelude::*,
};
use rand::Rng;

use super::components::{Dong, DongPlugin};

const DONG_COLOR: Color = Color::srgb(0.36, 0.25, 0.2);
const DONG_SIZE: Vec2 = Vec2::new(50.0, 50.0);

impl Plugin for DongPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_dong, despawn_on_collision, dong_ground_check)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn spawn_dong(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
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
            CollisionLayer {
                layer: layers::DONG,
                mask: layers::PLAYER,
            },
            FallAffected::set_fall_force(random_fall_speed),
            FallVelocity::new(0.0, -1.0),
            GameEntity,
        ));
    }
}

fn dong_ground_check(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Collider), With<Dong>>,
    window: Single<&Window>,
    mut score: Option<ResMut<Score>>,
) {
    let window_bottom = -window.height() / 2.0;

    for (entity, transform, collider) in query.iter() {
        let bottom_line = window_bottom - (collider.size.y / 2.0);
        if transform.translation.y <= bottom_line {
            commands.entity(entity).despawn();

            if let Some(ref mut score) = score {
                score.value += 1;
            }
        }
    }
}

fn despawn_on_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    dong_query: Query<Entity, With<Dong>>,
    player_query: Query<Entity, With<Player>>,
) {
    for collision_event in collision_events.read() {
        let entity_a = collision_event.entity_a;
        let entity_b = collision_event.entity_b;

        if dong_query.contains(entity_a) && player_query.contains(entity_b) {
            commands.entity(entity_a).despawn();
        }

        if dong_query.contains(entity_b) && player_query.contains(entity_a) {
            commands.entity(entity_b).despawn();
        }
    }
}
