//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceoutputstreamconfiguration?language=objc)
    #[unsafe(super(VZVirtioSoundDeviceStreamConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZVirtioSoundDeviceStreamConfiguration")]
    pub struct VZVirtioSoundDeviceOutputStreamConfiguration;
);

#[cfg(feature = "VZVirtioSoundDeviceStreamConfiguration")]
unsafe impl NSCopying for VZVirtioSoundDeviceOutputStreamConfiguration {}

#[cfg(feature = "VZVirtioSoundDeviceStreamConfiguration")]
unsafe impl CopyingHelper for VZVirtioSoundDeviceOutputStreamConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZVirtioSoundDeviceStreamConfiguration")]
unsafe impl NSObjectProtocol for VZVirtioSoundDeviceOutputStreamConfiguration {}

extern_methods!(
    #[cfg(feature = "VZVirtioSoundDeviceStreamConfiguration")]
    unsafe impl VZVirtioSoundDeviceOutputStreamConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "VZAudioOutputStreamSink")]
        #[method_id(@__retain_semantics Other sink)]
        pub unsafe fn sink(&self) -> Option<Retained<VZAudioOutputStreamSink>>;

        #[cfg(feature = "VZAudioOutputStreamSink")]
        #[method(setSink:)]
        pub unsafe fn setSink(&self, sink: Option<&VZAudioOutputStreamSink>);
    }
);

extern_methods!(
    /// Methods declared on superclass `VZVirtioSoundDeviceStreamConfiguration`
    #[cfg(feature = "VZVirtioSoundDeviceStreamConfiguration")]
    unsafe impl VZVirtioSoundDeviceOutputStreamConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
