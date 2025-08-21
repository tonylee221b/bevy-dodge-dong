use crate::{
    engine::physics::collisions::{components::*, events::CollisionEvent},
    game::global::components::{GameEntity, GameState},
    prelude::*,
};

use super::components::{Player, PlayerHealth, PlayerPlugin};

const PLAYER_SIZE: Vec2 = Vec2::new(40.0, 60.0);
const PLAYER_SPEED: f32 = 750.0;
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (spawn_player, setup_player_health),
        )
        .add_systems(OnExit(GameState::InGame), cleanup_player_health)
        .add_systems(
            Update,
            (move_player, take_damage, check_game_over).run_if(in_state(GameState::InGame)),
        );
    }
}

impl Default for PlayerHealth {
    fn default() -> Self {
        Self { cnt: 3 }
    }
}

fn spawn_player(mut commands: Commands, window: Single<&Window>) {
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
        CollisionLayer {
            layer: layers::PLAYER,
            mask: layers::DONG,
        },
        GameEntity,
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

fn take_damage(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_health: Option<ResMut<PlayerHealth>>,
) {
    for _ in collision_events.read() {
        if let Some(ref mut player_health) = player_health {
            player_health.cnt = player_health.cnt.saturating_sub(1);
        }
    }
}

fn check_game_over(
    player_health: Option<Res<PlayerHealth>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(health) = player_health {
        if health.cnt == 0 {
            next_state.set(GameState::GameOver);
        }
    }
}

fn setup_player_health(mut commands: Commands) {
    commands.init_resource::<PlayerHealth>();
}

fn cleanup_player_health(mut commands: Commands) {
    commands.remove_resource::<PlayerHealth>();
}
