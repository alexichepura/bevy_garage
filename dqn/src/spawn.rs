use crate::dqn_bevy::CarDqn;
use bevy::prelude::*;
use bevy_garage_car::{sensor::CarSensors, Car};

pub fn add_dqn_on_spawned_car_system(
    query: Query<(Entity, &Car), Added<Car>>,
    mut commands: Commands,
) {
    for (car_entity, car) in &query {
        commands
            .entity(car_entity)
            .insert(CarDqn::new())
            .insert(CarSensors::new(&car.size));
    }
}
