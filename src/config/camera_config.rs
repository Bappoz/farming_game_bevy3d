use bevy::prelude::*;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

#[derive(Component)]
pub struct MyCameraMaker;

#[derive(Component)]
pub struct CameraLimits {
    #[allow(dead_code)]
    min_x: f32,
    #[allow(dead_code)]
    max_x: f32,
    #[allow(dead_code)]
    min_y: f32,
    #[allow(dead_code)]
    max_y: f32,
}

#[derive(Bundle)]
pub struct MyCameraBundle{
    camera: Camera2d,
    transform: Transform,
    global_transform: GlobalTransform,
    marker: MyCameraMaker,
    limits: CameraLimits,
}

impl Default for MyCameraBundle {
    fn default() -> Self {
        Self {
            camera: Camera2d,
            transform: Transform::from_xyz(SCREEN_WIDTH * 0.078, SCREEN_HEIGHT * 0.25, 0.0),
            global_transform: GlobalTransform::default(),
            marker: MyCameraMaker,
            limits: CameraLimits {
                min_x: -SCREEN_WIDTH * 0.39,
                max_x: SCREEN_WIDTH * 0.39,
                min_y: -SCREEN_HEIGHT * 0.375,
                max_y: SCREEN_HEIGHT * 0.375,
            }
        }
    }
}

#[allow(dead_code)]
pub struct CustomCameraPlugin;

// Plugin Personalizado
impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }    
}

#[allow(dead_code)]
fn setup_camera(mut commands: Commands) {
    commands.spawn(MyCameraBundle::default());
}