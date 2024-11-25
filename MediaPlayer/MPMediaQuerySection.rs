//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaquerysection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMediaQuerySection;
);

unsafe impl NSCoding for MPMediaQuerySection {}

unsafe impl NSCopying for MPMediaQuerySection {}

unsafe impl CopyingHelper for MPMediaQuerySection {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MPMediaQuerySection {}

unsafe impl NSSecureCoding for MPMediaQuerySection {}

extern_methods!(
    unsafe impl MPMediaQuerySection {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(range)]
        pub unsafe fn range(&self) -> NSRange;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPMediaQuerySection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
