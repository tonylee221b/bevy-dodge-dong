use bevy::prelude::*;
use bevy_dodge_dong::GamePlugin;
use bevy_egui::EguiPlugin;

const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }),))
        .add_plugins(EguiPlugin::default())
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
