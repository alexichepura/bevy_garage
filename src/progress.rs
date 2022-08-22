use crate::{car::*, config::*, track::*};
use bevy::prelude::*;
use bevy_rapier3d::{na::Point3, prelude::*, rapier::prelude::ColliderShape};
use obj::*;
use parry3d::query::PointQueryWithLocation;
use parry3d::shape::{Polyline, SegmentPointLocation};
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;

pub fn track_polyline_start_system(mut commands: Commands, mut config: ResMut<Config>) {
    let obj_path = "assets/track-polyline.obj";
    let polyline_buf = BufReader::new(File::open(obj_path).unwrap());
    let model = raw::parse_obj(polyline_buf).unwrap();
    let vertices: Vec<Point3<Real>> = model
        .positions
        .iter()
        .map(|pos| Point3::new(pos.0, pos.1, pos.2))
        .collect();

    let polyline = Polyline::new(vertices.clone(), None);
    let initial_point = Point3::from(config.translation);
    let point_location = polyline.project_local_point_and_get_location(&initial_point, true);
    let (segment_i, segment_location) = point_location.1;
    let segment = polyline.segment(segment_i);
    config.polyline = Some(polyline.clone());
    config.segment_i = segment_i;

    match segment_location {
        SegmentPointLocation::OnVertex(_i) => {
            config.segment_m = 0.;
        }
        SegmentPointLocation::OnEdge(uv) => {
            config.segment_m = uv[1] * segment.length();
        }
    }

    let mut meters = 0.;
    for s in polyline.segments() {
        config.meters.push(meters);
        meters += s.length();
    }
    config.meters_shift = config.meters[config.segment_i as usize];
    config.meters_total = meters;

    println!(
        "track length: {meters:.1} polyline shift: {:.1}",
        config.meters_shift
    );

    let collider = Collider::from(ColliderShape::polyline(vertices, None));
    commands
        .spawn()
        .insert(Name::new("Track polyline"))
        .insert(collider)
        .insert(RigidBody::Fixed)
        .insert(Sensor)
        .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
        .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
            0., 1., 0.,
        )));
}

pub fn progress_system(config: Res<Config>, mut cars: Query<(&Transform, &mut Car, Entity)>) {
    let polyline = config.polyline.as_ref().unwrap();
    let mut board: Vec<(Entity, f32)> = Vec::new();
    for (tr, mut car_progress, e) in cars.iter_mut() {
        let point: Point3<Real> = Point3::from(tr.translation);
        let point_location = polyline.project_local_point_and_get_location(&point, true);
        let (segment_i, segment_location) = point_location.1;
        let segment = polyline.segment(segment_i);
        match segment_location {
            SegmentPointLocation::OnVertex(_i) => {
                // println!("vertex_i_{i:?}");
            }
            SegmentPointLocation::OnEdge(uv) => {
                let m = uv[1] * segment.length();
                let mut meters = match segment_i {
                    i if i >= config.segment_i => {
                        m + config.meters[segment_i as usize] - config.meters_shift
                    }
                    _ => {
                        m + config.meters[segment_i as usize] + config.meters_total
                            - config.meters_shift
                    }
                };
                if meters - car_progress.meters > 1000. {
                    meters = -(config.meters_total - meters);
                }
                let line_dir = Vec3::from(segment.direction().unwrap());
                car_progress.line_dir = line_dir;
                car_progress.meters = meters;
                board.push((e, meters));
            }
        }
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
