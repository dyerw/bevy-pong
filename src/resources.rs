use bevy::prelude::*;

use crate::constants::*;

pub struct PongResourcesPlugin;

impl Plugin for PongResourcesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
            .insert_resource(WindowDescriptor {
                title: "Pong".to_string(),
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                ..Default::default()
            })
            .insert_resource(Score::default())
            .add_startup_system(setup_resources_system.system());
    }
}

fn setup_resources_system(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(Materials {
        white_material: materials.add(Color::WHITE.into()),
    });
}

pub struct Materials {
    pub white_material: Handle<ColorMaterial>,
}

#[derive(Default)]
pub struct Score {
    pub left_player: i32,
    pub right_player: i32,
}
