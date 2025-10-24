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
    pub sprite_preset: Option<SpritePreset>,
    #[serde(default)]
    pub variant: Option<String>,
    #[serde(default)]
    pub events: Vec<String>, 
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

#[derive(Resource, Default)]
pub struct SpriteCache {
    pub loaded: HashMap<String, Handle<Image>>,
}