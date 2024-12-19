//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clregionstate?language=objc)
// NS_CLOSED_ENUM
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CLRegionState {
    #[doc(alias = "CLRegionStateUnknown")]
    Unknown = 0,
    #[doc(alias = "CLRegionStateInside")]
    Inside = 1,
    #[doc(alias = "CLRegionStateOutside")]
    Outside = 2,
}

unsafe impl Encode for CLRegionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CLRegionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clproximity?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLProximity(pub NSInteger);
impl CLProximity {
    #[doc(alias = "CLProximityUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CLProximityImmediate")]
    pub const Immediate: Self = Self(1);
    #[doc(alias = "CLProximityNear")]
    pub const Near: Self = Self(2);
    #[doc(alias = "CLProximityFar")]
    pub const Far: Self = Self(3);
}

unsafe impl Encode for CLProximity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CLProximity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clregion?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLRegion;
);

unsafe impl NSCoding for CLRegion {}

unsafe impl NSCopying for CLRegion {}

unsafe impl CopyingHelper for CLRegion {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CLRegion {}

unsafe impl NSSecureCoding for CLRegion {}

extern_methods!(
    unsafe impl CLRegion {
        #[cfg(feature = "CLLocation")]
        #[deprecated = "Please see CLCircularRegion"]
        #[method_id(@__retain_semantics Init initCircularRegionWithCenter:radius:identifier:)]
        pub unsafe fn initCircularRegionWithCenter_radius_identifier(
            this: Allocated<Self>,
            center: CLLocationCoordinate2D,
            radius: CLLocationDistance,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "CLLocation")]
        #[deprecated = "Please see CLCircularRegion"]
        #[method(center)]
        pub unsafe fn center(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "CLLocation")]
        #[deprecated = "Please see CLCircularRegion"]
        #[method(radius)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method(notifyOnEntry)]
        pub unsafe fn notifyOnEntry(&self) -> bool;

        #[method(setNotifyOnEntry:)]
        pub unsafe fn setNotifyOnEntry(&self, notify_on_entry: bool);

        #[method(notifyOnExit)]
        pub unsafe fn notifyOnExit(&self) -> bool;

        #[method(setNotifyOnExit:)]
        pub unsafe fn setNotifyOnExit(&self, notify_on_exit: bool);

        #[cfg(feature = "CLLocation")]
        #[deprecated = "Please see CLCircularRegion"]
        #[method(containsCoordinate:)]
        pub unsafe fn containsCoordinate(&self, coordinate: CLLocationCoordinate2D) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLRegion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
