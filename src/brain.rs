use bevy::prelude::*;
use bevy_polyline::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::thread_rng;
use rand::{distributions::Standard, Rng};

use crate::car::*;

#[derive(Debug)]
pub struct CarBrain {
    levels: Vec<Level>,
}
impl CarBrain {
    pub fn new() -> CarBrain {
        let ins = Level::new(5, 6);
        let hidden = Level::new(6, 4);
        let outs = Level::new(4, 0);
        CarBrain {
            levels: [ins, hidden, outs].to_vec(),
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
) {
    for _ in 0..5 {
        commands
            .spawn_bundle(PolylineBundle {
                polyline: polylines.add(Polyline {
                    vertices: vec![-Vec3::ONE, Vec3::ONE],
                    ..Default::default()
                }),
                material: polyline_materials.add(PolylineMaterial {
                    width: 2.0,
                    color: Color::RED,
                    perspective: false,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(CarSensor);
    }
}
pub fn car_brain_system(
    rapier_context: Res<RapierContext>,
    mut cars: Query<(&mut Car, &Transform, With<Car>)>,
    mut polylines: ResMut<Assets<Polyline>>,
    // mut sensors: Query<(&mut Polyline, With<CarSensor>)>,
    sensors: Query<(Entity, &Handle<Polyline>)>,
) {
    let (mut car, tf, _car) = cars.single_mut();
    let mut meters: Vec<f32> = Vec::new();
    let max_toi: f32 = 10.;
    sensors.for_each(|(_, polyline)| {
        let ray_origin: Vect = tf.translation + tf.rotation.mul_vec3(Vec3::new(0., 0., 2.));
        let ray_dir: Vect = tf.rotation.mul_vec3(Vec3::new(0., 0., max_toi));
        polylines.get_mut(polyline).unwrap().vertices = vec![ray_origin, ray_origin + ray_dir];

        let hit = rapier_context.cast_ray(
            ray_origin,
            ray_dir,
            max_toi,
            false,
            InteractionGroups::default(),
            None,
        );
        match hit {
            Some((_, sensor_units)) => {
                if sensor_units > 1. {
                    meters.push(0.);
                    return;
                }
                meters.push(sensor_units * max_toi);
            }
            None => (),
        }
    });
    if meters.len() == 0 {
        meters = vec![0., 0., 0., 0., 0.];
    }
    // println!("Meters {:?}", meters);
    car.brain.feed_forward(meters);

    // println!("{:?}", car.brain);
    // let torque: f32 = 200.;
    // for (mut forces, transform, _) in wheels.iter_mut() {
    //     forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., torque, 0.))).into();
    // }
}
