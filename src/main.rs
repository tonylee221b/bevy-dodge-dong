use bevy::prelude::*;

mod game;
mod plugins;

use game::fall::FallPlugin;
use game::timer;
use plugins::{collider::CollisionPlugin, dong::DongPlugin, player::PlayerPlugin};

const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_systems(Startup, (setup, timer::setup_spawn_timer).chain())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::Fifo,
                    ..default()
                }),
                ..default()
            }),
            FallPlugin,
        ))
        .add_plugins((PlayerPlugin, DongPlugin, CollisionPlugin))
        .insert_resource(ClearColor(BG_COLOR))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
