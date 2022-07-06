use bevy::prelude::*;
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
    pub fn feed_forward(&mut self, new_inputs: Vec<f64>) {
        let mut outputs: Vec<f64> = new_inputs.clone();
        for level in self.levels.iter_mut() {
            level.feed_forward(outputs.clone());
            outputs = level.outputs.clone();
        }
    }
}
#[derive(Debug, Clone)]
struct Level {
    inputs: Vec<f64>,
    outputs: Vec<f64>,
    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
}

impl Level {
    pub fn new(n_in: usize, n_out: usize) -> Level {
        let inputs: Vec<f64> = vec![0.; n_in];
        let outputs: Vec<f64> = vec![0.; n_out];
        let weights: Vec<Vec<f64>> = (0..n_in)
            .map(|_| thread_rng().sample_iter(Standard).take(n_out).collect())
            .collect();
        let biases: Vec<f64> = thread_rng().sample_iter(Standard).take(n_out).collect();

        Level {
            weights,
            biases,
            inputs,
            outputs,
        }
    }
    pub fn feed_forward(&mut self, new_inputs: Vec<f64>) {
        for (index, input) in self.inputs.iter_mut().enumerate() {
            *input = new_inputs[index];
        }
        for (index_out, output) in self.outputs.iter_mut().enumerate() {
            let mut sum: f64 = 0.;
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
pub fn car_brain_system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut cars: Query<(&mut Car, &Transform, With<Car>)>,
    bodies: Query<&RigidBody>,
    // mut wheels: Query<(&mut ExternalForce, &Transform, With<Wheel>)>,
) {
    let (mut car, transform, _car) = cars.single_mut();

    let ray_origin: Vect =
        transform.translation + transform.rotation.mul_vec3(Vec3::new(0., 0., 2.));
    let ray_dir: Vect = transform.rotation.mul_vec3(Vec3::new(0., 0., 1.));
    // println!("ray {:?} {:?}", transform.translation, transform.rotation);
    // println!("ray {:?}", ray_dir);
    let hit = rapier_context.cast_ray(
        ray_origin,
        ray_dir,
        f32::MAX,
        false,
        InteractionGroups::default(),
        None,
    );
    if let Some((entity, _toi)) = hit {
        // let e = commands.entity(entity);
        if let Ok(rb) = bodies.get(entity) {
            if *rb == RigidBody::Dynamic {
                let color = Color::BLUE; // Color in blue.
                commands.entity(entity).insert(ColliderDebugColor(color));
            }
            // let name = bodies.get_component(Name);
            println!(
                "HIT {} {} {}",
                (ray_origin * 10.).round() / 10.,
                (ray_dir * 10.).round() / 10.,
                (_toi * 10.).round() / 10.,
                // ((transform.rotation.xyz() * 10.).round() / 10.)
                //     .extend((transform.rotation.w * 10.).round() / 10.)
            )
        }
    }

    car.brain.feed_forward(vec![0., 0., 0., 0., 0.]);

    // println!("{:?}", car.brain);
    // let torque: f32 = 200.;
    // for (mut forces, transform, _) in wheels.iter_mut() {
    //     forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., torque, 0.))).into();
    // }
}
