//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlctensoroptimizerdevicedata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCTensorOptimizerDeviceData;
);

unsafe impl NSCopying for MLCTensorOptimizerDeviceData {}

unsafe impl CopyingHelper for MLCTensorOptimizerDeviceData {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCTensorOptimizerDeviceData {}

extern_methods!(
    unsafe impl MLCTensorOptimizerDeviceData {
        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLCTensorOptimizerDeviceData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
