use bevy::prelude::*;
use bevy_garage_car::{
    car::{car_start_system, spawn_car},
    config::CarConfig,
};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Variable {
                max_dt: 1. / 60.,
                time_scale: 1.,
                substeps: 10,
            },
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(CarConfig::default())
        .add_startup_systems((
            plane_start,
            car_start_system,
            spawn_car_system.after(car_start_system),
        ))
        .run();
}

fn spawn_car_system(mut commands: Commands, car_config: Res<CarConfig>) {
    spawn_car(
        &mut commands,
        &car_config.car_scene.as_ref().unwrap(),
        &car_config.wheel_scene.as_ref().unwrap(),
        true,
        Transform::from_translation(Vec3 {
            x: 0.,
            y: 1.,
            z: 0.,
        }),
        1200.,
    );
}

fn plane_start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 100.;
    let (cols, rows) = (10, 10);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(size).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        RigidBody::Fixed,
        ColliderScale::Absolute(Vec3::ONE),
        Friction::coefficient(3.),
        Restitution::coefficient(0.),
        Collider::heightfield(vec![0.; rows * cols], rows, cols, Vec3::new(size, 0., size)),
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 10., 25.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
