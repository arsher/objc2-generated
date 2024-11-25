//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-map-kit")]
use objc2_map_kit::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekstructuredlocation?language=objc)
    #[unsafe(super(EKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EKObject")]
    pub struct EKStructuredLocation;
);

#[cfg(feature = "EKObject")]
unsafe impl NSCopying for EKStructuredLocation {}

#[cfg(feature = "EKObject")]
unsafe impl CopyingHelper for EKStructuredLocation {
    type Result = Self;
}

#[cfg(feature = "EKObject")]
unsafe impl NSObjectProtocol for EKStructuredLocation {}

extern_methods!(
    #[cfg(feature = "EKObject")]
    unsafe impl EKStructuredLocation {
        #[method_id(@__retain_semantics Other locationWithTitle:)]
        pub unsafe fn locationWithTitle(title: &NSString) -> Retained<Self>;

        #[cfg(feature = "objc2-map-kit")]
        #[method_id(@__retain_semantics Other locationWithMapItem:)]
        pub unsafe fn locationWithMapItem(map_item: &MKMapItem) -> Retained<Self>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other geoLocation)]
        pub unsafe fn geoLocation(&self) -> Option<Retained<CLLocation>>;

        #[cfg(feature = "objc2-core-location")]
        #[method(setGeoLocation:)]
        pub unsafe fn setGeoLocation(&self, geo_location: Option<&CLLocation>);

        #[method(radius)]
        pub unsafe fn radius(&self) -> c_double;

        #[method(setRadius:)]
        pub unsafe fn setRadius(&self, radius: c_double);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "EKObject")]
    unsafe impl EKStructuredLocation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
