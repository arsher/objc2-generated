//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem", feature = "block2"))]
pub type CMAccelerometerHandler =
    *mut block2::Block<dyn Fn(*mut CMAccelerometerData, *mut NSError)>;

#[cfg(all(feature = "CMGyro", feature = "CMLogItem", feature = "block2"))]
pub type CMGyroHandler = *mut block2::Block<dyn Fn(*mut CMGyroData, *mut NSError)>;

#[cfg(all(feature = "CMDeviceMotion", feature = "CMLogItem", feature = "block2"))]
pub type CMDeviceMotionHandler = *mut block2::Block<dyn Fn(*mut CMDeviceMotion, *mut NSError)>;

#[cfg(all(feature = "CMLogItem", feature = "CMMagnetometer", feature = "block2"))]
pub type CMMagnetometerHandler = *mut block2::Block<dyn Fn(*mut CMMagnetometerData, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMMotionManager;

    unsafe impl ClassType for CMMotionManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CMMotionManager {}

extern_methods!(
    unsafe impl CMMotionManager {
        #[method(accelerometerUpdateInterval)]
        pub unsafe fn accelerometerUpdateInterval(&self) -> NSTimeInterval;

        #[method(setAccelerometerUpdateInterval:)]
        pub unsafe fn setAccelerometerUpdateInterval(
            &self,
            accelerometer_update_interval: NSTimeInterval,
        );

        #[method(isAccelerometerAvailable)]
        pub unsafe fn isAccelerometerAvailable(&self) -> bool;

        #[method(isAccelerometerActive)]
        pub unsafe fn isAccelerometerActive(&self) -> bool;

        #[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem"))]
        #[method_id(@__retain_semantics Other accelerometerData)]
        pub unsafe fn accelerometerData(&self) -> Option<Retained<CMAccelerometerData>>;

        #[method(startAccelerometerUpdates)]
        pub unsafe fn startAccelerometerUpdates(&self);

        #[cfg(all(feature = "CMAccelerometer", feature = "CMLogItem", feature = "block2"))]
        #[method(startAccelerometerUpdatesToQueue:withHandler:)]
        pub unsafe fn startAccelerometerUpdatesToQueue_withHandler(
            &self,
            queue: &NSOperationQueue,
            handler: CMAccelerometerHandler,
        );

        #[method(stopAccelerometerUpdates)]
        pub unsafe fn stopAccelerometerUpdates(&self);

        #[method(gyroUpdateInterval)]
        pub unsafe fn gyroUpdateInterval(&self) -> NSTimeInterval;

        #[method(setGyroUpdateInterval:)]
        pub unsafe fn setGyroUpdateInterval(&self, gyro_update_interval: NSTimeInterval);

        #[method(isGyroAvailable)]
        pub unsafe fn isGyroAvailable(&self) -> bool;

        #[method(isGyroActive)]
        pub unsafe fn isGyroActive(&self) -> bool;

        #[cfg(all(feature = "CMGyro", feature = "CMLogItem"))]
        #[method_id(@__retain_semantics Other gyroData)]
        pub unsafe fn gyroData(&self) -> Option<Retained<CMGyroData>>;

        #[method(startGyroUpdates)]
        pub unsafe fn startGyroUpdates(&self);

        #[cfg(all(feature = "CMGyro", feature = "CMLogItem", feature = "block2"))]
        #[method(startGyroUpdatesToQueue:withHandler:)]
        pub unsafe fn startGyroUpdatesToQueue_withHandler(
            &self,
            queue: &NSOperationQueue,
            handler: CMGyroHandler,
        );

        #[method(stopGyroUpdates)]
        pub unsafe fn stopGyroUpdates(&self);

        #[method(magnetometerUpdateInterval)]
        pub unsafe fn magnetometerUpdateInterval(&self) -> NSTimeInterval;

        #[method(setMagnetometerUpdateInterval:)]
        pub unsafe fn setMagnetometerUpdateInterval(
            &self,
            magnetometer_update_interval: NSTimeInterval,
        );

        #[method(isMagnetometerAvailable)]
        pub unsafe fn isMagnetometerAvailable(&self) -> bool;

        #[method(isMagnetometerActive)]
        pub unsafe fn isMagnetometerActive(&self) -> bool;

        #[cfg(all(feature = "CMLogItem", feature = "CMMagnetometer"))]
        #[method_id(@__retain_semantics Other magnetometerData)]
        pub unsafe fn magnetometerData(&self) -> Option<Retained<CMMagnetometerData>>;

        #[method(startMagnetometerUpdates)]
        pub unsafe fn startMagnetometerUpdates(&self);

        #[cfg(all(feature = "CMLogItem", feature = "CMMagnetometer", feature = "block2"))]
        #[method(startMagnetometerUpdatesToQueue:withHandler:)]
        pub unsafe fn startMagnetometerUpdatesToQueue_withHandler(
            &self,
            queue: &NSOperationQueue,
            handler: CMMagnetometerHandler,
        );

        #[method(stopMagnetometerUpdates)]
        pub unsafe fn stopMagnetometerUpdates(&self);

        #[method(deviceMotionUpdateInterval)]
        pub unsafe fn deviceMotionUpdateInterval(&self) -> NSTimeInterval;

        #[method(setDeviceMotionUpdateInterval:)]
        pub unsafe fn setDeviceMotionUpdateInterval(
            &self,
            device_motion_update_interval: NSTimeInterval,
        );

        #[cfg(feature = "CMAttitude")]
        #[method(availableAttitudeReferenceFrames)]
        pub unsafe fn availableAttitudeReferenceFrames() -> CMAttitudeReferenceFrame;

        #[cfg(feature = "CMAttitude")]
        #[method(attitudeReferenceFrame)]
        pub unsafe fn attitudeReferenceFrame(&self) -> CMAttitudeReferenceFrame;

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
            handler: CMDeviceMotionHandler,
        );

        #[cfg(feature = "CMAttitude")]
        #[method(startDeviceMotionUpdatesUsingReferenceFrame:)]
        pub unsafe fn startDeviceMotionUpdatesUsingReferenceFrame(
            &self,
            reference_frame: CMAttitudeReferenceFrame,
        );

        #[cfg(all(
            feature = "CMAttitude",
            feature = "CMDeviceMotion",
            feature = "CMLogItem",
            feature = "block2"
        ))]
        #[method(startDeviceMotionUpdatesUsingReferenceFrame:toQueue:withHandler:)]
        pub unsafe fn startDeviceMotionUpdatesUsingReferenceFrame_toQueue_withHandler(
            &self,
            reference_frame: CMAttitudeReferenceFrame,
            queue: &NSOperationQueue,
            handler: CMDeviceMotionHandler,
        );

        #[method(stopDeviceMotionUpdates)]
        pub unsafe fn stopDeviceMotionUpdates(&self);

        #[method(showsDeviceMovementDisplay)]
        pub unsafe fn showsDeviceMovementDisplay(&self) -> bool;

        #[method(setShowsDeviceMovementDisplay:)]
        pub unsafe fn setShowsDeviceMovementDisplay(&self, shows_device_movement_display: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMMotionManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
