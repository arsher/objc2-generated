//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/classkit/clsactivityitem?language=objc)
    #[unsafe(super(CLSObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CLSObject")]
    pub struct CLSActivityItem;
);

#[cfg(feature = "CLSObject")]
unsafe impl NSCoding for CLSActivityItem {}

#[cfg(feature = "CLSObject")]
unsafe impl NSObjectProtocol for CLSActivityItem {}

#[cfg(feature = "CLSObject")]
unsafe impl NSSecureCoding for CLSActivityItem {}

extern_methods!(
    #[cfg(feature = "CLSObject")]
    unsafe impl CLSActivityItem {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
