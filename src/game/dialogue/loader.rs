use super::data::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;

const JSON_DIALOGUE: &str = "assets/dialogue/presentation.dialogue";

pub fn load_dialogue(mut commands: Commands) {
    let json_string = fs::read_to_string(JSON_DIALOGUE).expect("Failed to read JSON file.");
    let dialogue_map: HashMap<String, DialogueNode> = load_map(&json_string).expect("Failed to parse JSON.");
    commands.spawn(DialogueData::new(dialogue_map, "dummy".to_string()));
}

fn load_map(json_string: &str) -> Result<HashMap<String, DialogueNode>, serde_json::Error> {
    let nodes: Vec<DialogueNode> = serde_json::from_str(json_string)?;
    let hashmap: HashMap<String, DialogueNode> = nodes.into_iter().map(|node| (node.id.clone(), node)).collect();
    Ok(hashmap)
}
