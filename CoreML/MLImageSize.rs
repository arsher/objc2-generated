//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlimagesize?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLImageSize;
);

unsafe impl NSCoding for MLImageSize {}

unsafe impl NSObjectProtocol for MLImageSize {}

unsafe impl NSSecureCoding for MLImageSize {}

extern_methods!(
    unsafe impl MLImageSize {
        #[method(pixelsWide)]
        pub unsafe fn pixelsWide(&self) -> NSInteger;

        #[method(pixelsHigh)]
        pub unsafe fn pixelsHigh(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLImageSize {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
