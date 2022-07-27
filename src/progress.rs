use crate::config::Config;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use nalgebra::Point3;
use obj::*;
use parry3d::query::PointQueryWithLocation;
use rapier3d::prelude::{ColliderShape, Polyline, SegmentPointLocation};
use std::fs::File;
use std::io::BufReader;

#[derive(Component)]
pub struct Tracker;

#[derive(Component)]
pub struct CarProgress {
    pub meters: f32,
}

pub fn track_polyline_start_system(
    mut commands: Commands,
    mut config: ResMut<Config>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    println!("meters: {meters:.1} shift: {:.1}", config.meters_shift);

    let collider = Collider::from(ColliderShape::polyline(vertices, None));
    commands
        .spawn()
        // .insert_bundle(pbr)
        .insert(Name::new("Track polyline"))
        .insert(collider)
        .insert(RigidBody::Fixed)
        .insert(Velocity::zero())
        .insert(Sensor)
        .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
            0., 1., 0.,
        )));

    let point_half = 0.25;
    let point_size = point_half * 2.;
    let point_mesh = Mesh::from(shape::Cube { size: point_size });
    commands.spawn().insert(Tracker).insert_bundle(PbrBundle {
        mesh: meshes.add(point_mesh.clone()),
        material: materials.add(Color::rgba(0.9, 0.9, 0.9, 0.9).into()),
        ..default()
    });
}

pub fn progress_system(
    config: Res<Config>,
    mut cars: Query<(&Transform, &mut CarProgress), With<CarProgress>>,
) {
    if let Some(polyline) = &config.polyline {
        for (transform, mut car_progress) in cars.iter_mut() {
            let tr = transform.translation;
            let point: Point3<Real> = Point3::new(tr.x, tr.y, tr.z);
            let point_location = polyline.project_local_point_and_get_location(&point, true);
            let (segment_i, segment_location) = point_location.1;
            let segment = polyline.segment(segment_i);
            match segment_location {
                SegmentPointLocation::OnVertex(_i) => {
                    // println!("vertex_i_{i:?}");
                }
                SegmentPointLocation::OnEdge(uv) => {
                    let m = uv[1] * segment.length();
                    let meters = match segment_i {
                        i if i >= config.segment_i => {
                            m + config.meters[segment_i as usize] - config.meters_shift
                        }
                        _ => {
                            m + config.meters[segment_i as usize] + config.meters_total
                                - config.meters_shift
                        }
                    };
                    if meters - car_progress.meters > 1000. {
                        // println!("car antiprogress {:.1}", car_progress.meters);
                        car_progress.meters = -(config.meters_total - meters);
                    } else {
                        car_progress.meters = meters;
                    }
                }
            }
        }
    }
}
