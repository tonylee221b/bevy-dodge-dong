mod engine;
mod game;
mod prelude;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((engine::plugin, game::plugin));
    }
}
