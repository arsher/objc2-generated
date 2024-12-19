//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklocalsearchresulttype?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKLocalSearchResultType(pub NSUInteger);
bitflags::bitflags! {
    impl MKLocalSearchResultType: NSUInteger {
        #[doc(alias = "MKLocalSearchResultTypeAddress")]
        const Address = 1<<0;
        #[doc(alias = "MKLocalSearchResultTypePointOfInterest")]
        const PointOfInterest = 1<<1;
        #[doc(alias = "MKLocalSearchResultTypePhysicalFeature")]
        const PhysicalFeature = 1<<2;
    }
}

unsafe impl Encode for MKLocalSearchResultType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MKLocalSearchResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklocalsearchrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLocalSearchRequest;
);

unsafe impl NSCopying for MKLocalSearchRequest {}

unsafe impl CopyingHelper for MKLocalSearchRequest {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MKLocalSearchRequest {}

extern_methods!(
    unsafe impl MKLocalSearchRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithNaturalLanguageQuery:)]
        pub unsafe fn initWithNaturalLanguageQuery(
            this: Allocated<Self>,
            natural_language_query: &NSString,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method_id(@__retain_semantics Init initWithNaturalLanguageQuery:region:)]
        pub unsafe fn initWithNaturalLanguageQuery_region(
            this: Allocated<Self>,
            natural_language_query: &NSString,
            region: MKCoordinateRegion,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other naturalLanguageQuery)]
        pub unsafe fn naturalLanguageQuery(&self) -> Option<Retained<NSString>>;

        #[method(setNaturalLanguageQuery:)]
        pub unsafe fn setNaturalLanguageQuery(&self, natural_language_query: Option<&NSString>);

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[cfg(feature = "MKTypes")]
        #[method(regionPriority)]
        pub unsafe fn regionPriority(&self) -> MKLocalSearchRegionPriority;

        #[cfg(feature = "MKTypes")]
        #[method(setRegionPriority:)]
        pub unsafe fn setRegionPriority(&self, region_priority: MKLocalSearchRegionPriority);

        #[method(resultTypes)]
        pub unsafe fn resultTypes(&self) -> MKLocalSearchResultType;

        #[method(setResultTypes:)]
        pub unsafe fn setResultTypes(&self, result_types: MKLocalSearchResultType);

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Retained<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[cfg(feature = "MKAddressFilter")]
        #[method_id(@__retain_semantics Other addressFilter)]
        pub unsafe fn addressFilter(&self) -> Option<Retained<MKAddressFilter>>;

        #[cfg(feature = "MKAddressFilter")]
        #[method(setAddressFilter:)]
        pub unsafe fn setAddressFilter(&self, address_filter: Option<&MKAddressFilter>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLocalSearchRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
