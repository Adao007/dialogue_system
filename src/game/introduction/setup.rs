use super::scene;
use bevy::prelude::*;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_view, scene::init_scene).chain());
    }
}

fn init_view(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}
