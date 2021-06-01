use bevy::prelude::*;

mod collision;
mod components;
mod constants;
mod debugger;
mod events;
mod physics;
mod resources;
mod scoring;
mod sound;
mod spawn;
mod ui;

fn main() {
    App::build()
        .add_plugin(resources::PongResourcesPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(ui::PongUiPlugin)
        .add_plugin(spawn::PongSpawnPlugin)
        .add_plugin(physics::PongPhysicsPlugin)
        .add_plugin(events::PongEventsPlugin)
        .add_plugin(collision::PongCollisionPlugin)
        .add_plugin(sound::PongSoundPlugin)
        .add_plugin(debugger::PongConsoleDebuggerPlugin)
        .add_plugin(scoring::PongScoringPlugin)
        .add_startup_system(camera_setup_system.system())
        .add_system(paddle_movement.system())
        .run();
}

// Systems

fn camera_setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn paddle_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query_set: QuerySet<(
        Query<&mut Transform, With<components::LeftPlayer>>,
        Query<&mut Transform, With<components::RightPlayer>>,
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
        transform.translation.y += time.delta_seconds() * direction * constants::PADDLE_SPEED;
    }
    if let Ok(mut transform) = player_query_set.q1_mut().single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            direction = 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction = -1.0;
        }
        transform.translation.y += time.delta_seconds() * direction * constants::PADDLE_SPEED;
    }
}
