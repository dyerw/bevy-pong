use bevy::prelude::*;

use crate::events::*;

pub struct PongConsoleDebuggerPlugin;

impl Plugin for PongConsoleDebuggerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(collision_debugger_system.system());
    }
}

fn collision_debugger_system(mut collision_reader: EventReader<CollisionEvent>) {
    for event in collision_reader.iter() {
        eprintln!("{:?} collided with {:?}!", event.0, event.1);
    }
}
