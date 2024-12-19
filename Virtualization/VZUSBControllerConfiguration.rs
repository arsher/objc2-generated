//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzusbcontrollerconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZUSBControllerConfiguration;
);

unsafe impl NSCopying for VZUSBControllerConfiguration {}

unsafe impl CopyingHelper for VZUSBControllerConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZUSBControllerConfiguration {}

extern_methods!(
    unsafe impl VZUSBControllerConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "VZUSBDeviceConfiguration")]
        #[method_id(@__retain_semantics Other usbDevices)]
        pub unsafe fn usbDevices(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn VZUSBDeviceConfiguration>>>;

        #[cfg(feature = "VZUSBDeviceConfiguration")]
        #[method(setUsbDevices:)]
        pub unsafe fn setUsbDevices(
            &self,
            usb_devices: &NSArray<ProtocolObject<dyn VZUSBDeviceConfiguration>>,
        );
    }
);
