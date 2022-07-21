use crate::car::*;
use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;
use rand::{distributions::Standard, Rng};
use serde::{Deserialize, Serialize};
use std::fs;

fn car_lerp(a: f32, random_0_to_1: f32) -> f32 {
    let b = random_0_to_1 * 2. - 1.;
    let t = 0.1;
    a + (b - a) * t
}

#[derive(Debug, Component, Clone, Serialize, Deserialize)]
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

    pub fn mutate_random(&mut self) {
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

    pub fn clone_randomised(brain: Option<CarBrain>) -> Option<CarBrain> {
        if let Some(brain) = brain {
            let mut rng = rand::thread_rng();
            let mut levels: Vec<Level> = vec![];
            for level in brain.levels.iter() {
                let mut cloned_level = level.clone();
                for bias in cloned_level.biases.iter_mut() {
                    *bias = car_lerp(*bias, rng.gen::<f32>());
                }
                for weighti in cloned_level.weights.iter_mut() {
                    for weight in weighti.iter_mut() {
                        *weight = car_lerp(*weight, rng.gen::<f32>());
                    }
                }
                levels.push(cloned_level)
            }
            return Some(CarBrain { levels });
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn reset_pos_system(
    car_init: Res<CarInit>,
    mut q_car: Query<(Entity, &mut Transform), With<Car>>,
    // mut commands: Commands,
) {
    for (_e, mut car_transform) in q_car.iter_mut() {
        // println!("{}", car_transform.rotation.to_axis_angle());
        if car_transform.translation.y > 10. || car_transform.translation.y < 0. {
            // commands.entity(e).despawn_recursive();
            // commands.entity(e).remove::<Collider>();
            println!("car is out of bound, resetting transform");
            *car_transform =
                Transform::from_translation(car_init.translation).with_rotation(car_init.quat);
        }
    }
}

pub fn car_brain_system(
    rapier_context: Res<RapierContext>,
    car_init: Res<CarInit>,
    mut q_car: Query<(Entity, &mut Car, &mut CarBrain, &Children), With<Car>>,
    q_near: Query<(&GlobalTransform, With<SensorNear>)>,
    q_far: Query<(&GlobalTransform, With<SensorFar>)>,
    mut ray_set: ParamSet<(
        Query<(&mut Transform, With<RayOrig>)>,
        Query<(&mut Transform, With<RayDir>)>,
        Query<(&mut Transform, With<RayHit>)>,
    )>,
    mut lines: ResMut<DebugLines>,
) {
    let sensor_filter = QueryFilter::new().exclude_dynamic().exclude_sensors();

    let e_hid_car = car_init.hid_car.unwrap();
    for (e, mut car, mut brain, children) in q_car.iter_mut() {
        let is_hid_car = e == e_hid_car;
        let mut origins: Vec<Vec3> = Vec::new();
        let mut dirs: Vec<Vec3> = Vec::new();

        for &child in children.iter() {
            if let Ok((gtrf, _)) = q_near.get(child) {
                origins.push(gtrf.translation);
            }
            if let Ok((gtrf, _)) = q_far.get(child) {
                dirs.push(gtrf.translation);
            }
        }

        let mut inputs: Vec<f32> = vec![0.; 5];
        let mut hit_points: Vec<Vec3> = vec![Vec3::ZERO; 5];
        let max_toi: f32 = 20.;
        let solid = false;
        for (i, &ray_dir_pos) in dirs.iter().enumerate() {
            let ray_pos = origins[i];
            if is_hid_car {
                lines.line_colored(
                    ray_pos,
                    ray_dir_pos,
                    0.0,
                    Color::rgba(0.25, 0.88, 0.82, 0.1),
                );
            }
            let ray_dir = (ray_dir_pos - ray_pos).normalize();
            rapier_context.intersections_with_ray(
                ray_pos,
                ray_dir,
                max_toi,
                solid,
                sensor_filter,
                |_entity, intersection| {
                    let toi = intersection.toi;
                    hit_points[i] = intersection.point;
                    if toi > 0. {
                        inputs[i] = 1. - toi / max_toi;
                        lines.line_colored(
                            ray_pos,
                            intersection.point,
                            0.0,
                            Color::rgba(0.98, 0.5, 0.45, 0.9),
                        );
                    } else {
                        inputs[i] = 0.;
                    }
                    false
                },
            );
        }

        for contact_pair in rapier_context.contacts_with(e) {
            let other_collider = if contact_pair.collider1() == e {
                contact_pair.collider2()
            } else {
                contact_pair.collider1()
            };
            println!("other_collider: {:?}", other_collider);
            for manifold in contact_pair.manifolds() {
                println!("Local-space contact normal: {}", manifold.local_n1());
                println!("Local-space contact normal: {}", manifold.local_n2());
                println!("World-space contact normal: {}", manifold.normal());
                for contact_point in manifold.points() {
                    println!("local contact point 1: {:?}", contact_point.local_p1());
                    println!("contact distance: {:?}", contact_point.dist());
                    println!("contact impulse: {}", contact_point.impulse());
                    println!("friction impulse: {:?}", contact_point.tangent_impulse());
                }
                for solver_contact in manifold.solver_contacts() {
                    println!("solver contact point: {:?}", solver_contact.point());
                    println!("solver contact distance: {:?}", solver_contact.dist());
                }
            }
        }
        if is_hid_car {
            for (i, (mut trf, _)) in ray_set.p0().iter_mut().enumerate() {
                trf.translation = origins[i];
            }
            for (i, (mut trf, _)) in ray_set.p1().iter_mut().enumerate() {
                trf.translation = dirs[i];
            }
            for (i, (mut trf, _)) in ray_set.p2().iter_mut().enumerate() {
                trf.translation = hit_points[i];
            }
            // println!(
            //     "inputs {:?}",
            //     inputs
            //         .iter()
            //         .map(|x| format!("{:.1} ", x))
            //         .collect::<String>(),
            // );
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

pub fn cars_pick_brain_mutate_restart(
    mut events: EventReader<PickingEvent>,
    mut cars: Query<(&mut CarBrain, &mut Transform, &mut Velocity, With<CarBrain>)>,
    car_init: Res<CarInit>,
) {
    let mut selected_brain_option: Option<CarBrain> = None;
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(e) => {
                println!("clicked entity {:?}", e);
                let (brain, _, _, _) = cars.get(*e).unwrap();
                let mut selected_brain = brain.clone();
                for level in selected_brain.levels.iter_mut() {
                    level.inputs.fill(0.);
                    level.outputs.fill(0.);
                }
                selected_brain_option = Some(selected_brain);
            }
            _ => (),
        }
    }
    if let Some(selected_brain) = selected_brain_option {
        let serialized = serde_json::to_string(&selected_brain).unwrap();
        println!("saving brain.json");
        fs::write("brain.json", serialized).expect("Unable to write brain.json");

        for (mut brain, mut transform, mut velocity, _) in cars.iter_mut() {
            let mut new_brain = selected_brain.clone();
            new_brain.mutate_random();
            *brain = new_brain;
            *transform =
                Transform::from_translation(car_init.translation).with_rotation(car_init.quat);
            *velocity = Velocity::default();
        }
    }
}
