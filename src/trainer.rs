use crate::{brain::*, car::Car, config::Config, progress::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::{cmp::Ordering, fs};
use std::{fs::File, path::Path};

pub struct Trainer {
    pub interval: f64,
    pub generation: i32,
    pub record: f32,
    pub last_check_at: f64,
    pub best_brain: Option<CarBrain>,
    pub sensor_count: usize,
}

impl Trainer {
    pub fn get_brain(&self) -> CarBrain {
        let brain = match self.best_brain {
            Some(ref b) => CarBrain::clone_randomised(&b),
            None => CarBrain::new(7),
        };
        brain
    }
}

impl Default for Trainer {
    fn default() -> Self {
        let saved_brain: Option<CarBrain>;
        let json_file = File::open(Path::new("brain.json"));
        if json_file.is_ok() {
            println!("brain.json found");
            saved_brain = serde_json::from_reader(json_file.unwrap()).unwrap();
        } else {
            saved_brain = None;
        }
        Self {
            interval: 20.,
            generation: 0,
            record: 0.,
            last_check_at: 0.,
            best_brain: saved_brain,
            sensor_count: 7,
        }
    }
}

#[derive(Component)]
pub struct TrainerTimingText;
#[derive(Component)]
pub struct TrainerRecordDistanceText;
#[derive(Component)]
pub struct TrainerGenerationText;

pub fn trainer_system(
    config: Res<Config>,
    mut trainer: ResMut<Trainer>,
    time: Res<Time>,
    mut cars: Query<
        (
            &mut CarProgress,
            &mut CarBrain,
            &mut Transform,
            &mut Car,
            &mut ExternalForce,
            &mut Velocity,
        ),
        With<CarProgress>,
    >,
    mut dash_set: ParamSet<(
        Query<&mut Text, With<TrainerTimingText>>,
        Query<&mut Text, With<TrainerRecordDistanceText>>,
        Query<&mut Text, With<TrainerGenerationText>>,
    )>,
) {
    let seconds = time.seconds_since_startup();
    if !config.use_brain {
        return;
    }
    let seconds_diff = seconds - trainer.last_check_at;

    let mut q_trainer_timing = dash_set.p0();
    let mut text = q_trainer_timing.single_mut();
    let round_seconds = ((trainer.interval - seconds_diff) * 10.).round() / 10.;
    text.sections[1].value = round_seconds.to_string();

    if seconds_diff > trainer.interval {
        trainer.last_check_at = seconds;

        let best_car = cars
            .iter()
            .max_by(|a, b| {
                if a.0.meters > b.0.meters {
                    return Ordering::Greater;
                }
                Ordering::Less
            })
            .unwrap();
        let (progress, best_brain, _, _, _, _) = best_car;
        trainer.best_brain = Some(best_brain.clone());
        let best_brain = best_brain.clone();

        let minimal_progress_delta = 1.;
        if progress.meters > (trainer.record + minimal_progress_delta) {
            println!("distance record {:.1}", progress.meters);
            trainer.record = progress.meters;
        } else {
            trainer.generation += 1;
            trainer.record = 0.;
            for (_i, (_progress, mut brain, mut t, mut car, mut f, mut v)) in
                cars.iter_mut().enumerate()
            {
                let cloned_best: CarBrain = CarBrain::clone_randomised(&best_brain);
                brain.levels = cloned_best.levels.clone();
                car.gas = 0.;
                car.brake = 0.;
                car.steering = 0.;
                *t = car.init_transform;
                *f = ExternalForce::default();
                *v = Velocity::zero();
            }
            println!("new generation {:?}", trainer.generation);

            let mut brain_dump = best_brain.clone();
            for level in brain_dump.levels.iter_mut() {
                level.inputs.fill(0.);
                level.outputs.fill(0.);
            }
            let serialized = serde_json::to_string(&brain_dump).unwrap();
            println!("saving brain.json");
            fs::write("brain.json", serialized).expect("Unable to write brain.json");
        }
    }

    let mut q_record_distance_text = dash_set.p1();
    let mut record_text = q_record_distance_text.single_mut();
    record_text.sections[1].value = ((trainer.record * 10.).round() / 10.).to_string();

    let mut q_generation_text = dash_set.p2();
    let mut generation_text = q_generation_text.single_mut();
    generation_text.sections[1].value = trainer.generation.to_string();
}

pub fn reset_collider_system(
    mut q_colliding_entities: Query<(&Parent, &CollidingEntities), With<CollidingEntities>>,
    mut q_parent: Query<(
        &mut Transform,
        &mut Car,
        &mut ExternalForce,
        &mut Velocity,
        &mut CarBrain,
    )>,
    q_name: Query<&Name>,
    trainer: Res<Trainer>,
) {
    for (p, colliding_entities) in q_colliding_entities.iter_mut() {
        for e in colliding_entities.iter() {
            let colliding_entity = q_name.get(e).unwrap();
            println!("colliding_entity {:?}", colliding_entity);
        }
        if !colliding_entities.is_empty() {
            let (mut t, mut car, mut f, mut v, mut car_brain) = q_parent.get_mut(p.get()).unwrap();
            car.gas = 0.;
            car.brake = 0.;
            car.steering = 0.;
            *car_brain = trainer.get_brain();
            *t = car.init_transform;
            *f = ExternalForce::default();
            *v = Velocity::zero();
        }
    }
}

pub fn reset_pos_system(
    mut q_car: Query<(&mut Transform, &mut Car, &mut ExternalForce, &mut Velocity)>,
) {
    for (mut t, mut car, mut f, mut v) in q_car.iter_mut() {
        if t.translation.y > 500. || t.translation.y < 0.
        // || v.linvel.length() > 100.
        // || v.angvel.length() > PI
        {
            println!("car is out of bound {:?}", t.translation.round());
            car.gas = 0.;
            car.brake = 0.;
            car.steering = 0.;
            *t = car.init_transform;
            *f = ExternalForce::default();
            *v = Velocity::zero();
        }
    }
}

pub fn reset_spawn_key_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Car, &mut Transform, &mut ExternalForce, &mut Velocity)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        println!("KeyCode::Space, cleanup");
        for (mut car, mut t, mut f, mut v) in query.iter_mut() {
            car.gas = 0.;
            car.brake = 0.;
            car.steering = 0.;
            *t = car.init_transform;
            *f = ExternalForce::default();
            *v = Velocity::zero();
        }
    }
}

