use crate::{joint::build_joint, spawn_wheel, CarRes, CarSpec, Wheel, WheelJoint};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct HID;

#[derive(Component, Debug)]
pub struct Car {
    pub gas: f32,
    pub brake: f32,
    pub steering: f32,
    pub spawn_transform: Transform,
    pub prev_steering: f32,
    pub prev_torque: f32,
    pub prev_dir: f32,
}
impl Default for Car {
    fn default() -> Self {
        Self {
            gas: 0.,
            brake: 0.,
            steering: 0.,
            prev_steering: 0.,
            prev_torque: 0.,
            prev_dir: 0.,
            spawn_transform: Transform::default(),
        }
    }
}
impl Car {
    pub fn new(spawn_transform: Transform) -> Self {
        Self {
            spawn_transform,
            ..default()
        }
    }
}
#[derive(Component, Debug)]
pub struct CarWheels {
    pub entities: [Entity; 4],
}
impl CarWheels {
    pub fn despawn(&mut self, commands: &mut Commands) {
        for e in self.entities.iter() {
            commands.entity(*e).despawn_recursive();
        }
    }
}

pub const STATIC_GROUP: Group = Group::GROUP_1;
pub const CAR_TRAINING_GROUP: Group = Group::GROUP_10;
pub fn car_start_system(mut config: ResMut<CarRes>, asset_server: Res<AssetServer>) {
    let wheel_gl: Handle<Scene> = asset_server.load("wheelRacing.glb#Scene0");
    config.wheel_scene = Some(wheel_gl.clone());
    let car_gl: Handle<Scene> = asset_server.load("car-race.glb#Scene0");
    config.car_scene = Some(car_gl.clone());
}

pub fn spawn_car(
    commands: &mut Commands,
    car_gl: &Handle<Scene>,
    wheel_gl: &Handle<Scene>,
    hid: bool,
    transform: Transform,
) -> Entity {
    let spec = CarSpec::default();

    let wheels_joints: [(Entity, GenericJoint); 4] = spec.wheel_mount.clone().map(|m| {
        let wheel = Wheel::new(spec.wheel_radius, spec.wheel_width, m.front, m.left);
        let translation = transform.translation + transform.rotation.mul_vec3(m.anchor);
        let wheel_id = spawn_wheel(commands, wheel_gl, wheel, translation);
        let joint = build_joint(m.anchor, m.left);
        (wheel_id, joint)
    });

    let car_id = spawn_car_body(
        commands,
        car_gl,
        Car::new(transform),
        spec,
        wheels_joints.map(|wj| wj.0),
    );
    if hid {
        commands.entity(car_id).insert(HID);
    }
    for (wheel_id, joint) in wheels_joints.iter() {
        commands
            .entity(*wheel_id)
            .insert(WheelJoint::new(car_id, *joint));
    }
    println!("spawn_car: {car_id:?}");
    return car_id;
}

pub fn spawn_car_body(
    commands: &mut Commands,
    car_gl: &Handle<Scene>,
    car: Car,
    spec: CarSpec,
    wheels: [Entity; 4],
) -> Entity {
    let car_border_radius = 0.1;
    let local_center_of_mass = Vec3::new(0., -spec.size.hh, 0.);
    let collider = Collider::round_cuboid(
        spec.size.hw - car_border_radius,
        spec.size.hh - car_border_radius,
        spec.size.hl - car_border_radius,
        car_border_radius,
    );
    let scene = SceneBundle {
        scene: car_gl.clone(),
        transform: car.spawn_transform,
        ..default()
    };
    commands
        .spawn((
            Name::new("car"),
            car,
            spec,
            CarWheels { entities: wheels },
            scene,
            (
                collider,
                ColliderMassProperties::MassProperties(MassProperties {
                    local_center_of_mass,
                    mass: 1000.0,
                    principal_inertia: Vec3::new(5000., 5000., 2000.), // https://www.nhtsa.gov/DOT/NHTSA/NRD/Multimedia/PDFs/VRTC/ca/capubs/sae1999-01-1336.pdf
                    ..default()
                }),
                Damping {
                    linear_damping: 0.05,
                    angular_damping: 0.1,
                },
                Friction::coefficient(0.5),
                Restitution::coefficient(0.),
                CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP),
                ActiveEvents::COLLISION_EVENTS,
                ContactForceEventThreshold(0.1),
            ),
            (
                Ccd::enabled(),
                CollidingEntities::default(),
                ColliderScale::Absolute(Vec3::ONE),
                ExternalForce::default(),
                ReadMassProperties::default(),
                RigidBody::Dynamic,
                Sleeping::disabled(),
                Velocity::zero(),
            ),
        ))
        .id()
}
