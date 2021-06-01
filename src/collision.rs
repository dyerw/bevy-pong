use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::components::*;
use crate::events::*;

pub struct PongCollisionPlugin;

impl Plugin for PongCollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(collision_detection_system.system());
    }
}

fn collision_detection_system(
    mut collision_writer: EventWriter<CollisionEvent>,
    q: Query<(Entity, &Transform, &Collider)>,
) {
    let mut sent_collisions: Vec<CollisionEvent> = vec![];
    for (entity, transform, collider) in q.iter() {
        let others = q.iter().filter(|(_e, t, _c)| *t != transform);
        for (other_entity, other_transform, other_collider) in others {
            let collision = collide(
                transform.translation,
                collider.0,
                other_transform.translation,
                other_collider.0,
            );
            if let Some(_collision) = collision {
                let collision_event = CollisionEvent(entity, other_entity);
                if !sent_collisions.contains(&collision_event) {
                    sent_collisions.push(collision_event);
                    collision_writer.send(collision_event);
                }
            }
        }
    }
}
