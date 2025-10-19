use super::{
    choices::{ResponseUi},
    data::{DialogueData, DialogueNodeId},
    state::{ActiveDialogue, DialogueState},
    ui::{DialogueBox, DialogueRoot, DialogueText},
};
use bevy::prelude::*;

pub fn handle_input(
    mut text_query: Query<(Entity, &mut DialogueText)>,
    mut box_query: Query<&mut DialogueBox, With<DialogueRoot>>,
    mut writer: TextUiWriter,
    mut active: ResMut<ActiveDialogue>,
    commands: Commands, 
    dialogue_ui: Query<Entity, With<DialogueRoot>>, 
    response_ui_query: Query<Entity, With<ResponseUi>>,
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
            let Ok(data) = data_query.get(active.source) else {
                return;
            };

            let current_node = &data.nodes[&active.node_id];

            if !current_node.choices.is_empty() {
                active.state = DialogueState::Response;
            } else if let Some(next_id) = current_node.next {
                advance(&mut active, &mut text_query, &mut box_query, data, next_id);
            } 
        }
        DialogueState::End => {
            end(dialogue_ui, response_ui_query, commands); 
        }
        _ => {}
    }
}

pub fn advance(
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

fn end(
    dialogue_ui: Query<Entity, With<DialogueRoot>>, 
    response_ui_query: Query<Entity, With<ResponseUi>>,
    mut commands: Commands, 
) {
    for ui in dialogue_ui.iter() {
        commands.entity(ui).despawn();
    }

    for ui in response_ui_query.iter() {
        commands.entity(ui).despawn(); 
    }

    commands.remove_resource::<ActiveDialogue>();
}