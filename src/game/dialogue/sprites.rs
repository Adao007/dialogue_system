use bevy::prelude::*;
use super::{
    data::{DialogueData, DialogueSprite, ActiveSprite, SpriteCache}, 
    state::ActiveDialogue,
};

#[derive(Component)]
pub struct ActiveDialogueSprite; 

pub fn manage_sprites(
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
    for (_entity, sprite) in existing_sprites.iter() {
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

    if let Some(handle) = load_portrait(
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

fn load_portrait(
    speaker: &str,
    variant: Option<&str>, 
    asset_server: &AssetServer, 
    cache: &mut SpriteCache, 
) -> Option<Handle<Image>> {
    let paths = if let Some(var) = variant {
        vec![
            format!("sprites/{}_{}.png", speaker.to_lowercase(), var),
            format!("sprites/{}.png", speaker.to_lowercase()), 
        ]
    } else {
        vec![format!("sprites/{}.png", speaker.to_lowercase())]
    }; 

    for path in paths {
        if let Some(handle) = cache.loaded.get(&path) {
            return Some(handle.clone());
        }

        let handle = asset_server.load(&path); 
        cache.loaded.insert(path.clone(), handle.clone()); 
        return Some(handle); 
    }

    None
}

pub fn cleanup_sprites(
    mut commands: Commands,
    active: Option<Res<ActiveDialogue>>, 
    sprites: Query<Entity, With<ActiveSprite>>,
) {
    if active.is_none() {
        for entity in sprites.iter() {
            commands.entity(entity).despawn(); 
        }
    }
}