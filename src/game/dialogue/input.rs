use bevy::prelude::*;
use super::{state::{ActiveDialogue, DialogueState}, ui::{DialogueRoot, DialogueText, DialogueBox}, data::{DialogueData, DialogueNodeId}};

#[derive(Message)]
struct DialogueAdvanced {
    entity: Entity,
}

fn handle_input(
    mut text_query: Query<(Entity, &mut DialogueText)>,
    mut box_query: Query<&mut DialogueBox, With<DialogueRoot>>, 
    mut writer: TextUiWriter,
    mut active: ResMut<ActiveDialogue>,
    data_query: Query<&DialogueData>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    match active.state {
        DialogueState::Output => {
            for (entity, mut text) in text_query.iter_mut() {
                text.show_all();
                *writer.text(entity, 0) = text.get_visible();
            }
            active.state = DialogueState::Confirmation;
        }
        DialogueState::Confirmation => {
            let Ok(data) = data_query.get(active.source) else { return }; 

            let current_node = &data.nodes[&active.node_id]; 

            if !current_node.choices.is_empty() {
                active.state = DialogueState::Response;
            }
            else if let Some(next_id) = current_node.next {
                advance(&mut active, &mut text_query, &mut box_query, data, next_id);
            }
            else {

            }
            
        }
        DialogueState::Response => {}
    }
}

fn advance(
    active: &mut ActiveDialogue,
    text_query: &mut Query<(Entity, &mut DialogueText)>, 
    box_query: &mut Query<&mut DialogueBox, With<DialogueRoot>>,
    data: &DialogueData, 
    next_node_id: DialogueNodeId,
) {
    active.node_id = next_node_id; 
    
    let new_node = &data.nodes[&next_node_id];
    for (_entity, mut text) in text_query.iter_mut() {
        text.set_text(new_node.text.clone()); 
    }

    
    if let Ok(mut text) = box_query.single_mut() {
        text.auto_scroll = true; 
    }
    
    active.state = DialogueState::Output; 
}