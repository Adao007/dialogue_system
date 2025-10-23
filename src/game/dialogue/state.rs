use super::{
    data::{DialogueData},
}; 
use bevy::prelude::*;

#[derive(Resource)]
pub struct ActiveDialogue {
    pub source: Entity,
    pub node_id: String,
    pub state: DialogueState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogueState {
    Output,
    Confirmation,
    Response,
    End,
}

#[derive(Message, Debug)]
pub struct SpriteSpawner; 

// Helper Systems
pub fn active_dialogue(
    mut commands: Commands,
    mut query: Query<(Entity, &DialogueData)>,
    input: Res<ButtonInput<KeyCode>>,
    mut tony_spawner: MessageWriter<SpriteSpawner>, 
) {
    if input.just_pressed(KeyCode::Enter) {
        tony_spawner.write(SpriteSpawner); 
        if let Ok((entity, data)) = query.single_mut() {
            commands.insert_resource(ActiveDialogue {
                source: entity,
                node_id: data.start_node.clone(),
                state: DialogueState::Output,
            });
        }
    }
}

pub fn end_dialogue(
    mut active: ResMut<ActiveDialogue>,
    data_query: Query<&DialogueData>,
) {
    let Ok(data) = data_query.get(active.source) else {return;}; 
    let current_node = &data.nodes[&active.node_id]; 

    if current_node.choices.is_empty() && current_node.next.is_none() && active.state == DialogueState::Confirmation {
        active.state = DialogueState::End;
    }
}