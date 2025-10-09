use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

pub struct WindowsPlugin;
impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_window);
    }
}

fn init_window(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.single_mut() {
        window.resolution = WindowResolution::new(1920, 1080);
    }
}
