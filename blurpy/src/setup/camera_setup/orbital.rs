use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, orbit_camera_system);
    }
}

#[derive(Component)]
pub struct OrbitCamera {
    pub radius: f32,
    pub focus: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        OrbitCamera {
            radius: 15.0,
            focus: Vec3::ZERO,
            yaw: 0.0,
            pitch: -0.5,
        },
    ));
}

fn orbit_camera_system(
    mut motion: MessageReader<MouseMotion>,
    mut scroll: MessageReader<MouseWheel>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut camera: Single<(&mut Transform, &mut OrbitCamera)>,
) {
    let (mut transform, mut orbit) = camera.into_inner();

    // ROTATION
    if buttons.pressed(MouseButton::Right) {
        for ev in motion.read() {
            orbit.yaw -= ev.delta.x * 0.005;
            orbit.pitch -= ev.delta.y * 0.005;
            orbit.pitch = orbit.pitch.clamp(-1.54, 1.54);
        }
    }

    // ZOOM
    for ev in scroll.read() {
        orbit.radius -= ev.y * 0.5;
        orbit.radius = orbit.radius.clamp(2.0, 50.0)
    }

    let rotation =
        Quat::from_axis_angle(Vec3::Y, orbit.yaw) * Quat::from_axis_angle(Vec3::X, orbit.pitch);

    transform.translation = orbit.focus + rotation * Vec3::new(0.0, 0.0, orbit.radius);
    transform.look_at(orbit.focus, Vec3::Y);
}
