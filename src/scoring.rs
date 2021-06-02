use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::events::*;
use crate::resources::*;

pub struct PongScoringPlugin;

impl Plugin for PongScoringPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(score_detection_system.system())
            .add_system(ball_reset_system.system())
            .add_system(update_score_system.system().before(UI_LABEL));
    }
}

fn score_detection_system(
    ball_query: Query<&Transform, With<Ball>>,
    mut score_writer: EventWriter<ScoreEvent>,
) {
    if let Ok(transform) = ball_query.single() {
        let x_pos = transform.translation.x;
        if x_pos > SCREEN_WIDTH / 2. {
            score_writer.send(ScoreEvent(Player::Left));
        }
        if x_pos < -1. * (SCREEN_WIDTH / 2.) {
            score_writer.send(ScoreEvent(Player::Right));
        }
    }
}

fn ball_reset_system(
    mut score_reader: EventReader<ScoreEvent>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    for event in score_reader.iter() {
        if let Ok((mut transform, mut velocity)) = ball_query.single_mut() {
            transform.translation = Vec3::splat(0.);
            velocity.0.y = 0.;
            match event.0 {
                Player::Left => velocity.0.x = -1. * BALL_X_VELOCITY,
                Player::Right => velocity.0.x = BALL_X_VELOCITY,
            }
        }
    }
}

fn update_score_system(
    mut score_reader: EventReader<ScoreEvent>,
    mut score_resource: ResMut<Score>,
) {
    for event in score_reader.iter() {
        match event.0 {
            Player::Left => {
                score_resource.left_player += 1;
            }
            Player::Right => {
                score_resource.right_player += 1;
            }
        }
    }
}
