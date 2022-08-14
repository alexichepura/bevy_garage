// use crate::track::ASSET_ROAD;
use crate::{brain::*, car::Car, config::Config, progress::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::{cmp::Ordering, fs};
use std::{fs::File, path::Path};

pub struct Trainer {
    pub interval: f64,
    pub generation: i32,
    pub record: f32,
    pub leader: Option<Entity>,
    pub last_check_at: f64,
    pub best_brain: Option<CarBrain>,
    pub sensor_count: usize,
}

impl Trainer {
    pub fn clone_best_brain_or_get_new(&self) -> CarBrain {
        let brain = match self.best_brain {
            Some(ref b) => CarBrain::clone_randomised(&b),
            None => CarBrain::new(8),
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
            interval: 10.,
            generation: 0,
            record: 0.,
            leader: None,
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

        let leader_car = cars
            .iter()
            .max_by(|a, b| {
                if a.0.meters > b.0.meters {
                    return Ordering::Greater;
                }
                Ordering::Less
            })
            .unwrap();
        let (leader_progress, leader_brain, _, _, _, _) = leader_car;

        let minimal_progress_delta = 1.;
        if leader_progress.meters > (trainer.record + minimal_progress_delta) {
            println!("distance record {:.1}", leader_progress.meters);
            trainer.record = leader_progress.meters;
        } else {
            trainer.generation += 1;
            trainer.record = 0.;
            if leader_progress.meters > 5. {
                println!("cloning leader_brain");
                let new_brain = leader_brain.clone();
                trainer.best_brain = Some(new_brain.clone());

                for (_progress, mut brain, mut t, mut car, mut f, mut v) in cars.iter_mut() {
                    brain.levels = CarBrain::clone_randomised(&new_brain).levels.clone();
                    car.gas = 0.;
                    car.brake = 0.;
                    car.steering = 0.;
                    *t = car.init_transform;
                    *f = ExternalForce::default();
                    *v = Velocity::zero();
                }
                println!("new generation {:?}", trainer.generation);

                let mut brain_dump = new_brain.clone();
                for level in brain_dump.levels.iter_mut() {
                    level.inputs.fill(0.);
                    level.outputs.fill(0.);
                }
                let serialized = serde_json::to_string(&brain_dump).unwrap();
                println!("saving brain.json");
                fs::write("brain.json", serialized).expect("Unable to write brain.json");
            } else {
                println!("small progress, getting new brains");
                println!("best_brain.is_some() {:?}", trainer.best_brain.is_some());
                for (_progress, mut brain, mut t, mut car, mut f, mut v) in cars.iter_mut() {
                    let new_brain = trainer.clone_best_brain_or_get_new();
                    brain.levels = new_brain.levels.clone();
                    car.gas = 0.;
                    car.brake = 0.;
                    car.steering = 0.;
                    *t = car.init_transform;
                    *f = ExternalForce::default();
                    *v = Velocity::zero();
                }
            }
        }
    }

    let mut q_record_distance_text = dash_set.p1();
    let mut record_text = q_record_distance_text.single_mut();
    record_text.sections[1].value = ((trainer.record * 10.).round() / 10.).to_string();

    let mut q_generation_text = dash_set.p2();
    let mut generation_text = q_generation_text.single_mut();
    generation_text.sections[1].value = trainer.generation.to_string();
}

// pub fn reset_collider_system(
//     mut q_colliding_entities: Query<(&Parent, &CollidingEntities), With<CollidingEntities>>,
//     q_name: Query<&Name>,
//     mut trainer: ResMut<Trainer>,
//     time: Res<Time>,
//     mut paramset: ParamSet<(
//         Query<(&mut Car, &mut CarBrain, &CarProgress)>,
//         Query<(&mut Transform, &mut Car, &mut ExternalForce, &mut Velocity)>,
//     )>,
//     config: Res<Config>,
// ) {
//     let seconds = time.seconds_since_startup();
//     let reset_at = seconds + 1.;
//     for (p, colliding_entities) in q_colliding_entities.iter_mut() {
//         let mut should_reset: bool = false;
//         for e in colliding_entities.iter() {
//             let colliding_entity = q_name.get(e).unwrap();
//             if !colliding_entity.contains(ASSET_ROAD) {
//                 should_reset = true;
//             }
//         }
//         if should_reset {
//             let mut q_parent = paramset.p0();
//             let (mut car, mut car_brain, progress) = q_parent.get_mut(p.get()).unwrap();
//             if car.reset_at.is_none() {
//                 println!("should_reset, car.reset_at=Some");
//                 if progress.place == 0 {
//                     trainer.record = 0.;
//                 }
//                 car.gas = 0.;
//                 car.brake = 0.;
//                 car.steering = 0.;
//                 car.use_brain = false;
//                 car.reset_at = Some(reset_at);
//                 *car_brain = trainer.clone_best_brain_or_get_new();
//             }
//         }
//     }
//     let mut q_car = paramset.p1();
//     for (mut t, mut car, mut f, mut v) in q_car.iter_mut() {
//         f.force = -v.linvel * 100.;
//         f.torque = -v.angvel * 100.;
//         if car.reset_at.unwrap_or(seconds) < seconds {
//             car.use_brain = config.use_brain;
//             car.reset_at = None;
//             *t = car.init_transform;
//             *f = ExternalForce::default();
//             *v = Velocity::zero();
//         }
//     }
// }

pub fn reset_pos_system(
    mut q_car: Query<(&mut Transform, &mut Car, &mut ExternalForce, &mut Velocity)>,
) {
    for (mut t, mut car, mut f, mut v) in q_car.iter_mut() {
        if t.translation.y > 500. || t.translation.y < -10. {
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
