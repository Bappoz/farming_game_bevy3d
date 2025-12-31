use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_ball_altitude)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

}

fn setup_physics(mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<StandardMaterial>>) {
    // Create the ground
    commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(200.0, 0.2, 200.0))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.3, 0.5, 0.3),
                    ..default()
                })),
                Collider::cuboid(100.0, 0.1, 100.0),
                Transform::from_xyz(0.0, -2.0, 0.0),
            ));

    // Esferas
    create_sphere(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 4.0, 0.0),
        0.5,
        0.7,
        None,
        None,
        false,
        Color::srgb(1.0, 0.2, 0.2),
    );

    create_sphere(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(2.0, 6.0, 0.0),
        0.5,
        0.7,
        Some(Velocity {
            linvel: Vec3::new(-2.0, 2.0, 0.0),
            angvel: Vec3::new(0.2, 0.0, 0.0),
        }),
        Some(0.5),
        true,
        Color::srgb(0.2, 0.2, 1.0),
    );
}


fn create_sphere(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    radius: f32,
    restitution: f32,
    velocity: Option<Velocity>,
    gravity_scale: Option<f32>,
    enabled_ccd: bool,
    color: Color,
) {
    let mut entity = commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(radius),
        Restitution::coefficient(restitution),
        Transform::from_translation(position),

        Mesh3d(meshes.add(Sphere::new(radius))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            ..default()
        })),
    ));
    
    if let Some(vel) = velocity {
        entity.insert(vel);
    }
    
    if let Some(scale) = gravity_scale {
        entity.insert(GravityScale(scale));
    }

    if enabled_ccd {
        entity.insert(Ccd::enabled());
    }
    // Desabilita sleeping por padrão para objetos dinâmicos
    entity.insert(Sleeping::disabled());
}


fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
