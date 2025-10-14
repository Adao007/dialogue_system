use bevy::prelude::*;

pub struct ScrollPlugin; 
impl Plugin for ScrollPlugin {
    fn build(&self, app: &mut App) {

    }
}

#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
struct Scroll {
    entity: Entity, 
    delta: Vec2, 
}