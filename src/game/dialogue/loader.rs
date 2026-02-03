use super::data::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;

// const JSON_DIALOGUE: &str = "assets/dialogue/presentation.dialogue";

// pub fn load_dialogue(mut commands: Commands) {
//     let json_string = fs::read_to_string(JSON_DIALOGUE).expect("Failed to read JSON file.");
//     let dialogue_map: HashMap<String, DialogueNode> = load_map(&json_string).expect("Failed to parse JSON.");
//     commands.spawn(DialogueData::new(dialogue_map, "start".to_string()));
// }

// fn load_map(json_string: &str) -> Result<HashMap<String, DialogueNode>, serde_json::Error> {
//     let nodes: Vec<DialogueNode> = serde_json::from_str(json_string)?;
//     let hashmap: HashMap<String, DialogueNode> = nodes.into_iter().map(|node| (node.id.clone(), node)).collect();
//     Ok(hashmap)
// }

use std::path::Path; // Type supports a number of operations for inspecting a path. 
// Separate by "/" on Unix and by either "/" or "\" on Windows. 

use std::error::Error; // Boxing errors: simple code while preserving the original errors. 

/* --- Systems --- */
// Change capacity to handle RON files, maintain JSON capabilities. 
fn load_dialogue_file(
    content: &str, 
    file_path: &str
) -> Result<Vec<DialogueNode>, Box<dyn std::error::Error>> {
    
    // returns the extension of the file: file.ron -> "ron"
    let extension = Path::new(file_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or(""); 

    let nodes: Vec<DialogueNode> = match extension {
        "json" | "dialogue" => serde_json::from_str(content)?, 
        "ron" => ron::from_str(content)?,
        _ => return Err(format!("Unsupported format: {}", extension).into()),
    }; 

    Ok(nodes)
}

