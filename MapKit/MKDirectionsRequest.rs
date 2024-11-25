//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkdirectionsroutepreference?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKDirectionsRoutePreference(pub NSInteger);
impl MKDirectionsRoutePreference {
    #[doc(alias = "MKDirectionsRoutePreferenceAny")]
    pub const Any: Self = Self(0);
    #[doc(alias = "MKDirectionsRoutePreferenceAvoid")]
    pub const Avoid: Self = Self(1);
}

unsafe impl Encode for MKDirectionsRoutePreference {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MKDirectionsRoutePreference {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkdirectionsrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKDirectionsRequest;
);

unsafe impl NSObjectProtocol for MKDirectionsRequest {}

extern_methods!(
    unsafe impl MKDirectionsRequest {
        #[cfg(feature = "MKMapItem")]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Option<Retained<MKMapItem>>;

        #[cfg(feature = "MKMapItem")]
        #[method(setSource:)]
        pub unsafe fn setSource(&self, source: Option<&MKMapItem>);

        #[cfg(feature = "MKMapItem")]
        #[method_id(@__retain_semantics Other destination)]
        pub unsafe fn destination(&self) -> Option<Retained<MKMapItem>>;

        #[cfg(feature = "MKMapItem")]
        #[method(setDestination:)]
        pub unsafe fn setDestination(&self, destination: Option<&MKMapItem>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKDirectionsRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// MKRequestOptions
    unsafe impl MKDirectionsRequest {
        #[cfg(feature = "MKDirectionsTypes")]
        #[method(transportType)]
        pub unsafe fn transportType(&self) -> MKDirectionsTransportType;

        #[cfg(feature = "MKDirectionsTypes")]
        #[method(setTransportType:)]
        pub unsafe fn setTransportType(&self, transport_type: MKDirectionsTransportType);

        #[method(requestsAlternateRoutes)]
        pub unsafe fn requestsAlternateRoutes(&self) -> bool;

        #[method(setRequestsAlternateRoutes:)]
        pub unsafe fn setRequestsAlternateRoutes(&self, requests_alternate_routes: bool);

        #[method_id(@__retain_semantics Other departureDate)]
        pub unsafe fn departureDate(&self) -> Option<Retained<NSDate>>;

        #[method(setDepartureDate:)]
        pub unsafe fn setDepartureDate(&self, departure_date: Option<&NSDate>);

        #[method_id(@__retain_semantics Other arrivalDate)]
        pub unsafe fn arrivalDate(&self) -> Option<Retained<NSDate>>;

        #[method(setArrivalDate:)]
        pub unsafe fn setArrivalDate(&self, arrival_date: Option<&NSDate>);

        #[method(tollPreference)]
        pub unsafe fn tollPreference(&self) -> MKDirectionsRoutePreference;

        #[method(setTollPreference:)]
        pub unsafe fn setTollPreference(&self, toll_preference: MKDirectionsRoutePreference);

        #[method(highwayPreference)]
        pub unsafe fn highwayPreference(&self) -> MKDirectionsRoutePreference;

        #[method(setHighwayPreference:)]
        pub unsafe fn setHighwayPreference(&self, highway_preference: MKDirectionsRoutePreference);
    }
);

extern_methods!(
    /// MKDirectionsURL
    unsafe impl MKDirectionsRequest {
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[method(isDirectionsRequestURL:)]
        pub unsafe fn isDirectionsRequestURL(url: &NSURL) -> bool;
    }
);
