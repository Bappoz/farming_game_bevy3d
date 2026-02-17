use bevy::{prelude::*, tasks::futures_lite::stream::Or};
use blurpy::setup::{
    camera_setup::orbital::OrbitCameraPlugin, window_setup::window::GameWindowPlugin,
};

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Luz
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));

    // Cubo no centro
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::default(),
    ));
}

fn main() {
    App::new()
        .add_plugins(GameWindowPlugin)
        .add_plugins(OrbitCameraPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}
