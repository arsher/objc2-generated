//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevicebatterystate?language=objc)
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
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevicebattery?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCDeviceBattery;
);

unsafe impl NSObjectProtocol for GCDeviceBattery {}

extern_methods!(
    unsafe impl GCDeviceBattery {
        #[method(batteryLevel)]
        pub unsafe fn batteryLevel(&self) -> c_float;

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
