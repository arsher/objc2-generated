//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnull?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNull;
);

unsafe impl Send for NSNull {}

unsafe impl Sync for NSNull {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSNull {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSNull {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSNull {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSNull {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSNull {}

extern_methods!(
    unsafe impl NSNull {
        #[method_id(@__retain_semantics Other null)]
        pub unsafe fn null() -> Retained<NSNull>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSNull {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
