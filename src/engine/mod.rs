pub mod physics;
pub mod timer;

use crate::prelude::*;

use physics::{
    collisions::{self, components::CollisionIgnoreList},
    fallings,
};

pub fn plugin(app: &mut App) {
    app.init_resource::<CollisionIgnoreList>()
        .add_systems(Startup, timer::systems::setup_spawn_timer)
        .add_event::<collisions::events::CollisionEvent>()
        .add_systems(
            Update,
            (
                collisions::systems::collision_detection_event,
                fallings::systems::apply_fall_system,
                fallings::systems::apply_velocity_system,
            ),
        );
}
