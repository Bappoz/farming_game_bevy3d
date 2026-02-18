use bevy::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_terrain);
    }
}

fn setup_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene: Handle<Scene> = asset_server.load("indie_terrain.glb#Scene0");
    commands.spawn((SceneRoot(scene), Transform::from_scale(Vec3::splat(1.0))));
}
