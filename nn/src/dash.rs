use bevy::prelude::Component;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TrainerEpsilonText;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TrainerGenerationText;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TrainerRewardsText;
