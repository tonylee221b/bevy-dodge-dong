use bevy::prelude::*;

pub struct CollisionPlugin;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct CollisionBehaviour {
    pub destroy_on_collision: bool,
    pub damage: i32,
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

pub fn collision_detection_system(
    mut collision_events: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Transform, &Collider)>,
) {
    let mut combinations = query.iter_combinations();
    while let Some(
        [
            (entity_a, transform_a, collider_a),
            (entity_b, transform_b, collider_b),
        ],
    ) = combinations.fetch_next()
    {
        if check_collision(transform_a, collider_a, transform_b, collider_b) {
            collision_events.write(CollisionEvent { entity_a, entity_b });
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
