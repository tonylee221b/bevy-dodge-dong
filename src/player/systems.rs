use crate::{physics::collisions::components::*, prelude::*};

use super::components::Player;

const PLAYER_SIZE: Vec2 = Vec2::new(40.0, 60.0);
const PLAYER_SPEED: f32 = 750.0;
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

pub fn spawn_player(mut commands: Commands, window: Single<&Window>) {
    let ground_pos_y = (-window.height() / 2.0) + (PLAYER_SIZE.y / 2.0) + 10.0;

    commands.spawn((
        Sprite::from_color(PLAYER_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, ground_pos_y, 0.0),
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

pub fn move_player(
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
