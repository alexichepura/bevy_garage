use crate::config::Config;
use bevy::prelude::*;
use bevy_garage_car::{car::spawn_car, config::CarConfig, spawn::SpawnCarEvent};

pub fn spawn_car_system(
    mut events: EventReader<SpawnCarEvent>,
    mut commands: Commands,
    config: ResMut<Config>,
    car_config: ResMut<CarConfig>,
) {
    for spawn_event in events.iter() {
        dbg!(spawn_event);

        let (transform, init_meters) = if let Some(init_meters) = spawn_event.init_meters {
            let (translate, quat) = config.get_transform_by_meter(init_meters);
            let transform = Transform::from_translation(translate).with_rotation(quat);
            (transform, init_meters)
        } else {
            config.get_transform_random()
        };

        spawn_car(
            &mut commands,
            &car_config.car_scene.as_ref().unwrap(),
            &car_config.wheel_scene.as_ref().unwrap(),
            spawn_event.is_hid,
            transform,
            spawn_event.index,
            init_meters,
            car_config.max_torque,
        );
    }
}