// TODO velocity does not work
// pub fn reset_spawn_key_system(keys: Res<Input<KeyCode>>, mut q: Query<&mut Velocity>) {
//     if keys.just_pressed(KeyCode::Space) {
//         println!("KeyCode::Space, cleanup");
//         for mut v in &mut q {
//             println!("reset velocity");
//             *v = Velocity::zero();
//         }
//     }
// }

// TODO impulse does not work
// pub fn reset_spawn_key_system(
//     keys: Res<Input<KeyCode>>,
//     mut q: Query<&mut ExternalImpulse, &Wheel>,
// ) {
//     if keys.just_pressed(KeyCode::Space) {
//         println!("KeyCode::Space, cleanup");
//         for mut impulse in &mut q {
//             println!("impulse.impulse = Vec3::Y * 1_000_000.;");
//             impulse.impulse = Vec3::X * 1_000_000.;
//         }
//     }
// }

// https://github.com/dimforge/bevy_rapier/issues/196
// pub fn reset_spawn_key_system(
//     keys: Res<Input<KeyCode>>,
//     mut commands: Commands,
//     query: Query<Entity, With<MultibodyJoint>>,
// ) {
//     if !keys.just_pressed(KeyCode::Space) {
//         return;
//     }
//     println!("KeyCode::Space, cleanup");
//     for e in query.iter() {
//         println!("cleanup MultibodyJoint");
//         /// commands.entity(e).remove::<MultibodyJoint>();
//         commands.entity(e).despawn();
//     }
// }
