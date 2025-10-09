use super::{introduction::setup, window::windows};
use bevy::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((setup::SetupPlugin, windows::WindowsPlugin));
    }
}
