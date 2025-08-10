use bevy::prelude::*;
use bevy_dodge_dong::GamePlugin;

const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }),))
        .add_plugins(GamePlugin)
        .insert_resource(ClearColor(BG_COLOR))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
