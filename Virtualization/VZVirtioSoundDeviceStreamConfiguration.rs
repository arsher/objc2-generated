//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtiosounddevicestreamconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZVirtioSoundDeviceStreamConfiguration;
);

unsafe impl NSCopying for VZVirtioSoundDeviceStreamConfiguration {}

unsafe impl CopyingHelper for VZVirtioSoundDeviceStreamConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZVirtioSoundDeviceStreamConfiguration {}

extern_methods!(
    unsafe impl VZVirtioSoundDeviceStreamConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
