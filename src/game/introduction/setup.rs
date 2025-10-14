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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>, 
) {
    commands.spawn(Camera2d);

    // commands.spawn((
    //     Mesh2d(meshes.add(Rectangle::new(1175.0, 156.0))), // 1152 is the weight of the dialogue UI, 135 is the height dialogue UI should be!
    //     MeshMaterial2d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.9))),
    //     Transform::from_xyz(0.0, -340.0, 1.0), 
    // )); 
}
