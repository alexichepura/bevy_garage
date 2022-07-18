use bevy::prelude::*;
use bevy_polyline::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;
use rand::thread_rng;
use rand::{distributions::Standard, Rng};
use std::f32::consts::PI;

use crate::car::*;

fn car_lerp(a: f32, random_0_to_1: f32) -> f32 {
    let b = random_0_to_1 * 2. - 1.;
    let t = 0.1;
    a + (b - a) * t
}

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

    #[allow(dead_code)]
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        for level in self.levels.iter_mut() {
            for bias in level.biases.iter_mut() {
                *bias = car_lerp(*bias, rng.gen::<f32>());
            }
            for weighti in level.weights.iter_mut() {
                for weight in weighti.iter_mut() {
                    *weight = car_lerp(*weight, rng.gen::<f32>());
                }
            }
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

pub fn car_brain_system(
    rapier_context: Res<RapierContext>,
    mut cars: Query<(
        &mut Car,
        &Transform,
        &mut CarBrain,
        &Children,
        With<CarBrain>,
    )>,
    polylines: ResMut<Assets<Polyline>>,
    rays: Query<(Entity, &Handle<Polyline>)>,
) {
    for (mut car, transform, mut brain, children, _) in cars.iter_mut() {
        let mut inputs: Vec<f32> = Vec::new();
        let max_toi: f32 = 10.;

        for &child in children.iter() {
            let mut ray_origin: Vec3 = Vec3::ZERO;
            let mut ray_dir: Vec3 = Vec3::ZERO;

            if let Ok((_, polyline)) = rays.get(child) {
                let vertices = &polylines.get(polyline).unwrap().vertices;
                ray_origin = transform.translation + transform.rotation.mul_vec3(vertices[0]);
                ray_dir = transform.rotation.mul_vec3(vertices[1]);
            }

            let hit =
                rapier_context.cast_ray(ray_origin, ray_dir, max_toi, false, QueryFilter::new());
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
        }
        if inputs.len() != 5 {
            println!("inputs 5!={:?}", inputs);
            inputs = vec![0., 0., 0., 0., 0.];
        }

        if !car.use_brain {
            return;
        }

        brain.feed_forward(inputs.clone());

        let outputs: &Vec<f32> = &brain.levels.last().unwrap().outputs;

        let gas = outputs[0];
        let brake = outputs[1];
        let left = outputs[2];
        let right = outputs[3];

        car.gas = gas;
        car.brake = brake;
        car.steering = -left + right;
    }
}
