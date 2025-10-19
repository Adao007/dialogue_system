use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub enum DialogueNodeId {
//     #[serde(rename = "dummy")]
//     Dummy,
//     #[serde(rename = "start")]
//     Start,
//     #[serde(rename = "start_continued")]
//     StartContinued,
//     #[serde(rename = "jupi_question")]
//     JupiQuestion,
//     #[serde(rename = "jupi_confused")]
//     JupiConfused,
//     #[serde(rename = "jupi_scared")]
//     JupiScared,
// }

pub enum DialogueNodeId {
    #[serde(rename = "dummy")]
    Dummy,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "go_to_node_a")]
    NodeA,
    #[serde(rename = "go_to_node_b")]
    NodeB,
    #[serde(rename = "linear_node")]
    LinearNode,
    #[serde(rename = "linear_part_two")]
    LinearNodeTwo, 
    #[serde(rename = "end_example")]
    EndExample, 
    #[serde(rename = "end_class")]
    EndClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub text: String,
    pub next: DialogueNodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: DialogueNodeId,
    pub speaker: String,
    pub text: String,
    #[serde(default)] // Defaults missing values to None
    pub next: Option<DialogueNodeId>,
    #[serde(default)] // Defaults missing values to Vec::new()
    pub choices: Vec<DialogueChoice>,
}

#[derive(Component)]
pub struct DialogueData {
    pub nodes: HashMap<DialogueNodeId, DialogueNode>,
    pub start_node: DialogueNodeId,
}

impl DialogueData {
    pub fn new(nodes: HashMap<DialogueNodeId, DialogueNode>, start: DialogueNodeId) -> Self {
        DialogueData {
            nodes: nodes,
            start_node: start,
        }
    }
}
