//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// This is the battery status and it's represented by one of the following values:
/// GCControllerBatteryStateUnknown means that the current state of battery is unknown or cannot be determined
/// GCControllerBatteryStateDischarging means that controller is on battery and discharging at this moment
/// GCControllerBatteryStateCharging means that controller is plugged in, but it's battery level is less than 100%
/// GCControllerBatteryStateFull means that controller is plugged in and it's battery level is 100%
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevicebatterystate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GCDeviceBatteryState(pub NSInteger);
impl GCDeviceBatteryState {
    #[doc(alias = "GCDeviceBatteryStateUnknown")]
    pub const Unknown: Self = Self(-1);
    #[doc(alias = "GCDeviceBatteryStateDischarging")]
    pub const Discharging: Self = Self(0);
    #[doc(alias = "GCDeviceBatteryStateCharging")]
    pub const Charging: Self = Self(1);
    #[doc(alias = "GCDeviceBatteryStateFull")]
    pub const Full: Self = Self(2);
}

unsafe impl Encode for GCDeviceBatteryState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GCDeviceBatteryState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A controller battery is an abstract representation of the battery level and battery status of a GCController instance.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevicebattery?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCDeviceBattery;
);

unsafe impl NSObjectProtocol for GCDeviceBattery {}

extern_methods!(
    unsafe impl GCDeviceBattery {
        /// This is the battery level for controller.
        /// Battery level ranges from 0.0 (fully discharged) to 1.0 (100% charged) and defaults to 0
        #[method(batteryLevel)]
        pub unsafe fn batteryLevel(&self) -> c_float;

        /// A battery state for controller, defaults to GCControllerBatteryStateUnknown
        ///
        ///
        /// Note: This property might be useful if you display the information about currently connected controller for player's convenience
        #[method(batteryState)]
        pub unsafe fn batteryState(&self) -> GCDeviceBatteryState;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCDeviceBattery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
