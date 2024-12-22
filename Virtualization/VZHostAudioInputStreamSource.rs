//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Host audio input stream source provides audio from the host system's default input device.
    ///
    /// Host input data comes from the same device that AudioQueueNewInput uses.
    ///
    ///
    /// See: VZVirtioSoundDeviceInputStreamConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzhostaudioinputstreamsource?language=objc)
    #[unsafe(super(VZAudioInputStreamSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZAudioInputStreamSource")]
    pub struct VZHostAudioInputStreamSource;
);

#[cfg(feature = "VZAudioInputStreamSource")]
unsafe impl NSObjectProtocol for VZHostAudioInputStreamSource {}

extern_methods!(
    #[cfg(feature = "VZAudioInputStreamSource")]
    unsafe impl VZHostAudioInputStreamSource {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZAudioInputStreamSource`
    #[cfg(feature = "VZAudioInputStreamSource")]
    unsafe impl VZHostAudioInputStreamSource {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
