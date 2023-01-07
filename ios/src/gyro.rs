#![allow(non_snake_case)]
use objc2::foundation::NSObject;
use objc2::rc::{Id, Owned};
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, sel, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct CMMotionManager;

    unsafe impl ClassType for CMMotionManager {
        type Super = NSObject;
    }
);

impl CMMotionManager {
    pub fn new() -> Id<Self, Owned> {
        let _class = Self::class();
        dbg!(_class.responds_to(sel!(gyroAvailable)));
        dbg!(_class.name());
        unsafe { msg_send_id![_class, new] }
    }
    pub fn msg_send_gyroAvailable(&self) -> bool {
        let is_gyro: bool = unsafe { msg_send![self, gyroAvailable] };
        is_gyro
    }
}

extern_methods!(
    unsafe impl CMMotionManager {
        #[sel(gyroAvailable)]
        pub fn gyroAvailable(&self) -> bool;
    }
);
