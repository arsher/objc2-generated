//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MKStandardMapEmphasisStyle {
        MKStandardMapEmphasisStyleDefault = 0,
        MKStandardMapEmphasisStyleMuted = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKStandardMapConfiguration")]
    pub struct MKStandardMapConfiguration;

    #[cfg(feature = "MapKit_MKStandardMapConfiguration")]
    unsafe impl ClassType for MKStandardMapConfiguration {
        #[inherits(NSObject)]
        type Super = MKMapConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKStandardMapConfiguration")]
unsafe impl NSCoding for MKStandardMapConfiguration {}

#[cfg(feature = "MapKit_MKStandardMapConfiguration")]
unsafe impl NSCopying for MKStandardMapConfiguration {}

#[cfg(feature = "MapKit_MKStandardMapConfiguration")]
unsafe impl NSObjectProtocol for MKStandardMapConfiguration {}

#[cfg(feature = "MapKit_MKStandardMapConfiguration")]
unsafe impl NSSecureCoding for MKStandardMapConfiguration {}

extern_methods!(
    #[cfg(feature = "MapKit_MKStandardMapConfiguration")]
    unsafe impl MKStandardMapConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithElevationStyle:)]
        pub unsafe fn initWithElevationStyle(
            this: Option<Allocated<Self>>,
            elevation_style: MKMapElevationStyle,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithElevationStyle:emphasisStyle:)]
        pub unsafe fn initWithElevationStyle_emphasisStyle(
            this: Option<Allocated<Self>>,
            elevation_style: MKMapElevationStyle,
            emphasis_style: MKStandardMapEmphasisStyle,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithEmphasisStyle:)]
        pub unsafe fn initWithEmphasisStyle(
            this: Option<Allocated<Self>>,
            emphasis_style: MKStandardMapEmphasisStyle,
        ) -> Id<Self>;

        #[method(emphasisStyle)]
        pub unsafe fn emphasisStyle(&self) -> MKStandardMapEmphasisStyle;

        #[method(setEmphasisStyle:)]
        pub unsafe fn setEmphasisStyle(&self, emphasis_style: MKStandardMapEmphasisStyle);

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[method(showsTraffic)]
        pub unsafe fn showsTraffic(&self) -> bool;

        #[method(setShowsTraffic:)]
        pub unsafe fn setShowsTraffic(&self, shows_traffic: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKMapConfiguration`
    #[cfg(feature = "MapKit_MKStandardMapConfiguration")]
    unsafe impl MKStandardMapConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
