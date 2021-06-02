use bevy::prelude::*;

mod collision;
mod components;
mod constants;
mod debugger;
mod events;
mod input;
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
        .add_plugin(input::PongInputPlugin)
        .add_plugin(ui::PongUiPlugin)
        .add_plugin(spawn::PongSpawnPlugin)
        .add_plugin(physics::PongPhysicsPlugin)
        .add_plugin(events::PongEventsPlugin)
        .add_plugin(collision::PongCollisionPlugin)
        .add_plugin(sound::PongSoundPlugin)
        .add_plugin(debugger::PongConsoleDebuggerPlugin)
        .add_plugin(scoring::PongScoringPlugin)
        .add_startup_system(camera_setup_system.system())
        .run();
}

// Systems

fn camera_setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
