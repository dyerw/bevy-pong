use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct CollisionEvent(pub Entity, pub Entity);

impl CollisionEvent {
    pub fn contains(&self, e: Entity) -> bool {
        return self.0 == e || self.1 == e;
    }
}

impl PartialEq for CollisionEvent {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.1 == other.0 && self.0 == other.1)
    }
}
impl Eq for CollisionEvent {}

pub enum Player {
    Right,
    Left,
}

pub struct ScoreEvent(pub Player);

pub struct WallBounceEvent;
