//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzusbdeviceconfiguration?language=objc)
    pub unsafe trait VZUSBDeviceConfiguration: NSObjectProtocol {
        #[method_id(@__retain_semantics Other uuid)]
        unsafe fn uuid(&self) -> Retained<NSUUID>;

        #[method(setUuid:)]
        unsafe fn setUuid(&self, uuid: &NSUUID);
    }

    unsafe impl ProtocolType for dyn VZUSBDeviceConfiguration {}
);
