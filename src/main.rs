use bevy::prelude::*;

mod plugins;

use plugins::player::PlayerPlugin;

const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .insert_resource(ClearColor(BG_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
