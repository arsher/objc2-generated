//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklocalsearchresponse?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLocalSearchResponse;
);

unsafe impl NSObjectProtocol for MKLocalSearchResponse {}

extern_methods!(
    unsafe impl MKLocalSearchResponse {
        #[cfg(feature = "MKMapItem")]
        #[method_id(@__retain_semantics Other mapItems)]
        pub unsafe fn mapItems(&self) -> Retained<NSArray<MKMapItem>>;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method(boundingRegion)]
        pub unsafe fn boundingRegion(&self) -> MKCoordinateRegion;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLocalSearchResponse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
