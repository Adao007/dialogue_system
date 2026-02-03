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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DialogueNode {
    // Core (always required)
    pub dialogue_type: DialogueType,
    pub id: String, 
    pub text: Vec<String>, // Multiple texts -> Randomly Selected

    // Optional include
    #[serde(default)]
    pub speaker: Option<String>, 

    #[serde(default)]
    pub sprites: SpriteConfig,
    
    #[serde(default)]
    pub selection: SelectionConfig, 

    #[serde(default)]
    pub conditions: Option<Vec<DialogueCondition>>, 

    #[serde(default)]
    pub player_choices: Option<Vec<DialogueChoice>>, 
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SpriteConfig {
    #[serde(default)]
    pub sprite_preset: Option<SpritePreset>, 

    #[serde(default)]
    pub sprite_variant: Option<String>, 
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SelectionConfig {
    #[serde(default)]
    pub priority: i32, // 0

    // Given equal priority, weight determines selection 
    #[serde(default = "default_weight")]
    pub weights: f32, // 1.0
}

fn default_weight () -> f32 {1.0}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DialogueChoice {
    pub text: String, 
    pub next: String, 
    pub conditions: Option<Vec<DialogueCondition>>, 
}

/* --- CONDITIONS --- */
// As of right now, we will keep Conditions here. 
// But I foresee this maybe it's own file. 
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DialogueCondition {
    pub condition_type: ConditionType, 
    // Can add UI hints like pud display_name: Option<String>
    // As long as ConditionType is separate. Leave room to expand. 
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ConditionType {
    // ... all possible condition types
    // EXAMPLES... 
    // Inventory: HasItem(String), HasCurrency{ amount: i32},
    // Quests: QuestActive(String), QuestComplete(String),
    // Stats: StatCheck {stat: String, min: i32}, SkillCheck {skill:String, min:i32},
    // Relational: RelationMeter {character: String, threshold: i32}, 
    // Location/Time: InLocation(String), TimeOfDay(start: i32, end:i32), 
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