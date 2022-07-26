use crate::car::HID;
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
    let track_length: f32 = polyline
        .clone()
        .segments()
        .fold(0., |acc, x| acc + x.length());
    println!("track_length_{track_length:?}");
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
        "s_{:?} sm_{:?} sh_{:?}",
        config.segment_i, config.segment_m, config.meters_shift
    );

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
// pub fn progress_system(config: Res<Config>, q_car: Query<&Transform, With<HID>>) {
//     if let Some(polyline) = &config.polyline {
//         for transform in q_car.iter() {
//             let tr = transform.translation;
//             let point: Point3<Real> = Point3::new(tr.x, tr.y, tr.z);
//             let progress = polyline.project_local_point_and_get_location(&point, true);
//             println!("progress {progress:?}");
//         }
//     }
// }
pub fn progress_system(
    config: Res<Config>,
    mut set: ParamSet<(
        Query<&Transform, With<HID>>,
        Query<&mut Transform, With<Tracker>>,
    )>,
) {
    if let Some(polyline) = &config.polyline {
        let mut progress: Point3<Real> = Point3::new(0., 0., 0.);
        for transform in set.p0().iter() {
            let tr = transform.translation;
            let point: Point3<Real> = Point3::new(tr.x, tr.y, tr.z);
            let point_location = polyline.project_local_point_and_get_location(&point, true);
            progress = point_location.0.point;
            let (segment_i, segment_location) = point_location.1;
            let segment = polyline.segment(segment_i);
            match segment_location {
                SegmentPointLocation::OnVertex(i) => {
                    println!("vertex_i_{i:?}");
                }
                SegmentPointLocation::OnEdge(uv) => {
                    let m = uv[1] * segment.length();
                    // println!("edge_{uv:?} {:?} {:?}", m, m - config.segment_m);
                    // let meters = m + config.meters[segment_i as usize] - config.meters_shift;
                    let meters = match segment_i {
                        i if i >= config.segment_i => {
                            m + config.meters[segment_i as usize] - config.meters_shift
                        }
                        _ => {
                            m + config.meters[segment_i as usize]
                                - (config.meters_total - config.meters_shift)
                        }
                    };
                    println!(
                        "s_{segment_i:?} {:?} {:?} {:?} {:?}",
                        m.round(),
                        config.meters_shift.round(),
                        config.meters[segment_i as usize].round(),
                        meters.round()
                    );
                }
            }

            // println!("segment {segment:?} location {location:?}");
        }

        for mut transform in set.p1().iter_mut() {
            transform.translation.x = progress.x;
            transform.translation.y = 1.;
            //   transform.translation.y = progress.point.y;
            transform.translation.z = progress.z;
        }
    }
}
