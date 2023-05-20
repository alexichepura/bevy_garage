use bevy::prelude::{Quat, Vec3};
use bevy_rapier3d::{
    prelude::{GenericJoint, GenericJointBuilder},
    rapier::prelude::{JointAxesMask, JointAxis},
};

pub fn build_joint(anchor: Vec3, is_left: bool) -> GenericJoint {
    let joint = GenericJointBuilder::new(
        JointAxesMask::ANG_Y | JointAxesMask::ANG_Z | JointAxesMask::X | JointAxesMask::Z,
    )
    .local_axis1(Vec3::X)
    .local_axis2(match is_left {
        true => -Vec3::Y,
        false => Vec3::Y,
    })
    .local_basis1(Quat::from_axis_angle(Vec3::Y, 0.)) // hackfix, prevents jumping on collider edges
    .local_anchor1(anchor)
    .local_anchor2(Vec3::ZERO)
    .set_motor(JointAxis::Y, 0., 0., 1e6, 1e3)
    .build();
    joint
}
