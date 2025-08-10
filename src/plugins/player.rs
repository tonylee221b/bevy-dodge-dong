use bevy::prelude::*;

use super::collider::{Collider, CollisionBehaviour, CollisionLayer, layers};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, move_player);
    }
}

const PLAYER_SIZE: Vec2 = Vec2::new(40.0, 60.0);
const PLAYER_SPEED: f32 = 750.0;
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn spawn_player(mut commands: Commands, window: Single<&Window>) {
    let ground_pos = (-window.height() / 2.0) + (PLAYER_SIZE.y / 2.0) + 10.0;

    commands.spawn((
        Sprite::from_color(PLAYER_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, ground_pos, 0.0),
            scale: PLAYER_SIZE.extend(1.0),
            ..default()
        },
        Player,
        Collider {
            size: Vec2 {
                x: PLAYER_SIZE.x,
                y: PLAYER_SIZE.y,
            },
        },
        CollisionBehaviour {
            entity_name: String::from("Player"),
        },
        CollisionLayer {
            layer: layers::PLAYER,
            mask: layers::DONG,
        },
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    player_transform.translation.x += direction.x * PLAYER_SPEED * time.delta_secs();
}
