use bevy::prelude::*;

use crate::events::*;

pub struct PingSound(pub Handle<AudioSource>);
pub struct WallBounceSound(pub Handle<AudioSource>);

pub fn load_sounds(mut commands: Commands, server: Res<AssetServer>) {
    let ping_handle = server.load("sounds/ping.mp3");
    commands.insert_resource(PingSound(ping_handle));

    let wallbounce_handle = server.load("sounds/wallbounce.mp3");
    commands.insert_resource(WallBounceSound(wallbounce_handle));
}

pub fn ping_sound_system(
    mut collision_reader: EventReader<CollisionEvent>,
    ping_res: Res<PingSound>,
    audio: Res<Audio>,
) {
    for _event in collision_reader.iter() {
        audio.play(ping_res.0.clone());
    }
}

pub fn wallbounce_sound_system(
    mut bounce_reader: EventReader<WallBounceEvent>,
    wallbounce_res: Res<WallBounceSound>,
    audio: Res<Audio>,
) {
    for _event in bounce_reader.iter() {
        audio.play(wallbounce_res.0.clone());
    }
}
