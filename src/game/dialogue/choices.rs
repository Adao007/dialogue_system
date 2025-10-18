use bevy::prelude::*; 
use super::state::{ActiveDialogue, DialogueState}; 

fn setup_reponses(
    active: Res<ActiveDialogue>,
) {
    if active.state != DialogueState::Response {
        return; 
    }
}

// Spawn UI for responses
// Fill UI with choices 