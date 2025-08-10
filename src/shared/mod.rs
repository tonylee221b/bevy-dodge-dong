pub mod timer;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, timer::systems::setup_spawn_timer);
}
