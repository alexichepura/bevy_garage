use crate::dqn_bevy::CarDqn;
use bevy::prelude::*;
use bevy_garage_car::{sensor::CarSensors, Car, CarSpec};

pub fn add_dqn_on_spawned_car_system(
    query: Query<(Entity, &CarSpec), Added<Car>>,
    mut commands: Commands,
) {
    for (car_entity, spec) in &query {
        commands
            .entity(car_entity)
            .insert(CarDqn::new())
            .insert(CarSensors::new(&spec.size));
    }
}
