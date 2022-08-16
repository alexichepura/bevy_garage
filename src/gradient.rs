// use bevy::{prelude::*, render::mesh::VertexAttributeValues};
// TODO gradient visualisation mesh
pub fn gradient_vis_start_system(// mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let mut colorful_cube = Mesh::from(shape::Cube { size: 5.0 });
    // // let mut colorful_cube = Mesh::from(shape::RegularPolygon::new(10., 6));
    // if let Some(VertexAttributeValues::Float32x3(positions)) =
    //     colorful_cube.attribute(Mesh::ATTRIBUTE_POSITION)
    // {
    //     // dbg!(positions);
    //     let colors: Vec<[f32; 4]> = positions
    //         .iter()
    //         .map(|[r, g, b]| [(1. - *r) / 2., (1. - *g) / 2., (1. - *b) / 2., 1.])
    //         .collect();
    //     colorful_cube.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    // }
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(colorful_cube),
    //     // This is the default color, but note that vertex colors are
    //     // multiplied by the base color, so you'll likely want this to be
    //     // white if using vertex colors.
    //     material: materials.add(Color::rgb(1., 1., 1.).into()),
    //     transform: Transform::from_xyz(-10.0, 10., -10.0),
    //     ..default()
    // });
}
