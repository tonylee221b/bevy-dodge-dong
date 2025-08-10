pub mod dong;
pub mod physics;
pub mod player;
pub mod prelude;
pub mod shared;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::plugin,
            dong::plugin,
            physics::plugin,
            shared::plugin,
        ));
    }
}
