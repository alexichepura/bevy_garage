use crate::{
    AsphaltCell, ExtendedMaterialAsphalt, ExtendedMaterialGround, GroundCell, MaterialHandle,
};
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::*;

#[cfg(any(target_os = "ios", target_os = "android"))]
const VISIBILITY: f32 = 200.;
#[cfg(any(target_arch = "wasm32"))]
const VISIBILITY: f32 = 400.;
#[cfg(not(any(target_arch = "wasm32", target_os = "ios", target_os = "android")))]
const VISIBILITY: f32 = 750.;

const VISIBILITY_COLOR: f32 = VISIBILITY * 0.4;

pub fn far_culling(
    mut cmd: Commands,
    handled_materials: Res<MaterialHandle>,
    mut pset: ParamSet<(
        Query<&Transform, With<Camera>>,
        Query<
            (
                &Transform,
                &mut Visibility,
                &InheritedVisibility,
                Entity,
                &mut GroundCell,
            ),
            With<GroundCell>,
        >,
        Query<
            (
                &Transform,
                &mut Visibility,
                &InheritedVisibility,
                Entity,
                &mut AsphaltCell,
            ),
            With<AsphaltCell>,
        >,
    )>,
) {
    let cam_translation = if let Ok(cam_transform) = pset.p0().single() {
        cam_transform.translation
    } else {
        return;
    };

    for (transform, mut cell_visibility, inherited_visibility, entity, mut cell) in
        pset.p1().iter_mut()
    {
        let distance = (cam_translation - transform.translation).length();
        if distance > VISIBILITY_COLOR {
            if !cell.is_color {
                cmd.entity(entity)
                    .remove::<MeshMaterial3d<ExtendedMaterialGround>>();
                cmd.entity(entity)
                    .insert(MeshMaterial3d(handled_materials.ground_color.clone()));
                cell.is_color = true;
            }
        } else {
            if cell.is_color {
                cmd.entity(entity)
                    .remove::<MeshMaterial3d<StandardMaterial>>();
                cmd.entity(entity)
                    .insert(MeshMaterial3d(handled_materials.ground.clone()));
                cell.is_color = false;
            }
        }
        if distance > VISIBILITY {
            if inherited_visibility.get() {
                *cell_visibility = Visibility::Hidden;
            }
        } else {
            *cell_visibility = Visibility::Inherited;
        }
    }
    for (transform, mut cell_visibility, inherited_visibility, entity, mut cell) in
        pset.p2().iter_mut()
    {
        let distance = (cam_translation - transform.translation).length();
        if distance > VISIBILITY_COLOR {
            if !cell.is_color {
                cmd.entity(entity)
                    .remove::<MeshMaterial3d<ExtendedMaterialAsphalt>>();
                cmd.entity(entity)
                    .insert(MeshMaterial3d(handled_materials.asphalt_color.clone()));
                cell.is_color = true;
            }
        } else {
            if cell.is_color {
                cmd.entity(entity)
                    .remove::<MeshMaterial3d<StandardMaterial>>();
                cmd.entity(entity)
                    .insert(MeshMaterial3d(handled_materials.asphalt.clone()));
                cell.is_color = false;
            }
        }
        if distance > VISIBILITY {
            if inherited_visibility.get() {
                *cell_visibility = Visibility::Hidden;
            }
        } else {
            *cell_visibility = Visibility::Inherited;
        }
    }
}
