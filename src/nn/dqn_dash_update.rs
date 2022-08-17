use crate::{
    dash::*,
    dqn::{DqnResource, SYNC_INTERVAL_STEPS},
};
use bevy::prelude::*;

pub fn dqn_dash_update_system(
    mut dash_set: ParamSet<(
        Query<&mut Text, With<TrainerRecordDistanceText>>,
        Query<&mut Text, With<TrainerGenerationText>>,
    )>,
    dqn: NonSend<DqnResource>,
) {
    let mut q_generation_text = dash_set.p1();
    let mut generation_text = q_generation_text.single_mut();
    generation_text.sections[0].value = format!(
        "rb {:?}, sync {:?}",
        dqn.rb.len(),
        (dqn.step / SYNC_INTERVAL_STEPS)
    );

    let mut q_timing_text = dash_set.p0();
    let mut timing_text = q_timing_text.single_mut();
    timing_text.sections[0].value = format!("epsilon {:.3}", dqn.eps);
}
