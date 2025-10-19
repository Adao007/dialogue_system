use bevy::prelude::*;

pub fn init_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_image = asset_server.load("barstow.png");
    commands.spawn((
        Sprite {
            image: scene_image.clone(),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

