//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZStorageDeviceConfiguration")]
    pub struct VZNVMExpressControllerDeviceConfiguration;

    #[cfg(feature = "VZStorageDeviceConfiguration")]
    unsafe impl ClassType for VZNVMExpressControllerDeviceConfiguration {
        #[inherits(NSObject)]
        type Super = VZStorageDeviceConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZStorageDeviceConfiguration")]
unsafe impl NSCopying for VZNVMExpressControllerDeviceConfiguration {}

#[cfg(feature = "VZStorageDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZNVMExpressControllerDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZStorageDeviceConfiguration")]
    unsafe impl VZNVMExpressControllerDeviceConfiguration {
        #[cfg(feature = "VZStorageDeviceAttachment")]
        #[method_id(@__retain_semantics Init initWithAttachment:)]
        pub unsafe fn initWithAttachment(
            this: Allocated<Self>,
            attachment: &VZStorageDeviceAttachment,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZStorageDeviceConfiguration`
    #[cfg(feature = "VZStorageDeviceConfiguration")]
    unsafe impl VZNVMExpressControllerDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
