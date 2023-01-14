#![allow(non_snake_case)]
use icrate::objc2::{
    encode::{Encode, Encoding},
    extern_class, extern_methods, msg_send_id,
    rc::{Id, Owned},
    ClassType,
};
use icrate::Foundation::{CGFloat, NSObject};

// #[link(name = "AppKit", kind = "framework")]
// extern "C" {}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMMotionManager;

    unsafe impl ClassType for CMMotionManager {
        type Super = NSObject;
    }
);

unsafe impl Send for CMMotionManager {}
unsafe impl Sync for CMMotionManager {}

impl CMMotionManager {
    pub fn new() -> Id<Self, Owned> {
        let _class = Self::class();
        let m = _class.instance_methods();
        for mr in m.iter() {
            dbg!(mr.name());
        }
        let id: Id<Self, Owned> = unsafe { msg_send_id![_class, new] };
        id
    }
}

use std::os::raw::c_double;
type Double = c_double;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CMRotationRate {
    pub x: Double,
    pub y: Double,
    pub z: Double,
}

unsafe impl Encode for CMRotationRate {
    const ENCODING: Encoding = Encoding::Struct(
        "CMRotationRate",
        // &[Double::ENCODING, Double::ENCODING, Double::ENCODING],
        &[Encoding::Double, Encoding::Double, Encoding::Double],
    );
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CMAttitude {
    pub roll: Double,
    pub pitch: Double,
    pub yaw: Double,
}

unsafe impl Encode for CMAttitude {
    const ENCODING: Encoding = Encoding::Struct(
        "CMAttitude",
        // &[Double::ENCODING, Double::ENCODING, Double::ENCODING],
        &[Encoding::Double, Encoding::Double, Encoding::Double],
    );
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CMGyroData {
    pub rotationRate: CMRotationRate,
}
unsafe impl Encode for CMGyroData {
    const ENCODING: Encoding = Encoding::Struct("CMGyroData", &[CMRotationRate::ENCODING]);
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CMDeviceMotion {
    pub rotationRate: CMRotationRate,
    pub attitude: CMAttitude,
}
unsafe impl Encode for CMDeviceMotion {
    const ENCODING: Encoding = Encoding::Struct(
        "CMDeviceMotion",
        &[CMRotationRate::ENCODING, CMAttitude::ENCODING],
    );
}

extern_methods!(
    unsafe impl CMMotionManager {
        #[method(startAccelerometerUpdates)]
        pub fn startAccelerometerUpdates(&self);
        #[method(startMagnetometerUpdates)]
        pub fn startMagnetometerUpdates(&self);

        #[method(isGyroAvailable)]
        pub fn isGyroAvailable(&self) -> bool;
        #[method(isGyroActive)]
        pub fn isGyroActive(&self) -> bool;
        #[method(startGyroUpdates)]
        pub fn startGyroUpdates(&self);
        #[method(gyroData)]
        pub fn gyroData(&self) -> CMGyroData;

        #[method(showsDeviceMovementDisplay)]
        pub fn showsDeviceMovementDisplay(&self) -> bool;
        #[method(setShowsDeviceMovementDisplay:)]
        pub(crate) fn setShowsDeviceMovementDisplay(&self, flag: bool);

        #[method(deviceMotionUpdateInterval)]
        pub fn deviceMotionUpdateInterval(&self) -> CGFloat;
        #[method(isDeviceMotionAvailable)]
        pub fn isDeviceMotionAvailable(&self) -> bool;
        #[method(isDeviceMotionActive)]
        pub fn isDeviceMotionActive(&self) -> bool;
        #[method(startDeviceMotionUpdates)]
        pub fn startDeviceMotionUpdates(&self);
        #[method(deviceMotion)]
        pub fn deviceMotion(&self) -> CMDeviceMotion;
    }
);
