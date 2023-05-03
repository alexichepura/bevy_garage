use crate::{car::*, config::*, track::*};
use bevy::prelude::*;
use bevy_rapier3d::{na::Point3, prelude::*, rapier::prelude::ColliderShape};
use parry3d::query::PointQueryWithLocation;
use parry3d::shape::{Polyline, SegmentPointLocation};
use std::cmp::Ordering;

pub fn track_polyline_start_system(mut commands: Commands, mut config: ResMut<Config>) {
    let positions = TRACK_POSITIONS;

    let vertices: Vec<Point3<Real>> = positions
        .iter()
        .map(|pos| Point3::new(pos.0, pos.1, pos.2))
        .collect();

    let polyline = Polyline::new(vertices.clone(), None);
    let initial_point = Point3::from(Vec3::ZERO);
    let point_location = polyline.project_local_point_and_get_location(&initial_point, true);
    let (segment_i, segment_location) = point_location.1;
    let segment = polyline.segment(segment_i);
    config.polyline = Some(polyline.clone());
    config.start_segment_i = segment_i as usize;

    match segment_location {
        SegmentPointLocation::OnVertex(_i) => {
            config.start_segment_shift = 0.;
        }
        SegmentPointLocation::OnEdge(uv) => {
            config.start_segment_shift = uv[1] * segment.length();
        }
    }

    let mut track_length = 0.;
    for s in polyline.segments() {
        config.segments.push(track_length);
        track_length += s.length();
    }
    let start_shift = config.segments[config.start_segment_i] + config.start_segment_shift;
    config.start_shift = start_shift;
    config.track_length = track_length;

    println!(
        "track length: {track_length:.1}, start_shift: {:.1}, segment_shift: {:.1}, segment_i: {}",
        start_shift, config.start_segment_shift, config.start_segment_i
    );

    let collider = Collider::from(ColliderShape::polyline(vertices, None));
    commands
        .spawn_empty()
        .insert(Name::new("Track polyline"))
        .insert(collider)
        .insert(RigidBody::Fixed)
        .insert(Sensor)
        .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            0., 1., 0.,
        )));
}

pub fn progress_system(config: Res<Config>, mut cars: Query<(&Transform, &mut Car, Entity)>) {
    let polyline = config.polyline.as_ref().unwrap();
    let mut board: Vec<(Entity, f32)> = Vec::new();
    for (tr, mut car, e) in cars.iter_mut() {
        let point: Point3<Real> = Point3::from(tr.translation);
        let point_location = polyline.project_local_point_and_get_location(&point, true);
        let (segment_i, segment_location) = point_location.1;
        let segment = polyline.segment(segment_i);
        let segment_progress = match segment_location {
            SegmentPointLocation::OnEdge(uv) => uv[1] * segment.length(),
            _ => {
                continue;
            }
        };

        let segments_progress: f32 =
            config.segments[segment_i as usize] + segment_progress - config.start_shift;
        let track_position: f32 = if segments_progress > 0. {
            segments_progress
        } else {
            segments_progress + config.track_length
        };

        let mut ride_distance = if track_position >= car.start_shift {
            track_position - car.start_shift
        } else {
            config.track_length + track_position - car.start_shift
        };
        let half = config.track_length / 2.;
        if ride_distance - car.ride_distance > half {
            // prevent increasing distance by going backward
            ride_distance = ride_distance - config.track_length;
        }
        if ride_distance < half && config.track_length - car.ride_distance < half {
            car.lap += 1;
        }
        if ride_distance > -half && config.track_length + car.ride_distance < half {
            car.lap -= 1;
        }
        car.track_position = track_position;
        car.ride_distance = ride_distance;

        let dir = Vec3::from(segment.direction().unwrap());
        car.line_dir = dir;
        car.line_pos = Vec3::from(segment.a) + dir * segment_progress;
        board.push((e, track_position));
    }
    board.sort_by(|a, b| {
        if a.1 > b.1 {
            return Ordering::Greater;
        }
        Ordering::Less
    });
    for (i, (e, _)) in board.iter().enumerate() {
        let (_, mut p, _) = cars.get_mut(*e).unwrap();
        p.place = i;
    }
}
