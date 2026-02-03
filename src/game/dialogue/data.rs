use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum DialogueType {
    Story, // Highest Priority, Main Story triggered dialogue
    Event, // High Priority, Event triggered dialogue   
    Transactional, // Medium Priority, specific characters
    Casual, // Low Priority, random character interaction 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub text: String,
    pub next: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DialogueNode {
//     pub id: String,
//     #[serde[default]]
//     pub speaker: Option<String>,
//     pub text: String,
//     pub priority: i32,
//     pub weight: f32, 
//     pub dialogue_type: DialogueType,
//     pub skippable: bool, 
//     #[serde(default)] // Defaults missing values to Vec::new()
//     pub choices: Vec<DialogueChoice>,
//     pub conditions: Option<Vec<Condition>>, 
//     pub random_variants: Option<Vec<String>, 
//     #[serde(default)]
//     pub sprite_preset: Option<SpritePreset>,
//     #[serde(default)]
//     pub variant: Option<String>,
//     #[serde(default)]
//     pub events: Vec<String>, 
// }
// ORGANIZE THE DIALOGUE NODE... 

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

// Need to separate the data structures for Sprites.
// -----------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpritePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpritePreset{
    Left,
    Right,
    Center,
    FarLeft,
    FarRight, 
    Custom(SpritePosition), 
}

impl SpritePreset {
    pub fn to_vec3(&self) -> Vec3{
        match self {
            Self::Left => Vec3::new(-250.0, -50.0, 10.0), 
            Self::Right => Vec3::new(250.0, -50.0, 10.0),
            Self::Center => Vec3::new(0.0, -50.0, 10.0),
            Self::FarLeft => Vec3::new(-400.0, -50.0, 10.0),
            Self::FarRight => Vec3::new(400.0, -50.0, 10.0), 
            Self::Custom(pos) => Vec3::new(pos.x, pos.y, pos.z), 
        }
    }
}

#[derive(Component)]
pub struct DialogueSprite {
    pub speaker: String,
    pub variant: Option<String>,
}

#[derive(Component)]
pub struct ActiveSprite;

#[derive(Resource, Default)]
pub struct SpriteCache {
    pub loaded: HashMap<String, Handle<Image>>,
}