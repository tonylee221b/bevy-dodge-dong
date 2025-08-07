use bevy::prelude::*;

use super::collider::Collider;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, move_player);
    }
}

const PLAYER_SIZE: Vec2 = Vec2::new(40.0, 60.0);
const PLAYER_SPEED: f32 = 1000.0;
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(PLAYER_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, -300.0, 0.0),
            scale: PLAYER_SIZE.extend(1.0),
            ..default()
        },
        Player,
        Collider,
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
