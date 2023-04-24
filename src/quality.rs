use crate::track::{AsphaltCell, GroundCell};
use bevy::prelude::*;

#[cfg(any(target_arch = "wasm32", target_os = "ios", target_os = "android"))]
const VISIBILITY: f32 = 200.;
#[cfg(not(any(target_arch = "wasm32", target_os = "ios", target_os = "android")))]
const VISIBILITY: f32 = 750.;

pub fn far_culling(
    mut pset: ParamSet<(
        Query<&Transform, With<Camera>>,
        Query<(&Transform, &mut Visibility, &ComputedVisibility), With<GroundCell>>,
        Query<(&Transform, &mut Visibility, &ComputedVisibility), With<AsphaltCell>>,
    )>,
) {
    let cam_translation = pset.p0().single().translation;

    for (ground_cell, mut cell_visibility, computed_visibility) in pset.p1().iter_mut() {
        if (cam_translation - ground_cell.translation).length() > VISIBILITY {
            if computed_visibility.is_visible_in_view() {
                *cell_visibility = Visibility::Hidden;
            }
        } else {
            *cell_visibility = Visibility::Inherited;
        }
    }
    for (asphalt_cell, mut cell_visibility, computed_visibility) in pset.p2().iter_mut() {
        if (cam_translation - asphalt_cell.translation).length() > VISIBILITY {
            if computed_visibility.is_visible_in_view() {
                *cell_visibility = Visibility::Hidden;
            }
        } else {
            *cell_visibility = Visibility::Inherited;
        }
    }
}
