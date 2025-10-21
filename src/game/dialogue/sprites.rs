use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct SpriteCache {
    pub loaded: HashMap<String, Handle<Image>>,
}
