//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzconsoledeviceconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZConsoleDeviceConfiguration;
);

unsafe impl NSCopying for VZConsoleDeviceConfiguration {}

unsafe impl CopyingHelper for VZConsoleDeviceConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZConsoleDeviceConfiguration {}

extern_methods!(
    unsafe impl VZConsoleDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
