//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Configuration for a USB pointing device that reports absolute coordinates.
    ///
    /// This device can be used by VZVirtualMachineView to send pointer events to the virtual machine.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzusbscreencoordinatepointingdeviceconfiguration?language=objc)
    #[unsafe(super(VZPointingDeviceConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZPointingDeviceConfiguration")]
    pub struct VZUSBScreenCoordinatePointingDeviceConfiguration;
);

#[cfg(feature = "VZPointingDeviceConfiguration")]
unsafe impl NSCopying for VZUSBScreenCoordinatePointingDeviceConfiguration {}

#[cfg(feature = "VZPointingDeviceConfiguration")]
unsafe impl CopyingHelper for VZUSBScreenCoordinatePointingDeviceConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZPointingDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZUSBScreenCoordinatePointingDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZPointingDeviceConfiguration")]
    unsafe impl VZUSBScreenCoordinatePointingDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZPointingDeviceConfiguration`
    #[cfg(feature = "VZPointingDeviceConfiguration")]
    unsafe impl VZUSBScreenCoordinatePointingDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
