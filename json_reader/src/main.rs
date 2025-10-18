use serde::{Deserialize, Serialize}; 
use std::collections::HashMap; 
use std::fs; 
use std::io::{self, Write}; 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DialogueNodeId{
    #[serde(rename = "example")]
    Example,
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
    pub next_node: DialogueNodeId, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: DialogueNodeId,
    pub speaker: String,
    pub text: String, 
    pub choices: Vec<DialogueChoice>,
}

pub fn parser(json_string: &str) -> Result<Vec<DialogueNode>, serde_json::Error> {
    serde_json::from_str(json_string)
}

pub fn insert_nodes(nodes: Vec<DialogueNode>) -> HashMap<DialogueNodeId, DialogueNode> {
    nodes
        .into_iter()
        .map(|node| (node.id, node))
        .collect()
}

pub fn load_dialogue(json_string: &str) -> Result<HashMap<DialogueNodeId, DialogueNode>, serde_json::Error> {
    let nodes: Vec<DialogueNode> = serde_json::from_str(json_string)?; 

    let hashmap = nodes.into_iter().map(|node| (node.id, node)).collect(); 

    Ok(hashmap)
}

struct DialogueState {
    nodes: HashMap<DialogueNodeId, DialogueNode>,
    current_node_id: DialogueNodeId,
}

impl DialogueState {
    fn new(nodes: HashMap<DialogueNodeId, DialogueNode>) -> Self {
        DialogueState {
            nodes,
            current_node_id: DialogueNodeId::Example,
        }
    }

    // Get the current node
    fn get_current_node(&self) -> Option<&DialogueNode> {
        self.nodes.get(&self.current_node_id)
    }

    // Move to the next node based on choice
    fn make_choice(&mut self, choice_index: usize) {
        if let Some(node) = self.get_current_node() {
            if let Some(choice) = node.choices.get(choice_index) {
                self.current_node_id = choice.next_node;
            }
        }
    }

    // Check if dialogue has ended
    fn is_dialogue_over(&self) -> bool {
        if let Some(node) = self.get_current_node() {
            node.choices.is_empty()
        } else {
            true
        }
    }
}

fn display_current_node(dialogue: &DialogueState) {
    println!("\n{}", "=".repeat(60));

    if let Some(node) = dialogue.get_current_node() {
        // Display speaker and text
        println!("[{}]", node.speaker);
        println!("{}", node.text);

        // Display choices if available
        if !node.choices.is_empty() {
            println!("\n{}", "-".repeat(60));
            println!("What do you do?");
            for (i, choice) in node.choices.iter().enumerate() {
                println!("  [{}] {}", i, choice.text);
            }
        } else {
            println!("\n(Dialogue ended)");
        }
    }

    println!("{}", "=".repeat(60));
}

fn get_user_choice(dialogue: &DialogueState) -> Option<usize> {
    if let Some(node) = dialogue.get_current_node() {
        if node.choices.is_empty() {
            return None;
        }

        if node.choices.len() == 1 && node.choices[0].text.to_lowercase() == "" {
            // Auto-advance without asking for input
            println!("\n(Continuing...)");
            return Some(0);
        }

        loop {
            print!("\nEnter your choice (0-{}): ", node.choices.len() - 1);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim().parse::<usize>() {
                Ok(choice) if choice < node.choices.len() => {
                    return Some(choice);
                }
                _ => {
                    println!("Invalid choice! Please enter a number between 0 and {}",
                        node.choices.len() - 1
                    );
                }
            }
        }
    }

    None
}

fn run_dialogue_loop(mut dialogue: DialogueState) {
    println!("\n{}", "*".repeat(60));
    println!("DIALOGUE SYSTEM");
    println!("{}", "*".repeat(60));

    loop {
        // Display current node
        display_current_node(&dialogue);

        // Check if dialogue is over
        if dialogue.is_dialogue_over() {
            println!("\nThanks for playing!");
            break;
        }

        // Get player input
        if let Some(choice_index) = get_user_choice(&dialogue) {
            dialogue.make_choice(choice_index);
        } else {
            break;
        }
    }
}

fn main() {
    let json_string = fs::read_to_string("assets/tutorial.dialogue")
        .expect("Failed to read assets/example.dialogue");

    // Parse JSON into HashMap
    let dialogue_map = load_dialogue(&json_string)
        .expect("Failed to parse JSON");

    // Create dialogue state
    let dialogue = DialogueState::new(dialogue_map);

    // Run the dialogue loop
    run_dialogue_loop(dialogue);
}
