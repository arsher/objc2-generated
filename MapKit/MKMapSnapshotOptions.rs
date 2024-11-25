//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmapsnapshotoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapSnapshotOptions;
);

unsafe impl NSCopying for MKMapSnapshotOptions {}

unsafe impl CopyingHelper for MKMapSnapshotOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MKMapSnapshotOptions {}

extern_methods!(
    unsafe impl MKMapSnapshotOptions {
        #[cfg(feature = "MKMapConfiguration")]
        #[method_id(@__retain_semantics Other preferredConfiguration)]
        pub unsafe fn preferredConfiguration(&self) -> Retained<MKMapConfiguration>;

        #[cfg(feature = "MKMapConfiguration")]
        #[method(setPreferredConfiguration:)]
        pub unsafe fn setPreferredConfiguration(
            &self,
            preferred_configuration: &MKMapConfiguration,
        );

        #[cfg(feature = "MKMapCamera")]
        #[method_id(@__retain_semantics Other camera)]
        pub unsafe fn camera(&self) -> Retained<MKMapCamera>;

        #[cfg(feature = "MKMapCamera")]
        #[method(setCamera:)]
        pub unsafe fn setCamera(&self, camera: &MKMapCamera);

        #[cfg(feature = "MKGeometry")]
        #[method(mapRect)]
        pub unsafe fn mapRect(&self) -> MKMapRect;

        #[cfg(feature = "MKGeometry")]
        #[method(setMapRect:)]
        pub unsafe fn setMapRect(&self, map_rect: MKMapRect);

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[cfg(feature = "MKTypes")]
        #[deprecated = "Use preferredConfiguration"]
        #[method(mapType)]
        pub unsafe fn mapType(&self) -> MKMapType;

        #[cfg(feature = "MKTypes")]
        #[deprecated = "Use preferredConfiguration"]
        #[method(setMapType:)]
        pub unsafe fn setMapType(&self, map_type: MKMapType);

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[deprecated = "Use preferredConfiguration"]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Retained<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[deprecated = "Use preferredConfiguration"]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[deprecated = "Use preferredConfiguration"]
        #[method(showsPointsOfInterest)]
        pub unsafe fn showsPointsOfInterest(&self) -> bool;

        #[deprecated = "Use preferredConfiguration"]
        #[method(setShowsPointsOfInterest:)]
        pub unsafe fn setShowsPointsOfInterest(&self, shows_points_of_interest: bool);

        #[deprecated = "No longer supported."]
        #[method(showsBuildings)]
        pub unsafe fn showsBuildings(&self) -> bool;

        #[deprecated = "No longer supported."]
        #[method(setShowsBuildings:)]
        pub unsafe fn setShowsBuildings(&self, shows_buildings: bool);

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other appearance)]
        pub unsafe fn appearance(&self) -> Option<Retained<NSAppearance>>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method(setAppearance:)]
        pub unsafe fn setAppearance(&self, appearance: Option<&NSAppearance>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKMapSnapshotOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
