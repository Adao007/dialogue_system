use super::{data::DialogueData, state::{ActiveDialogue, DialogueState, SpriteSpawner}, ui::{DialogueText, Title}};
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

// For now spawn the Tony Pug
pub fn output_sprites(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut spawner_reader: MessageReader<SpriteSpawner>,
) {
    for _ in spawner_reader.read() {  
         let sprite= asset_server.load("Tony2.png");
         commands.spawn((
            Sprite::from_image(sprite),
            Transform::from_xyz(0.0, 15.0, 1.0), 
        ));
    }
}