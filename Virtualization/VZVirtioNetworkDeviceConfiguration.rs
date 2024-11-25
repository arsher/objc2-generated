//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtionetworkdeviceconfiguration?language=objc)
    #[unsafe(super(VZNetworkDeviceConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZNetworkDeviceConfiguration")]
    pub struct VZVirtioNetworkDeviceConfiguration;
);

#[cfg(feature = "VZNetworkDeviceConfiguration")]
unsafe impl NSCopying for VZVirtioNetworkDeviceConfiguration {}

#[cfg(feature = "VZNetworkDeviceConfiguration")]
unsafe impl CopyingHelper for VZVirtioNetworkDeviceConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZNetworkDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZVirtioNetworkDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZNetworkDeviceConfiguration")]
    unsafe impl VZVirtioNetworkDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZNetworkDeviceConfiguration`
    #[cfg(feature = "VZNetworkDeviceConfiguration")]
    unsafe impl VZVirtioNetworkDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
