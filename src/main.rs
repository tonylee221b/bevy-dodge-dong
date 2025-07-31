use bevy::prelude::*;

pub mod game;
pub mod plugins;

fn main() {
    App::new()
        .add_plugins((plugins::custom_window::CustomWindowPlugin, game::GamePlugin))
        .run();
}
