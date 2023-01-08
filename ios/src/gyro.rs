#![allow(non_snake_case)]
use objc2::{
    encode::{Encode, Encoding},
    extern_class, extern_methods,
    foundation::{CGFloat, NSObject},
    msg_send_id,
    rc::{Id, Shared},
    ClassType,
};
// https://github.com/rust-windowing/winit/blob/master/src/platform_impl/ios/uikit/view.rs

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
    pub fn new() -> Id<Self, Shared> {
        let _class = Self::class();
        let m = _class.instance_methods();
        for mr in m.iter() {
            dbg!(mr.name());
        }
        let id: Id<Self, Shared> = unsafe { msg_send_id![_class, new] };
        id
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CMRotationRate {
    pub x: CGFloat,
    pub y: CGFloat,
    pub z: CGFloat,
}

unsafe impl Encode for CMRotationRate {
    const ENCODING: Encoding = Encoding::Struct(
        "CMRotationRate",
        &[CGFloat::ENCODING, CGFloat::ENCODING, CGFloat::ENCODING],
    );
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CMAttitude {
    pub roll: CGFloat,
    pub pitch: CGFloat,
    pub yaw: CGFloat,
}

unsafe impl Encode for CMAttitude {
    const ENCODING: Encoding = Encoding::Struct(
        "CMAttitude",
        &[CGFloat::ENCODING, CGFloat::ENCODING, CGFloat::ENCODING],
    );
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CMGyroData {
    pub rotationRate: CMRotationRate,
}
unsafe impl Encode for CMGyroData {
    const ENCODING: Encoding = Encoding::Struct("CMGyroData", &[CGFloat::ENCODING]);
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CMDeviceMotion {
    pub rotationRate: CMRotationRate,
    pub attitude: CMAttitude,
}
unsafe impl Encode for CMDeviceMotion {
    const ENCODING: Encoding =
        Encoding::Struct("CMDeviceMotion", &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

extern_methods!(
    unsafe impl CMMotionManager {
        #[sel(isGyroAvailable)]
        pub fn isGyroAvailable(&self) -> bool;
        #[sel(isGyroActive)]
        pub fn isGyroActive(&self) -> bool;
        #[sel(startGyroUpdates)]
        pub fn startGyroUpdates(&self);
        #[sel(gyroData)]
        pub fn gyroData(&self) -> CMGyroData;

        #[sel(isDeviceMotionAvailable)]
        pub fn isDeviceMotionAvailable(&self) -> bool;
        #[sel(isDeviceMotionActive)]
        pub fn isDeviceMotionActive(&self) -> bool;
        #[sel(startDeviceMotionUpdates)]
        pub fn startDeviceMotionUpdates(&self);
        #[sel(deviceMotion)]
        pub fn deviceMotion(&self) -> CMDeviceMotion;
    }
);
