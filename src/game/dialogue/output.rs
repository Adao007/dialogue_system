use super::{data::DialogueData, state::{ActiveDialogue, DialogueState}, ui::{DialogueText, Title}};
use bevy::prelude::*;

pub fn output_text(
    mut query: Query<(Entity, &mut DialogueText), With<Text>>,
    mut writer: TextUiWriter,
    mut active: ResMut<ActiveDialogue>,
    time: Res<Time>,
) {
    if active.state != DialogueState::Output {
        return;
    }

    for (entity, mut text) in query.iter_mut() {
        text.elapsed_time += time.delta_secs();
        *writer.text(entity, 0) = text.get_visible();

        if text.is_complete() {
            active.state = DialogueState::Confirmation;
        }
    }
}

pub fn output_speaker(
    active: Res<ActiveDialogue>, 
    data_query: Query<&DialogueData>,
    mut speaker_query: Query<&mut Text, With<Title>>,
) {
    if active.is_changed() {
        let Ok(data) = data_query.get(active.source) else {return;}; 
        let current_node = &data.nodes[&active.node_id]; 

        for mut text in speaker_query.iter_mut() {
            **text = current_node.speaker.clone(); 
        }
    }
}