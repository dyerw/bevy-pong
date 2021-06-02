use bevy::prelude::*;

use crate::events::*;

pub struct PongSoundPlugin;

impl Plugin for PongSoundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(load_sounds_system.system())
            .add_system(ping_sound_system.system())
            .add_system(wallbounce_sound_system.system())
            .add_system(score_sound_system.system());
    }
}

struct PingSound(pub Handle<AudioSource>);
struct WallBounceSound(pub Handle<AudioSource>);
struct ScoreSound(pub Handle<AudioSource>);

fn load_sounds_system(mut commands: Commands, server: Res<AssetServer>) {
    let ping_handle = server.load("sounds/ping.mp3");
    commands.insert_resource(PingSound(ping_handle));

    let wallbounce_handle = server.load("sounds/wallbounce.mp3");
    commands.insert_resource(WallBounceSound(wallbounce_handle));

    let score_handle = server.load("sounds/score.mp3");
    commands.insert_resource(ScoreSound(score_handle))
}

fn ping_sound_system(
    mut collision_reader: EventReader<CollisionEvent>,
    ping_res: Res<PingSound>,
    audio: Res<Audio>,
) {
    for _event in collision_reader.iter() {
        audio.play(ping_res.0.clone());
    }
}

fn wallbounce_sound_system(
    mut bounce_reader: EventReader<WallBounceEvent>,
    wallbounce_res: Res<WallBounceSound>,
    audio: Res<Audio>,
) {
    for _event in bounce_reader.iter() {
        audio.play(wallbounce_res.0.clone());
    }
}

fn score_sound_system(
    mut score_reader: EventReader<ScoreEvent>,
    score_res: Res<ScoreSound>,
    audio: Res<Audio>,
) {
    for _event in score_reader.iter() {
        audio.play(score_res.0.clone());
    }
}
