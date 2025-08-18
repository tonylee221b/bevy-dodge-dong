use crate::prelude::*;

use super::components::{Collider, CollisionIgnoreList, CollisionLayer};
use super::events::CollisionEvent;

pub fn collision_detection_event(
    mut collision_events: EventWriter<CollisionEvent>,
    mut ignore_list: ResMut<CollisionIgnoreList>,
    query: Query<(Entity, &Transform, &Collider, &CollisionLayer)>,
) {
    let mut combinations = query.iter_combinations();

    while let Some(
        [
            (entity_a, transform_a, collider_a, layer_a),
            (entity_b, transform_b, collider_b, layer_b),
        ],
    ) = combinations.fetch_next()
    {
        if ignore_list.ignored_pairs.contains(&(entity_a, entity_b)) {
            continue;
        }

        let a_can_collide_with_b = (layer_a.mask & layer_b.layer) != 0;
        let b_can_collide_with_a = (layer_b.mask & layer_a.layer) != 0;

        if !a_can_collide_with_b && !b_can_collide_with_a {
            continue;
        }

        if check_collision(transform_a, collider_a, transform_b, collider_b) {
            collision_events.write(CollisionEvent { entity_a, entity_b });

            ignore_list.ignored_pairs.insert((entity_a, entity_b));
        }
    }
}

fn check_collision(
    transform_a: &Transform,
    collider_a: &Collider,
    transform_b: &Transform,
    collider_b: &Collider,
) -> bool {
    let pos_a = transform_a.translation.truncate();
    let pos_b = transform_b.translation.truncate();

    let half_size_a = collider_a.size / 2.0;
    let half_size_b = collider_b.size / 2.0;

    let x_overlap = (pos_a.x - half_size_a.x) < (pos_b.x + half_size_b.x)
        && (pos_a.x + half_size_a.x) > (pos_b.x - half_size_b.x);
    let y_overlap = (pos_a.y - half_size_a.y) < (pos_b.y + half_size_b.y)
        && (pos_a.y + half_size_a.y) > (pos_b.y - half_size_b.y);

    x_overlap && y_overlap
}
