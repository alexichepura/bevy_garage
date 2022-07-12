use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_polyline::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::thread_rng;
use rand::{distributions::Standard, Rng};

use crate::car::*;

#[derive(Debug, Component)]
pub struct CarBrain {
    levels: Vec<Level>,
}
impl CarBrain {
    pub fn new() -> CarBrain {
        let ins = Level::new(5, 6);
        let hidden = Level::new(6, 4);
        CarBrain {
            levels: [ins, hidden].to_vec(),
        }
    }
    pub fn feed_forward(&mut self, new_inputs: Vec<f32>) {
        let mut outputs: Vec<f32> = new_inputs.clone();
        for level in self.levels.iter_mut() {
            level.feed_forward(outputs.clone());
            outputs = level.outputs.clone();
        }
    }
}
#[derive(Debug, Clone)]
struct Level {
    inputs: Vec<f32>,
    outputs: Vec<f32>,
    weights: Vec<Vec<f32>>,
    biases: Vec<f32>,
}

impl Level {
    pub fn new(n_in: usize, n_out: usize) -> Level {
        let inputs: Vec<f32> = vec![0.; n_in];
        let outputs: Vec<f32> = vec![0.; n_out];
        let weights: Vec<Vec<f32>> = (0..n_in)
            .map(|_| thread_rng().sample_iter(Standard).take(n_out).collect())
            .collect();
        let biases: Vec<f32> = thread_rng().sample_iter(Standard).take(n_out).collect();

        Level {
            weights,
            biases,
            inputs,
            outputs,
        }
    }
    pub fn feed_forward(&mut self, new_inputs: Vec<f32>) {
        for (index, input) in self.inputs.iter_mut().enumerate() {
            *input = new_inputs[index];
        }
        for (index_out, output) in self.outputs.iter_mut().enumerate() {
            let mut sum: f32 = 0.;
            for (index_in, input) in self.inputs.iter_mut().enumerate() {
                sum = sum + *input * self.weights[index_in][index_out];
            }
            if sum > self.biases[index_out] {
                *output = 1.;
            } else {
                *output = 0.;
            }
        }
    }
}
#[derive(Component)]
pub struct CarSensor;

pub fn car_brain_start_system(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    // cars: Query<(&Transform, With<Car>)>,
) {
    for _ in 0..5 {
        commands
            .spawn_bundle(PolylineBundle {
                polyline: polylines.add(Polyline {
                    vertices: vec![-Vec3::ONE, Vec3::ONE],
                    ..default()
                }),
                material: polyline_materials.add(PolylineMaterial {
                    width: 2.0,
                    color: Color::RED,
                    perspective: false,
                    ..default()
                }),
                ..default()
            })
            .insert(CarSensor);
    }
    // let (transform, _car) = cars.single();
}
pub fn car_brain_system(
    rapier_context: Res<RapierContext>,
    mut cars: Query<(&mut Car, &Transform, With<Car>)>,
    mut brains: Query<(&mut CarBrain, With<CarBrain>)>,
    mut polylines: ResMut<Assets<Polyline>>,
    sensors: Query<(Entity, &Handle<Polyline>)>,
) {
    let (mut car, transform, _car) = cars.single_mut();
    let (mut brain, _brain) = brains.single_mut();
    let mut inputs: Vec<f32> = Vec::new();
    let max_toi: f32 = 10.;
    let mut i: i8 = 0;
    sensors.for_each(|(_, polyline)| {
        let mut line_tf = transform.clone();
        let angle = PI / 8.;
        let start_angle = -2. * angle;
        line_tf.rotate(Quat::from_rotation_y(start_angle + i as f32 * angle));
        i += 1;

        let ray_origin: Vect =
            line_tf.translation + line_tf.rotation.mul_vec3(Vec3::new(0., 0., 2.));
        let ray_dir: Vect = line_tf.rotation.mul_vec3(Vec3::new(0., 0., max_toi));

        polylines.get_mut(polyline).unwrap().vertices = vec![ray_origin, ray_origin + ray_dir];

        let hit = rapier_context.cast_ray(ray_origin, ray_dir, max_toi, false, QueryFilter::new());
        match hit {
            Some((_, sensor_units)) => {
                if sensor_units > 1. {
                    inputs.push(0.);
                    return;
                }
                inputs.push(sensor_units);
            }
            None => inputs.push(-1.),
        }
    });
    if inputs.len() != 5 {
        println!("inputs 5!={:?}", inputs);
        inputs = vec![0., 0., 0., 0., 0.];
    }
    brain.feed_forward(inputs.clone());
    let outputs: &Vec<f32> = &brain.levels.last().unwrap().outputs;
    // if outputs.iter().any(|v| v > &0.) {
    //     println!("outputs {:?}", outputs);
    // }

    let _gas = outputs[0];
    let _brake = outputs[1];
    let _left = outputs[2];
    let _right = outputs[3];

    // println!("{:?}", car.brain);
    // let torque: f32 = 200.;
    // for (mut forces, transform, _) in wheels.iter_mut() {
    //     forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., torque, 0.))).into();
    // }
}
