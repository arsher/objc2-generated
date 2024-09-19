//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(feature = "CMDeviceMotion", feature = "CMLogItem", feature = "block2"))]
pub type CMHeadphoneDeviceMotionHandler =
    *mut block2::Block<dyn Fn(*mut CMDeviceMotion, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMHeadphoneMotionManager;

    unsafe impl ClassType for CMHeadphoneMotionManager {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for CMHeadphoneMotionManager {}

extern_methods!(
    unsafe impl CMHeadphoneMotionManager {
        #[cfg(feature = "CMAuthorization")]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> CMAuthorizationStatus;

        #[cfg(feature = "CMHeadphoneMotionManagerDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn CMHeadphoneMotionManagerDelegate>>>;

        #[cfg(feature = "CMHeadphoneMotionManagerDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CMHeadphoneMotionManagerDelegate>>,
        );

        #[method(isConnectionStatusActive)]
        pub unsafe fn isConnectionStatusActive(&self) -> bool;

        #[method(isDeviceMotionAvailable)]
        pub unsafe fn isDeviceMotionAvailable(&self) -> bool;

        #[method(isDeviceMotionActive)]
        pub unsafe fn isDeviceMotionActive(&self) -> bool;

        #[cfg(all(feature = "CMDeviceMotion", feature = "CMLogItem"))]
        #[method_id(@__retain_semantics Other deviceMotion)]
        pub unsafe fn deviceMotion(&self) -> Option<Retained<CMDeviceMotion>>;

        #[method(startDeviceMotionUpdates)]
        pub unsafe fn startDeviceMotionUpdates(&self);

        #[cfg(all(feature = "CMDeviceMotion", feature = "CMLogItem", feature = "block2"))]
        #[method(startDeviceMotionUpdatesToQueue:withHandler:)]
        pub unsafe fn startDeviceMotionUpdatesToQueue_withHandler(
            &self,
            queue: &NSOperationQueue,
            handler: CMHeadphoneDeviceMotionHandler,
        );

        #[method(stopDeviceMotionUpdates)]
        pub unsafe fn stopDeviceMotionUpdates(&self);

        #[method(startConnectionStatusUpdates)]
        pub unsafe fn startConnectionStatusUpdates(&self);

        #[method(stopConnectionStatusUpdates)]
        pub unsafe fn stopConnectionStatusUpdates(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMHeadphoneMotionManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
