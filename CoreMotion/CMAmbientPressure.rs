//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coremotion/cmambientpressuredata?language=objc)
    #[unsafe(super(CMLogItem, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CMLogItem")]
    pub struct CMAmbientPressureData;
);

#[cfg(feature = "CMLogItem")]
unsafe impl NSCoding for CMAmbientPressureData {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSCopying for CMAmbientPressureData {}

#[cfg(feature = "CMLogItem")]
unsafe impl CopyingHelper for CMAmbientPressureData {
    type Result = Self;
}

#[cfg(feature = "CMLogItem")]
unsafe impl NSObjectProtocol for CMAmbientPressureData {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSSecureCoding for CMAmbientPressureData {}

extern_methods!(
    #[cfg(feature = "CMLogItem")]
    unsafe impl CMAmbientPressureData {
        /// Discussion:
        /// The pressure as measured by the pressure sensor.
        /// Pressure is in kPa (kilopascals).
        #[method_id(@__retain_semantics Other pressure)]
        pub unsafe fn pressure(&self) -> Retained<NSMeasurement<NSUnitPressure>>;

        /// Discussion:
        /// The temperature as measured by the pressure sensor.
        /// Temperature is in C (degrees centrigrade).
        #[method_id(@__retain_semantics Other temperature)]
        pub unsafe fn temperature(&self) -> Retained<NSMeasurement<NSUnitTemperature>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CMLogItem")]
    unsafe impl CMAmbientPressureData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
