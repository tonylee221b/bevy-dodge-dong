use bevy::prelude::*;

mod game;
mod plugins;

use game::fall::FallPlugin;
use plugins::{dong::DongPlugin, player::PlayerPlugin};

const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FallPlugin))
        .add_plugins((PlayerPlugin, DongPlugin))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(BG_COLOR))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
