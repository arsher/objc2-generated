//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkhybridmapconfiguration?language=objc)
    #[unsafe(super(MKMapConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKMapConfiguration")]
    pub struct MKHybridMapConfiguration;
);

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSCoding for MKHybridMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSCopying for MKHybridMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl CopyingHelper for MKHybridMapConfiguration {
    type Result = Self;
}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSObjectProtocol for MKHybridMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSSecureCoding for MKHybridMapConfiguration {}

extern_methods!(
    #[cfg(feature = "MKMapConfiguration")]
    unsafe impl MKHybridMapConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithElevationStyle:)]
        pub unsafe fn initWithElevationStyle(
            this: Allocated<Self>,
            elevation_style: MKMapElevationStyle,
        ) -> Retained<Self>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Retained<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        /// Setter for [`pointOfInterestFilter`][Self::pointOfInterestFilter].
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[method(showsTraffic)]
        pub unsafe fn showsTraffic(&self) -> bool;

        /// Setter for [`showsTraffic`][Self::showsTraffic].
        #[method(setShowsTraffic:)]
        pub unsafe fn setShowsTraffic(&self, shows_traffic: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKMapConfiguration`
    #[cfg(feature = "MKMapConfiguration")]
    unsafe impl MKHybridMapConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
