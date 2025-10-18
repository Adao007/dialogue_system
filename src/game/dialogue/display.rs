use super::loader::*;
use super::scroll::ScrollPlugin;
use super::ui::UiPlugin;
use bevy::prelude::*;

pub struct DisplayPlugin;
impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, LoaderPlugin, ScrollPlugin))
            .add_message::<DialogueAdvanced>()
            .add_systems(
                Update,
                (
                    update_dialogue_text,
                    handle_dialogue_input,
                    advance_dialogue,
                ),
            );
    }
}

// Handles user space inputs to Full-Complete Text output or Continue to Next.
fn handle_dialogue_input(
    mut query: Query<(Entity, &mut DialogueText)>,
    mut writer: TextUiWriter,
    mut events: MessageWriter<DialogueAdvanced>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (entity, mut dialogue) in query.iter_mut() {
            if !dialogue.is_complete() {
                dialogue.show_all();
                *writer.text(entity, 0) = dialogue.get_visible();
            } else {
                events.write(DialogueAdvanced { entity });
            }
        }
    }
}

// Reads message from handle function to get next text.
fn advance_dialogue(
    mut query: Query<&mut DialogueText>,
    mut writer: TextUiWriter,
    mut events: MessageReader<DialogueAdvanced>,
    dialogue_box_query: Single<&mut DialogueBox>,
) {
    let mut dialogue_box = dialogue_box_query.into_inner();
    for event in events.read() {
        if let Ok(mut dialogue) = query.get_mut(event.entity) {
            // dialogue.set_text(node.text.clone());
            *writer.text(event.entity, 0) = dialogue.get_visible();
            dialogue_box.auto_scroll = true;
        }
    }
}
