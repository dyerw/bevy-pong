use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

pub struct PongSpawnPlugin;

impl Plugin for PongSpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_paddles.system())
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_ball.system());
    }
}

fn spawn_ball(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.white_material.clone(),
            sprite: Sprite::new(Vec2::new(BALL_SIZE, BALL_SIZE)),
            ..Default::default()
        })
        .insert(Velocity(Vec2::new(BALL_X_VELOCITY, 0.)))
        .insert(Collider(Vec2::new(BALL_SIZE, BALL_SIZE)))
        .insert(Ball);
}

fn spawn_paddles(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.white_material.clone(),
            sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            transform: Transform::from_xyz(-1. * (SCREEN_WIDTH / 2.) + 20., 0., 0.),
            ..Default::default()
        })
        .insert(Paddle)
        .insert(Collider(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)))
        .insert(LeftPlayer);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.white_material.clone(),
            sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            transform: Transform::from_xyz(SCREEN_WIDTH / 2. - 20., 0., 0.),
            ..Default::default()
        })
        .insert(Paddle)
        .insert(Collider(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)))
        .insert(RightPlayer);
}
