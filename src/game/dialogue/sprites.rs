use bevy::prelude::*;
use std::collections::HashMap;
use super::{
    data::{DialogueData, DialogueSprite, ActiveSprite}, 
    state::ActiveDialogue
};

#[derive(Resource)]
pub struct SpriteCache {
    pub loaded: HashMap<String, Handle<Image>>,
}

#[derive(Component)]
pub struct ActiveDialogueSprite; 

fn manage_sprites(
    mut commands: Commands,
    mut portrait_cache: ResMut<SpriteCache>,
    active: Res<ActiveDialogue>,
    asset_server: Res<AssetServer>, 
    data_query: Query<&DialogueData>, 
    existing_sprites: Query<(Entity, &DialogueSprite), With<ActiveDialogueSprite>>, 
) {
    if !active.is_changed() {
        return; 
    }

    let Ok(data) = data_query.get(active.source) else {return;}; 
    let current_node = &data.nodes[&active.node_id]; 
    let Some(preset) = &current_node.sprite_preset else {
        for (entity, _) in existing_sprites.iter() {
            commands.entity(entity).despawn(); 
        }
        return; 
    }; 

    let mut existing = false;
    for (entity, sprite) in existing_sprites.iter() {
        if sprite.speaker == current_node.speaker && sprite.variant == current_node.variant {
            existing = true; 
            break; 
        }
    }

    if existing {
        return; 
    }

    for (entity, _) in existing_sprites.iter() {
        commands.entity(entity).despawn();
    }

    if let Some(handle) = try_load_portrait(
        &current_node.speaker,
        current_node.variant.as_deref(),
        &asset_server,
        &mut portrait_cache,
    ) {
        let position = preset.to_vec3(); 

        commands.spawn((
            Sprite::from_image(handle),
            Transform::from_translation(position),
            DialogueSprite {
                speaker: current_node.speaker.clone(),
                variant: current_node.variant.clone(),
            },
            ActiveSprite,
        )); 
    }
}