use bevy::prelude::{Quat, Vec3};
use bevy_rapier3d::{
    dynamics::TypedJoint,
    prelude::{
        FixedJointBuilder, GenericJointBuilder, MotorModel, PrismaticJointBuilder,
        SpringJointBuilder,
    },
    rapier::prelude::{JointAxesMask, JointAxis},
};

pub fn build_suspension_joint(anchor: Vec3) -> TypedJoint {
    // let mut joint = PrismaticJointBuilder::new(Vec3::Y)
    //     // .local_anchor1(Vec3::ZERO)
    //     .local_anchor1(anchor)
    //     .local_anchor2(Vec3::ZERO)
    //     .build();

    // let mut joint = GenericJointBuilder::new(
    //     JointAxesMask::ANG_X
    //         | JointAxesMask::ANG_Y
    //         | JointAxesMask::ANG_Z
    //         | JointAxesMask::LIN_X
    //         | JointAxesMask::LIN_Z,
    // )
    // .local_axis1(Vec3::Y)
    // .local_axis2(Vec3::Y)
    // .local_anchor1(anchor)
    // // .local_anchor1(Vec3::ZERO)
    // .local_anchor2(Vec3::ZERO)
    // // .set_motor(JointAxis::LinY, 0., 0., 1e6, 1e3)
    // .build();

    // let mut joint = FixedJointBuilder::new()
    //     .local_anchor1(anchor)
    //     .local_anchor2(Vec3::ZERO)
    //     .build();
    // joint.set_contacts_enabled(false);
    // TypedJoint::FixedJoint(joint)
    // let mut joint = GenericJointBuilder::new(
    //     JointAxesMask::ANG_X
    //         | JointAxesMask::ANG_Y
    //         | JointAxesMask::ANG_Z
    //         | JointAxesMask::LIN_X
    //         | JointAxesMask::LIN_Y
    //         | JointAxesMask::LIN_Z,
    // )
    // .local_anchor1(anchor)
    // // .coupled_axes(JointAxesMask::LIN_Y)
    // .motor_position(JointAxis::LinY, 0., 1e8, 10000.)
    // // .motor_model(JointAxis::LinY, MotorModel::ForceBased)
    // .build();
    // // joint.set_contacts_enabled(false);
    // TypedJoint::GenericJoint(joint)

    // let joint = SpringJointBuilder::new(0.05, 1e5, 1e7)
    //     .local_anchor1(anchor)
    //     .local_anchor2(Vec3::ZERO)
    //     .contacts_enabled(false)
    //     .build();
    // TypedJoint::SpringJoint(joint)

    let mut joint = GenericJointBuilder::new(
        // JointAxesMask::empty(),
        // JointAxesMask::LOCKED_FIXED_AXES,
        JointAxesMask::ANG_X
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z
            // | JointAxesMask::LIN_X
            | JointAxesMask::LIN_Y
            | JointAxesMask::LIN_Z,
    )
    // .coupled_axes(JointAxesMask::LIN_Y)
    // .coupled_axes(JointAxesMask::LIN_AXES)
    .motor_position(JointAxis::LinX, -0.05, 1., 0.1)
    // .motor_model(JointAxis::LinY, MotorModel::ForceBased)
    .local_axis1(Vec3::Y)
    .local_axis2(Vec3::Y)
    .local_anchor1(anchor)
    .local_anchor2(Vec3::ZERO)
    .build();
    joint.set_contacts_enabled(false);
    TypedJoint::GenericJoint(joint)
}

pub fn build_wheel_joint(is_left: bool) -> TypedJoint {
    // let mut joint = FixedJointBuilder::new()
    //     .local_anchor1(Vec3::ZERO)
    //     .local_anchor2(Vec3::ZERO)
    //     .build();
    // joint.set_contacts_enabled(false);
    // TypedJoint::FixedJoint(joint)

    let mut joint = GenericJointBuilder::new(
        JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z
            | JointAxesMask::LIN_X
            | JointAxesMask::LIN_Y
            | JointAxesMask::LIN_Z,
    )
    .local_axis1(Vec3::Y)
    .local_axis2(match is_left {
        true => -Vec3::Y,
        false => Vec3::Y,
    })
    .local_basis1(Quat::from_axis_angle(Vec3::Y, 0.)) // hackfix, prevents jumping on collider edges
    // .local_anchor1(anchor)
    .local_anchor1(Vec3::ZERO)
    .local_anchor2(Vec3::ZERO)
    .build();
    joint.set_contacts_enabled(false);
    TypedJoint::GenericJoint(joint)
}
