use super::data::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;

const JSON_DIALOGUE: &str = "assets/dialogue/lost.dialogue";

pub fn load_dialogue(mut commands: Commands) {
    let json_string = fs::read_to_string(JSON_DIALOGUE).expect("Failed to read JSON file.");
    let dialogue_map = load_map(&json_string).expect("Failed to parse JSON.");
    commands.spawn(DialogueData::new(dialogue_map, DialogueNodeId::Dummy));
}

fn load_map(json_string: &str) -> Result<HashMap<DialogueNodeId, DialogueNode>, serde_json::Error> {
    let nodes: Vec<DialogueNode> = serde_json::from_str(json_string)?;
    let hashmap = nodes.into_iter().map(|node| (node.id, node)).collect();
    Ok(hashmap)
}
