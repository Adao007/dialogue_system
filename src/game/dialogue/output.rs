
use super::{state::{ActiveDialogue, DialogueState}, ui::DialogueText};
use bevy::prelude::*;

fn output_text(
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
