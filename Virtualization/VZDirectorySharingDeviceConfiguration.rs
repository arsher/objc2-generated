//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZDirectorySharingDeviceConfiguration;

    unsafe impl ClassType for VZDirectorySharingDeviceConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for VZDirectorySharingDeviceConfiguration {}

unsafe impl NSObjectProtocol for VZDirectorySharingDeviceConfiguration {}

extern_methods!(
    unsafe impl VZDirectorySharingDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);