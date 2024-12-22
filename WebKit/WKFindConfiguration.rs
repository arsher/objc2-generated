//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkfindconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKFindConfiguration;
);

unsafe impl NSCopying for WKFindConfiguration {}

unsafe impl CopyingHelper for WKFindConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for WKFindConfiguration {}

extern_methods!(
    unsafe impl WKFindConfiguration {
        #[method(backwards)]
        pub unsafe fn backwards(&self) -> bool;

        /// Setter for [`backwards`][Self::backwards].
        #[method(setBackwards:)]
        pub unsafe fn setBackwards(&self, backwards: bool);

        #[method(caseSensitive)]
        pub unsafe fn caseSensitive(&self) -> bool;

        /// Setter for [`caseSensitive`][Self::caseSensitive].
        #[method(setCaseSensitive:)]
        pub unsafe fn setCaseSensitive(&self, case_sensitive: bool);

        #[method(wraps)]
        pub unsafe fn wraps(&self) -> bool;

        /// Setter for [`wraps`][Self::wraps].
        #[method(setWraps:)]
        pub unsafe fn setWraps(&self, wraps: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKFindConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
