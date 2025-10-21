use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub text: String,
    pub next: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpritePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct DialogueSprite {
    pub speaker: String,
    pub variant: Option<String>,
}

#[derive(Component)]
pub struct ActiveSprite;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: String,
    pub speaker: String,
    pub text: String,
    #[serde(default)] // Defaults missing values to None
    pub next: Option<String>,
    #[serde(default)] // Defaults missing values to Vec::new()
    pub choices: Vec<DialogueChoice>,
    #[serde(default)]
    pub sprite_position: Option<SpritePosition>,
    #[serde(default)]
    pub variant: Option<String>,
}

#[derive(Component)]
pub struct DialogueData {
    pub nodes: HashMap<String, DialogueNode>,
    pub start_node: String,
}

impl DialogueData {
    pub fn new(nodes: HashMap<String, DialogueNode>, start: String) -> Self {
        DialogueData {
            nodes: nodes,
            start_node: start,
        }
    }
}
