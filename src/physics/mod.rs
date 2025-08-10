pub mod collisions;
pub mod fallings;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_event::<collisions::events::CollisionEvent>()
        .add_systems(
            Update,
            (
                collisions::systems::collision_response_event,
                collisions::systems::collision_detection_event,
                fallings::systems::apply_fall_system,
                fallings::systems::apply_velocity_system,
            ),
        );
}
