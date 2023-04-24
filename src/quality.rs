use crate::{
    material::*,
    track::{AsphaltCell, GroundCell},
};
use bevy::prelude::*;

#[cfg(any(target_arch = "wasm32", target_os = "ios", target_os = "android"))]
const VISIBILITY: f32 = 200.;
#[cfg(not(any(target_arch = "wasm32", target_os = "ios", target_os = "android")))]
const VISIBILITY: f32 = 750.;

const VISIBILITY_COLOR: f32 = VISIBILITY * 0.3;

pub fn far_culling(
    mut commands: Commands,
    handled_materials: Res<MaterialHandle>,
    mut pset: ParamSet<(
        Query<&Transform, With<Camera>>,
        Query<
            (
                &Transform,
                &mut Visibility,
                &ComputedVisibility,
                Entity,
                &mut GroundCell,
            ),
            With<GroundCell>,
        >,
        Query<
            (
                &Transform,
                &mut Visibility,
                &ComputedVisibility,
                Entity,
                &mut AsphaltCell,
            ),
            With<AsphaltCell>,
        >,
    )>,
) {
    let cam_translation = pset.p0().single().translation;

    for (transform, mut cell_visibility, computed_visibility, entity, mut cell) in
        pset.p1().iter_mut()
    {
        let distance = (cam_translation - transform.translation).length();
        if distance > VISIBILITY_COLOR {
            if !cell.is_color {
                commands.entity(entity).remove::<HandleGround>();
                commands
                    .entity(entity)
                    .insert(handled_materials.ground_color.clone());
                cell.is_color = true;
            }
        } else {
            if cell.is_color {
                commands.entity(entity).remove::<HandleStandard>();
                commands
                    .entity(entity)
                    .insert(handled_materials.ground.clone());
                cell.is_color = false;
            }
        }
        if distance > VISIBILITY {
            if computed_visibility.is_visible_in_view() {
                *cell_visibility = Visibility::Hidden;
            }
        } else {
            *cell_visibility = Visibility::Inherited;
        }
    }
    for (transform, mut cell_visibility, computed_visibility, entity, mut cell) in
        pset.p2().iter_mut()
    {
        let distance = (cam_translation - transform.translation).length();
        if distance > VISIBILITY_COLOR {
            if !cell.is_color {
                commands.entity(entity).remove::<HandleAsphalt>();
                commands
                    .entity(entity)
                    .insert(handled_materials.asphalt_color.clone());
                cell.is_color = true;
            }
        } else {
            if cell.is_color {
                commands.entity(entity).remove::<HandleStandard>();
                commands
                    .entity(entity)
                    .insert(handled_materials.asphalt.clone());
                cell.is_color = false;
            }
        }
        if distance > VISIBILITY {
            if computed_visibility.is_visible_in_view() {
                *cell_visibility = Visibility::Hidden;
            }
        } else {
            *cell_visibility = Visibility::Inherited;
        }
    }
}
