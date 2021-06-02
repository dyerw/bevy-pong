use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::events::*;
use crate::resources::*;

pub struct PongUiPlugin;

impl Plugin for PongUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui_system.system())
            .add_system(update_score_ui_system.system().label(UI_LABEL));
    }
}

fn setup_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        value: "-".to_string(),
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
        .insert(ScoreText);
}

fn update_score_ui_system(
    mut score_reader: EventReader<ScoreEvent>,
    score_res: Res<Score>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
) {
    for event in score_reader.iter() {
        if let Ok(mut text) = text_query.single_mut() {
            match event.0 {
                Player::Left => text.sections[0].value = score_res.left_player.to_string(),
                Player::Right => text.sections[2].value = score_res.right_player.to_string(),
            }
        }
    }
}
