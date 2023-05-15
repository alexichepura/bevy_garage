pub mod asphalt;
pub mod car_track;
pub mod config;
pub mod decor;
pub mod ground;
pub mod kerb;
pub mod material;
pub mod mesh;
pub mod progress;
pub mod quality;
pub mod shader;
pub mod track;
pub mod wall;

pub use asphalt::*;
use bevy_garage_car::CarSet;
pub use config::*;
pub use decor::*;
pub use ground::*;
pub use material::*;
pub use progress::*;
pub use quality::*;
pub use shader::*;
pub use track::*;

use bevy::prelude::*;

use self::{
    asphalt::spawn_road, ground::spawn_ground_heightfield, kerb::spawn_kerb, track::Track,
    wall::spawn_walls,
};

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrackConfig::default())
            .add_plugin(ShadersPlugin)
            .add_plugin(MaterialPlugin::<GroundMaterial>::default())
            .add_plugin(MaterialPlugin::<AsphaltMaterial>::default())
            .init_resource::<MaterialHandle>()
            .add_startup_systems((
                track_polyline_start_system,
                track_start_system,
                track_decorations_start_system.after(track_polyline_start_system),
            ))
            .add_systems((far_culling, progress_system.in_set(CarSet::Input)));
    }
}

pub fn track_start_system(
    handled_materials: Res<MaterialHandle>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let track = Track::new();
    let aabb = spawn_road(&handled_materials, &mut commands, &mut meshes, &track);
    spawn_ground_heightfield(&mut commands, &mut meshes, &handled_materials, &aabb, 100.);

    spawn_kerb(&mut commands, &mut meshes, &handled_materials, &track);
    let mut left_wall_points: Vec<Vec3> = vec![];
    let mut right_wall_points: Vec<Vec3> = vec![];
    for (i, p) in track.points.iter().enumerate() {
        left_wall_points.push(*p + track.right_norm[i] * 7.5);
        right_wall_points.push(*p + track.right_norm[i] * -7.5);
    }
    spawn_walls(
        &mut commands,
        &mut meshes,
        &handled_materials,
        &track.indices,
        &left_wall_points,
        &track.right_norm,
    );
    spawn_walls(
        &mut commands,
        &mut meshes,
        &handled_materials,
        &track.indices,
        &right_wall_points,
        &track.right_norm,
    );
}
