use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

pub struct PongInputPlugin;

impl Plugin for PongInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(paddle_movement_system.system());
    }
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query_set: QuerySet<(
        Query<&mut Transform, With<LeftPlayer>>,
        Query<&mut Transform, With<RightPlayer>>,
    )>,
) {
    if let Ok(mut transform) = player_query_set.q0_mut().single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::W) {
            direction = 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction = -1.0;
        }
        transform.translation.y += time.delta_seconds() * direction * PADDLE_SPEED;
    }
    if let Ok(mut transform) = player_query_set.q1_mut().single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            direction = 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction = -1.0;
        }
        transform.translation.y += time.delta_seconds() * direction * PADDLE_SPEED;
    }
}
