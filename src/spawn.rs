use bevy::prelude::*;
use bevy_garage_car::CarRes;
use bevy_garage_track::{spawn_car_on_track, SpawnCarOnTrackEvent, TrackConfig};

pub fn spawn_car_start_system(mut car_spawn_events: MessageWriter<SpawnCarOnTrackEvent>) {
    car_spawn_events.write(SpawnCarOnTrackEvent {
        player: true,
        index: 0,
        position: Some(0.),
    });
}

pub fn spawn_car_system(
    mut events: MessageReader<SpawnCarOnTrackEvent>,
    mut cmd: Commands,
    track_config: ResMut<TrackConfig>,
    car_res: ResMut<CarRes>,
) {
    for spawn_event in events.read() {
        dbg!(spawn_event);

        let (transform, init_meters) = if let Some(init_meters) = spawn_event.position {
            let (translate, quat) = track_config.get_transform_by_meter(init_meters);
            let transform = Transform::from_translation(translate).with_rotation(quat);
            (transform, init_meters)
        } else {
            track_config.get_transform_random()
        };

    // Scenes are currently disabled for Bevy 0.17 compatibility
    // Pass None/Option instead of unwrapping
    spawn_car_on_track(
        &mut cmd,
        &None, // car_scene temporarily disabled
        &None, // wheel_scene temporarily disabled
        spawn_event.player,
        transform,
        spawn_event.index,
        init_meters,
    );
    }
}
