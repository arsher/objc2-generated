//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmachardwaremodel?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMacHardwareModel;
);

unsafe impl NSCopying for VZMacHardwareModel {}

unsafe impl CopyingHelper for VZMacHardwareModel {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZMacHardwareModel {}

extern_methods!(
    unsafe impl VZMacHardwareModel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDataRepresentation:)]
        pub unsafe fn initWithDataRepresentation(
            this: Allocated<Self>,
            data_representation: &NSData,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other dataRepresentation)]
        pub unsafe fn dataRepresentation(&self) -> Retained<NSData>;

        #[method(isSupported)]
        pub unsafe fn isSupported(&self) -> bool;
    }
);
