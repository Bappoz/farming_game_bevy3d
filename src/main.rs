mod config;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use config::camera_config::CustomCameraPlugin;

#[derive(Component)]
struct Player;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 800.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).into(),
                title: "Development".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CustomCameraPlugin)
        .add_message::<CollisionEvent>()
        .add_systems(Startup, (setup_ground, spawn_player, spawn_random_coin))
        .add_systems(Update, (player_movement, detect_collisions, handle_coin_pickup))
        .run();
}

fn setup_ground(mut commands: Commands) {
    // Chão - largura baseada na tela
    commands
        .spawn(Collider::cuboid(SCREEN_WIDTH * 0.78, 50.0))
        .insert(Transform::from_xyz(0.0, -SCREEN_HEIGHT * 0.25, 0.0))
        .insert(RigidBody::Fixed);
}

fn spawn_player(mut commands:Commands){
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(20.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, SCREEN_HEIGHT * 0.375, 0.0))
        .insert(Velocity::zero())
        .insert(Player);
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for (mut vel, mut transform) in &mut query {
        if keyboard_input.pressed(KeyCode::KeyA){
            vel.linvel.x = -300.0;
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            vel.linvel.x = 300.0;
        } else {
            vel.linvel.x = 0.0;
        }

        if keyboard_input.just_pressed(KeyCode::KeyR) {
            transform.translation = Vec3::new(0.0, SCREEN_HEIGHT * 0.375, 0.0);
            vel.linvel = Vec2::ZERO;
        }

        if keyboard_input.just_pressed(KeyCode::Space){
            vel.linvel.y = 400.0;
        }
    }
}

fn detect_collisions(mut collision_events: MessageReader<CollisionEvent>) {
    for collision_event in collision_events.read() {
        println!("Entidades colidiram : {:?}", collision_event);
    }
}



fn spawn_coin(mut commands: Commands, x: f32, y: f32) {
    commands
        .spawn(Collider::ball(10.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Transform::from_xyz(x, y, 0.0));
}

fn handle_coin_pickup(
    mut collision_events: MessageReader<CollisionEvent>,
    query_coins: Query<Entity, With<Sensor>>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            for coin in query_coins.iter() {
                if *e1 == coin || *e2 == coin {
                    println!("Moeda coletada!");
                    commands.entity(coin).despawn();
                }
            }
        }
    }
}

fn spawn_random_coin(commands: Commands) {
    let mut rng = rand::rng();
    let x = rng.random_range(-SCREEN_WIDTH * 0.3..SCREEN_WIDTH * 0.3);
    let y = rng.random_range(-SCREEN_HEIGHT * 0.125..SCREEN_HEIGHT * 0.375);
    spawn_coin(commands, x, y);
}