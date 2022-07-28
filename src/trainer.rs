use std::{cmp::Ordering, fs};

use crate::{brain::*, config::Config, progress::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

pub struct Trainer {
    pub generation: i32,
    pub record: f32,
    pub last_check_at: f64,
    pub best_brain: Option<CarBrain>,
}

impl Default for Trainer {
    fn default() -> Self {
        Self {
            generation: 0,
            record: 0.,
            last_check_at: 0.,
            best_brain: None,
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
    if !config.use_brain {
        return;
    }
    let seconds = time.seconds_since_startup();
    let interval = 5.;
    let seconds_diff = seconds - trainer.last_check_at;

    let mut q_trainer_timing = dash_set.p0();
    let mut text = q_trainer_timing.single_mut();
    let round_seconds = ((interval - seconds_diff) * 10.).round() / 10.;
    text.sections[1].value = round_seconds.to_string();

    if seconds_diff > interval {
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
        let (progress, best_brain, _, _) = best_car;
        trainer.best_brain = Some(best_brain.clone());
        let best_brain = best_brain.clone();

        let minimal_progress_delta = 1.;
        if progress.meters > (trainer.record + minimal_progress_delta) {
            println!("distance record {:.1}", progress.meters);
            trainer.record = progress.meters;
        } else {
            trainer.generation += 1;
            trainer.record = 0.;
            for (_i, (_progress, mut brain, mut transform, mut velocity)) in
                cars.iter_mut().enumerate()
            {
                let cloned_best: CarBrain = CarBrain::clone_randomised(&best_brain);
                brain.levels = cloned_best.levels.clone();
                transform.rotation = config.quat;
                transform.translation = config.translation; // + config.quat.mul_vec3(-Vec3::Z * 5. * i as f32);
                velocity.linvel = Vec3::ZERO;
                velocity.angvel = Vec3::ZERO;
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
