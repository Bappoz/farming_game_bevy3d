use bevy::prelude::*;
use blurpy::setup::{
    camera_setup::orbital::OrbitCameraPlugin, window_setup::window::GameWindowPlugin,
    world_setup::terrain::TerrainPlugin,
};

fn setup_scene(mut commands: Commands) {
    // Luz
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
}

fn main() {
    App::new()
        .add_plugins(GameWindowPlugin)
        .add_plugins(OrbitCameraPlugin)
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}
