use super::data::{DialogueData, DialogueNodeId};
use bevy::prelude::*;

#[derive(Resource)]
pub struct ActiveDialogue {
    pub source: Entity,
    pub node_id: DialogueNodeId,
    pub state: DialogueState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogueState {
    Output,
    Confirmation,
    Response,
}

// Helper Systems
pub fn active_dialogue(
    mut commands: Commands,
    mut query: Query<(Entity, &DialogueData)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Enter) {
        if let Ok((entity, data)) = query.single_mut() {
            commands.insert_resource(ActiveDialogue {
                source: entity,
                node_id: data.start_node,
                state: DialogueState::Output,
            });
        }
    }
}

pub fn end_dialogue(
    mut commands: Commands,
    active: Option<Res<ActiveDialogue>>,
    query: Query<&DialogueData>,
) {
    // Check to end, if ActiveDialogue exists
    // If None, do nothing
}
