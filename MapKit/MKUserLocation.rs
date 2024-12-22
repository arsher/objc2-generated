//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkuserlocation?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKUserLocation;
);

#[cfg(feature = "MKAnnotation")]
unsafe impl MKAnnotation for MKUserLocation {}

unsafe impl NSObjectProtocol for MKUserLocation {}

extern_methods!(
    unsafe impl MKUserLocation {
        #[method(isUpdating)]
        pub unsafe fn isUpdating(&self) -> bool;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Retained<CLLocation>>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other heading)]
        pub unsafe fn heading(&self) -> Option<Retained<CLHeading>>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// Setter for [`title`][Self::title].
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        /// Setter for [`subtitle`][Self::subtitle].
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKUserLocation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
