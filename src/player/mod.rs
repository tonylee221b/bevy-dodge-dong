pub mod components;
pub mod systems;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, systems::spawn_player)
        .add_systems(Update, systems::move_player);
}
