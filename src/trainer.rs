use std::cmp::Ordering;

use crate::{brain::*, config::Config, progress::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

pub struct Trainer {
    pub record: f32,
    pub last_check_at: f64,
    pub best_brain: Option<CarBrain>,
}

impl Default for Trainer {
    fn default() -> Self {
        Self {
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
    )>,
) {
    let seconds = time.seconds_since_startup();
    let interval = 10.;
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
                    // if a.0.meters - trainer.record > 1000. {
                    //     // prevents records by moving backwards
                    //     println!("distance antirecord {:.1}", a.0.meters);
                    //     return Ordering::Less;
                    // }
                    return Ordering::Greater;
                }
                Ordering::Less
            })
            .unwrap();
        let (progress, brain, _, _) = best_car;
        trainer.best_brain = Some(brain.clone());

        if progress.meters > trainer.record {
            println!("distance record {:.1}", progress.meters);
            trainer.record = progress.meters;
        } else {
            println!("no distance updates {:.1}", trainer.record);
            trainer.record = 0.;
            let cloned_best: CarBrain = CarBrain::clone_randomised(&brain);
            for (_progress, mut brain, mut transform, mut velocity) in cars.iter_mut() {
                brain.levels = cloned_best.levels.clone();
                transform.rotation = config.quat;
                transform.translation = config.translation;
                velocity.linvel = Vec3::ZERO;
                velocity.angvel = Vec3::ZERO;
            }
        }
    }

    let mut q_record_distance_text = dash_set.p1();
    let mut record_text = q_record_distance_text.single_mut();
    record_text.sections[1].value = ((trainer.record * 10.).round() / 10.).to_string();
}
