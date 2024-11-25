//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmackeyboardconfiguration?language=objc)
    #[unsafe(super(VZKeyboardConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZKeyboardConfiguration")]
    pub struct VZMacKeyboardConfiguration;
);

#[cfg(feature = "VZKeyboardConfiguration")]
unsafe impl NSCopying for VZMacKeyboardConfiguration {}

#[cfg(feature = "VZKeyboardConfiguration")]
unsafe impl CopyingHelper for VZMacKeyboardConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZKeyboardConfiguration")]
unsafe impl NSObjectProtocol for VZMacKeyboardConfiguration {}

extern_methods!(
    #[cfg(feature = "VZKeyboardConfiguration")]
    unsafe impl VZMacKeyboardConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZKeyboardConfiguration`
    #[cfg(feature = "VZKeyboardConfiguration")]
    unsafe impl VZMacKeyboardConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
