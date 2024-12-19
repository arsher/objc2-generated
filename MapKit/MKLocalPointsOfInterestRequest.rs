//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkpointsofinterestrequestmaxradius?language=objc)
    #[cfg(feature = "objc2-core-location")]
    pub static MKPointsOfInterestRequestMaxRadius: CLLocationDistance;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklocalpointsofinterestrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLocalPointsOfInterestRequest;
);

unsafe impl NSCopying for MKLocalPointsOfInterestRequest {}

unsafe impl CopyingHelper for MKLocalPointsOfInterestRequest {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MKLocalPointsOfInterestRequest {}

extern_methods!(
    unsafe impl MKLocalPointsOfInterestRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Init initWithCenterCoordinate:radius:)]
        pub unsafe fn initWithCenterCoordinate_radius(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            radius: CLLocationDistance,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method_id(@__retain_semantics Init initWithCoordinateRegion:)]
        pub unsafe fn initWithCoordinateRegion(
            this: Allocated<Self>,
            region: MKCoordinateRegion,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "objc2-core-location")]
        #[method(radius)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Retained<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLocalPointsOfInterestRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
