//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtiotraditionalmemoryballoondeviceconfiguration?language=objc)
    #[unsafe(super(VZMemoryBalloonDeviceConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZMemoryBalloonDeviceConfiguration")]
    pub struct VZVirtioTraditionalMemoryBalloonDeviceConfiguration;
);

#[cfg(feature = "VZMemoryBalloonDeviceConfiguration")]
unsafe impl NSCopying for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}

#[cfg(feature = "VZMemoryBalloonDeviceConfiguration")]
unsafe impl CopyingHelper for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZMemoryBalloonDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZMemoryBalloonDeviceConfiguration")]
    unsafe impl VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZMemoryBalloonDeviceConfiguration`
    #[cfg(feature = "VZMemoryBalloonDeviceConfiguration")]
    unsafe impl VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
