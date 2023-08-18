use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_garage_camera::CarCameraPlugin;
use bevy_garage_car::{car_start_system, esp_system, spawn_car, Car, CarRes};
use bevy_garage_light::{animate_light_direction, light_start_system};
use bevy_overture_maps::*;
use bevy_rapier3d::prelude::*;

use crate::dash::{dash_fps_system, dash_speed_update_system, dash_start_system};

mod dash;

fn main() {
    let lat = std::env::var("MAP_LAT").expect("MAP_LAT env");
    let lat = lat.parse::<f64>().expect("lat to be f64");
    let lon = std::env::var("MAP_LON").expect("MAP_LON env");
    let lon = lon.parse::<f64>().expect("lon to be f64");
    let name = std::env::var("MAP_NAME").expect("MAP_NAME env");
    let lonlatname = format!("{lon}_{lat}_{name}");
    println!("{lonlatname}");

    let k = geodesic_to_coord(Coord { x: lon, y: lat });
    let center_xz: [f64; 2] = [lon * k[0], -lat * k[1]]; // Yto-Z

    let segments = query_transportation(TransportationQueryParams {
        from_string: format!("read_parquet('parquet/{lonlatname}_transportation.parquet')"),
        limit: None,
        k,
        center: center_xz,
    });
    let buildings = query_buildings(BuildingsQueryParams {
        from_string: format!("read_parquet('parquet/{lonlatname}_building.parquet')"),
        limit: None,
        k,
        center: center_xz,
    });
    App::new()
        .insert_resource(Buildings { buildings })
        .insert_resource(SegmentsRes { segments })
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Variable {
                max_dt: 1. / 60.,
                time_scale: 1.,
                substeps: 10,
            },
            ..default()
        })
        .insert_resource(CarRes {
            show_rays: true,
            ..default()
        })
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            CarCameraPlugin,
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .init_resource::<MapMaterialHandle>()
        .add_systems(
            Startup,
            (
                rapier_config_start_system,
                plane_start,
                transportations_start,
                buildings_start,
                light_start_system,
                car_start_system,
                spawn_car_system.after(car_start_system),
                dash_start_system,
            ),
        )
        .add_systems(
            Update,
            (
                input_system,
                esp_system.after(input_system),
                animate_light_direction,
                dash_fps_system,
                dash_speed_update_system,
            ),
        )
        .run();
}

fn rapier_config_start_system(mut c: ResMut<RapierContext>) {
    c.integration_parameters.max_velocity_iterations = 64;
    c.integration_parameters.max_velocity_friction_iterations = 64;
    c.integration_parameters.max_stabilization_iterations = 16;
    c.integration_parameters.erp = 0.99;
}

fn spawn_car_system(mut cmd: Commands, car_res: Res<CarRes>) {
    spawn_car(
        &mut cmd,
        &car_res.car_scene.as_ref().unwrap(),
        &car_res.wheel_scene.as_ref().unwrap(),
        true,
        Transform::from_translation(Vec3 {
            x: 0.,
            y: 1.,
            z: 0.,
        }),
    );
}

fn plane_start(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 5000.;
    let (cols, rows) = (10, 10);
    cmd.spawn((
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

    // cmd.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });

    // cmd.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0., 10., 20.).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}

fn input_system(
    input: Res<Input<KeyCode>>,
    mut cars: Query<&mut Car>,
    mut car_res: ResMut<CarRes>,
    mut debug_ctx: ResMut<bevy_rapier3d::render::DebugRenderContext>,
) {
    if input.just_pressed(KeyCode::R) {
        debug_ctx.enabled = !debug_ctx.enabled;
        car_res.show_rays = debug_ctx.enabled;
    }
    for mut car in cars.iter_mut() {
        if input.pressed(KeyCode::Up) {
            car.gas = 1.;
        }
        if input.just_released(KeyCode::Up) {
            car.gas = 0.;
        }

        if input.pressed(KeyCode::Down) {
            car.brake = 1.;
        }
        if input.just_released(KeyCode::Down) {
            car.brake = 0.;
        }

        if input.pressed(KeyCode::Left) {
            car.steering = -1.;
        }
        if input.pressed(KeyCode::Right) {
            car.steering = 1.;
        }
        if input.just_released(KeyCode::Left) {
            car.steering = 0.;
        }
        if input.just_released(KeyCode::Right) {
            car.steering = 0.;
        }
    }
}
