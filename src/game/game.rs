use super::{
    dialogue::DialoguePlugin,
    introduction::setup::SetupPlugin, 
    window::windows::WindowsPlugin,
};
use bevy::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DialoguePlugin, SetupPlugin, WindowsPlugin));
    }
}
