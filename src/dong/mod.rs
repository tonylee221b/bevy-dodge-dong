pub mod components;
pub mod systems;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (systems::spawn_dong, systems::despawn_on_lifetime_end),
    );
}
