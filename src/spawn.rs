use bevy::prelude::*;
use bevy_garage_car::CarRes;
use bevy_garage_track::{spawn_car_on_track, SpawnCarOnTrackEvent, TrackConfig};

pub fn spawn_car_start_system(mut car_spawn_events: EventWriter<SpawnCarOnTrackEvent>) {
    car_spawn_events.send(SpawnCarOnTrackEvent {
        player: true,
        index: 0,
        position: Some(0.),
    });
}

pub fn spawn_car_system(
    mut events: EventReader<SpawnCarOnTrackEvent>,
    mut cmd: Commands,
    track_config: ResMut<TrackConfig>,
    car_res: ResMut<CarRes>,
) {
    for spawn_event in events.iter() {
        dbg!(spawn_event);

        let (transform, init_meters) = if let Some(init_meters) = spawn_event.position {
            let (translate, quat) = track_config.get_transform_by_meter(init_meters);
            let transform = Transform::from_translation(translate).with_rotation(quat);
            (transform, init_meters)
        } else {
            track_config.get_transform_random()
        };

        spawn_car_on_track(
            &mut cmd,
            &car_res.car_scene.as_ref().unwrap(),
            &car_res.wheel_scene.as_ref().unwrap(),
            spawn_event.player,
            transform,
            spawn_event.index,
            init_meters,
        );
    }
}
