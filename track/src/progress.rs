use crate::car_track::CarTrack;
use crate::{TrackConfig, TRACK_POSITIONS};
use bevy::prelude::*;
use bevy_garage_car::{CarRes, CAR_TRAINING_GROUP, STATIC_GROUP};
use bevy_rapier3d::{na::Point3, prelude::*, rapier::prelude::ColliderShape};
use parry3d::query::PointQueryWithLocation;
use parry3d::shape::{Polyline, SegmentPointLocation};
use std::cmp::Ordering;

pub fn track_polyline_start_system(mut cmd: Commands, mut track_config: ResMut<TrackConfig>) {
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
    track_config.polyline = Some(polyline.clone());
    track_config.start_segment_i = segment_i as usize;

    match segment_location {
        SegmentPointLocation::OnVertex(_i) => {
            track_config.start_segment_shift = 0.;
        }
        SegmentPointLocation::OnEdge(uv) => {
            track_config.start_segment_shift = uv[1] * segment.length();
        }
    }

    let mut track_length = 0.;
    for s in polyline.segments() {
        track_config.segments.push(track_length);
        track_length += s.length();
    }
    let start_shift =
        track_config.segments[track_config.start_segment_i] + track_config.start_segment_shift;
    track_config.start_shift = start_shift;
    track_config.track_length = track_length;

    println!(
        "track length: {track_length:.1}, start_shift: {:.1}, segment_shift: {:.1}, segment_i: {}",
        start_shift, track_config.start_segment_shift, track_config.start_segment_i
    );

    cmd.spawn((
        Name::new("Track polyline"),
        Collider::from(ColliderShape::polyline(vertices, None)),
        RigidBody::Fixed,
        Sensor,
        CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP),
        TransformBundle::from_transform(Transform::from_xyz(0., 1., 0.)),
    ));
}

pub fn progress_system(
    track_config: Res<TrackConfig>,
    mut cars: Query<(&Transform, &mut CarTrack, Entity)>,
    car_res: Res<CarRes>,
    mut gizmos: Gizmos,
) {
    let polyline = track_config.polyline.as_ref().unwrap();
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
            track_config.segments[segment_i as usize] + segment_progress - track_config.start_shift;
        let track_position: f32 = if segments_progress > 0. {
            segments_progress
        } else {
            segments_progress + track_config.track_length
        };

        let mut ride_distance = if track_position >= car.start_shift {
            track_position - car.start_shift
        } else {
            track_config.track_length + track_position - car.start_shift
        };
        let half = track_config.track_length / 2.;
        if ride_distance - car.ride_distance > half {
            // prevent increasing distance by going backward
            ride_distance = ride_distance - track_config.track_length;
        }
        if ride_distance.is_sign_positive() && car.ride_distance.is_sign_negative()
            || ride_distance < half && car.ride_distance > half
        {
            car.lap += 1;
        }
        if ride_distance.is_sign_negative() && car.ride_distance.is_sign_positive()
            || ride_distance > -half && car.ride_distance < -half
        {
            car.lap -= 1;
        }
        car.track_position = track_position;
        car.ride_distance = ride_distance;

        let dir = Vec3::from(segment.direction().unwrap());
        car.line_dir = dir;
        car.line_pos = Vec3::from(segment.a) + dir * segment_progress;
        if car_res.show_rays {
            let h = Vec3::Y * 0.6;
            gizmos.line(
                h + tr.translation,
                h + car.line_pos + Vec3::Y * tr.translation.y,
                Color::rgba(0.5, 0.5, 0.5, 0.5),
            );
        }
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
