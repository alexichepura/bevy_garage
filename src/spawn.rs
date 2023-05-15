use bevy::prelude::*;
use bevy_garage_car::config::CarConfig;
use bevy_garage_track::{
    car_track::{spawn_car_on_track, SpawnCarOnTrackEvent},
    TrackConfig,
};

pub fn spawn_car_start_system(mut car_spawn_events: EventWriter<SpawnCarOnTrackEvent>) {
    car_spawn_events.send(SpawnCarOnTrackEvent {
        is_hid: true,
        index: 0,
        init_meters: Some(0.),
    });
}

pub fn spawn_car_system(
    mut events: EventReader<SpawnCarOnTrackEvent>,
    mut commands: Commands,
    track_config: ResMut<TrackConfig>,
    car_config: ResMut<CarConfig>,
) {
    for spawn_event in events.iter() {
        dbg!(spawn_event);

        let (transform, init_meters) = if let Some(init_meters) = spawn_event.init_meters {
            let (translate, quat) = track_config.get_transform_by_meter(init_meters);
            let transform = Transform::from_translation(translate).with_rotation(quat);
            (transform, init_meters)
        } else {
            track_config.get_transform_random()
        };

        spawn_car_on_track(
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