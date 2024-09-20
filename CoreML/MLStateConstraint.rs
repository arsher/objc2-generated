//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLStateConstraint;

    unsafe impl ClassType for MLStateConstraint {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for MLStateConstraint {}

unsafe impl NSObjectProtocol for MLStateConstraint {}

unsafe impl NSSecureCoding for MLStateConstraint {}

extern_methods!(
    unsafe impl MLStateConstraint {
        #[method_id(@__retain_semantics Other bufferShape)]
        pub unsafe fn bufferShape(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "MLMultiArray")]
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MLMultiArrayDataType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLStateConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
