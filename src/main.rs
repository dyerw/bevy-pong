use bevy::prelude::*;

mod collision;
mod components;
mod constants;
mod events;
mod physics;
mod resources;
mod sound;
mod spawn;

fn main() {
    App::build()
        .add_plugin(resources::PongResourcesPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(spawn::PongSpawnPlugin)
        .add_plugin(physics::PongPhysicsPlugin)
        .add_plugin(events::PongEventsPlugin)
        .add_plugin(collision::PongCollisionPlugin)
        .add_plugin(sound::PongSoundPlugin)
        // Systems
        .add_startup_system(setup.system())
        .add_system(paddle_movement.system())
        .add_system(collision_debugger.system())
        .add_system(ball_reset.system())
        .add_system(score_system.system())
        .run();
}

// Systems

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // Text
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(components::ScoreText);
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

fn collision_debugger(mut collision_reader: EventReader<events::CollisionEvent>) {
    for event in collision_reader.iter() {
        eprintln!("{:?} collided with {:?}!", event.0, event.1);
    }
}

fn ball_reset(
    mut ball_query: Query<&mut Transform, With<components::Ball>>,
    mut score_writer: EventWriter<events::ScoreEvent>,
) {
    if let Ok(mut transform) = ball_query.single_mut() {
        let x_pos = transform.translation.x;
        if x_pos > constants::SCREEN_WIDTH / 2. {
            score_writer.send(events::ScoreEvent(events::Player::Left));
            transform.translation.x = 0.;
        }
        if x_pos < -1. * (constants::SCREEN_WIDTH / 2.) {
            score_writer.send(events::ScoreEvent(events::Player::Right));
            transform.translation.x = 0.;
        }
    }
}

fn score_system(
    mut score_reader: EventReader<events::ScoreEvent>,
    mut text_query: Query<&mut Text, With<components::ScoreText>>,
    mut score_resource: ResMut<resources::Score>,
) {
    for event in score_reader.iter() {
        if let Ok(mut text) = text_query.single_mut() {
            match event.0 {
                events::Player::Left => {
                    score_resource.left_player += 1;
                    text.sections[0].value = score_resource.left_player.to_string()
                }
                events::Player::Right => {
                    score_resource.right_player += 1;
                    text.sections[1].value = score_resource.right_player.to_string()
                }
            }
        }
    }
}
