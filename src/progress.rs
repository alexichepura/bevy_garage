use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use nalgebra::Point3;
use obj::*;
use parry3d::query::PointQueryWithLocation;
use rapier3d::prelude::{ColliderShape, Polyline};
use std::fs::File;
use std::io::BufReader;

use crate::car::HID;
use crate::config::Config;

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
    config.polyline = Some(polyline);

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
pub fn progress_system(config: Res<Config>, q_car: Query<&Transform, With<HID>>) {
    if let Some(polyline) = &config.polyline {
        for transform in q_car.iter() {
            let tr = transform.translation;
            let point: Point3<Real> = Point3::new(tr.x, tr.y, tr.z);
            let progress = polyline.project_local_point_and_get_location(&point, true);
            println!("progress {progress:?}");
        }
    }
}
