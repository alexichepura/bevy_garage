use bevy::prelude::*;
use bevy_garage_car::spawn_car;

#[derive(Debug, Event)]
pub struct SpawnCarOnTrackEvent {
    pub player: bool,
    pub index: usize,
    pub position: Option<f32>,
}

#[derive(Component, Debug)]
pub struct CarTrack {
    pub index: usize,
    pub start_shift: f32,
    pub track_position: f32,
    pub ride_distance: f32,
    pub lap: i32,
    pub line_dir: Vec3,
    pub line_pos: Vec3,
    pub place: usize,
}
impl Default for CarTrack {
    fn default() -> Self {
        Self {
            index: 0,
            start_shift: 0.,
            track_position: 0.,
            ride_distance: 0.,
            place: 0,
            lap: 0,
            line_dir: Vec3::ZERO,
            line_pos: Vec3::ZERO,
        }
    }
}

pub fn spawn_car_on_track(
    cmd: &mut Commands,
    car_gl: &Handle<Scene>,
    wheel_gl: &Handle<Scene>,
    player: bool,
    transform: Transform,
    index: usize,
    start_shift: f32,
) -> Entity {
    let car_id = spawn_car(cmd, car_gl, wheel_gl, player, transform);
    cmd.entity(car_id).insert(CarTrack {
        index,
        start_shift,
        ..default()
    });
    car_id
}
