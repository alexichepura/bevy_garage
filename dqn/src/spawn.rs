use crate::dqn_bevy::CarDqn;
use bevy::prelude::*;
use bevy_garage_car::car::Car;

pub fn add_dqn_on_spawned_car_system(query: Query<Entity, Added<Car>>, mut commands: Commands) {
    for car_entity in &query {
        commands.entity(car_entity).insert(CarDqn::new());
    }
}
