use crate::{brain::*, config::Config, progress::*};
use bevy::prelude::*;

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
pub struct TrainerTiming;

pub fn trainer_system(
    config: Res<Config>,
    mut trainer: ResMut<Trainer>,
    time: Res<Time>,
    mut cars: Query<(&mut CarProgress, &mut CarBrain, &mut Transform), With<CarProgress>>,
    mut q_trainer_timing: Query<&mut Text, With<TrainerTiming>>,
) {
    let seconds = time.seconds_since_startup();
    let interval = 10.;
    let seconds_diff = seconds - trainer.last_check_at;

    let mut text = q_trainer_timing.single_mut();
    text.sections[1].value = seconds_diff.to_string();

    if seconds_diff > interval {
        trainer.last_check_at = seconds;
        let mut record_updated = false;
        let mut best_brain: Option<CarBrain> = None;
        for (progress, brain, _) in cars.iter() {
            if progress.meters > trainer.record {
                trainer.record = progress.meters;
                record_updated = true;
                best_brain = Some(brain.clone());
            }
        }
        if !best_brain.is_some() {
            best_brain = trainer.best_brain.clone();
        }
        if !record_updated {
            trainer.best_brain = best_brain.clone();
            let cloned_best: CarBrain = CarBrain::clone_randomised(best_brain).unwrap();
            for (_progress, mut brain, mut transform) in cars.iter_mut() {
                brain.levels = cloned_best.levels.clone();
                transform.rotation = config.quat;
                transform.translation = config.translation;
            }
        }
    }
}
