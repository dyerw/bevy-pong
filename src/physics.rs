use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::events::*;

pub struct PongPhysicsPlugin;

impl Plugin for PongPhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(ball_bounce_system.system())
            .add_system(ball_wall_bounce_system.system())
            .add_system(velocity_system.system());
    }
}

fn calculate_hit_position(ball_y: f32, paddle_y: f32) -> f32 {
    let transformed_ball_y = ball_y + SCREEN_HEIGHT / 2.;
    let transformed_paddle_y = paddle_y + SCREEN_HEIGHT / 2.;
    return (transformed_ball_y - transformed_paddle_y) / (PADDLE_HEIGHT / 2.);
}

fn ball_bounce_system(
    mut collision_reader: EventReader<CollisionEvent>,
    mut ball_query: Query<(Entity, &mut Velocity, &Transform), With<Ball>>,
    mut left_paddle_query: Query<(Entity, &Transform), With<LeftPlayer>>,
    mut right_paddle_query: Query<(Entity, &Transform), With<RightPlayer>>,
) {
    let queries = (
        ball_query.single_mut(),
        left_paddle_query.single_mut(),
        right_paddle_query.single_mut(),
    );
    if let (
        Ok((ball_entity, mut velocity, ball_transform)),
        Ok((left_paddle_entity, left_paddle_transform)),
        Ok((right_paddle_entity, right_paddle_transform)),
    ) = queries
    {
        for event in collision_reader.iter() {
            if event.contains(ball_entity) {
                let max_vy: f32 = BALL_HIT_MAX_ANGLE.tan() * BALL_X_VELOCITY;

                if event.contains(left_paddle_entity) {
                    velocity.0.x = BALL_X_VELOCITY;
                    velocity.0.y = max_vy
                        * calculate_hit_position(
                            ball_transform.translation.y,
                            left_paddle_transform.translation.y,
                        );
                }
                if event.contains(right_paddle_entity) {
                    velocity.0.x = -1. * BALL_X_VELOCITY;
                    velocity.0.y = max_vy
                        * calculate_hit_position(
                            ball_transform.translation.y,
                            right_paddle_transform.translation.y,
                        );
                }
            }
        }
    }
}

fn ball_wall_bounce_system(
    mut bounce_writer: EventWriter<WallBounceEvent>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    if let Ok((mut transform, mut velocity)) = ball_query.single_mut() {
        let hit_top_wall = (transform.translation.y + 0.5 * BALL_SIZE) >= SCREEN_HEIGHT / 2.;
        let hit_bottom_wall =
            (transform.translation.y - 0.5 * BALL_SIZE) <= (-1. * SCREEN_HEIGHT / 2.);
        if hit_top_wall || hit_bottom_wall {
            bounce_writer.send(WallBounceEvent);
            velocity.0.y = -1. * velocity.0.y;
            // Stop ball from getting stuck if it's velocity isn't enough to move it out of
            // the wall in the next frame
            if hit_bottom_wall {
                transform.translation.y += 0.5 * BALL_SIZE
            }
            if hit_top_wall {
                transform.translation.y -= 0.5 * BALL_SIZE
            }
        };
    }
}

fn velocity_system(time: Res<Time>, mut q: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, Velocity(vec2)) in q.iter_mut() {
        let delta_time_vector = Vec3::splat(time.delta_seconds());
        transform.translation += Vec3::from((*vec2, 0.)) * delta_time_vector;
    }
}
