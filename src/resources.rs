use bevy::prelude::*;

pub struct Materials {
    pub white_material: Handle<ColorMaterial>,
}

#[derive(Default)]
pub struct Score {
    pub left_player: i32,
    pub right_player: i32,
}
