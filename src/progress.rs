use crate::car::HID;
use crate::config::Config;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use nalgebra::Point3;
use obj::*;
use parry3d::query::PointQueryWithLocation;
use rapier3d::prelude::{ColliderShape, Polyline};
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
            let result = polyline.project_local_point_and_get_location(&point, true);
            progress = result.0.point;
            println!("result {:?}", result.1);
        }

        for mut transform in set.p1().iter_mut() {
            transform.translation.x = progress.x;
            transform.translation.y = 1.;
            //   transform.translation.y = progress.point.y;
            transform.translation.z = progress.z;
        }
    }
}
