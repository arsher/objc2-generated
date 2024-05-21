//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static NINearbyObjectDistanceNotAvailable: c_float;
}

extern "C" {
    pub static NINearbyObjectAngleNotAvailable: c_float;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NINearbyObjectVerticalDirectionEstimate(pub NSInteger);
impl NINearbyObjectVerticalDirectionEstimate {
    #[doc(alias = "NINearbyObjectVerticalDirectionEstimateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "NINearbyObjectVerticalDirectionEstimateSame")]
    pub const Same: Self = Self(1);
    #[doc(alias = "NINearbyObjectVerticalDirectionEstimateAbove")]
    pub const Above: Self = Self(2);
    #[doc(alias = "NINearbyObjectVerticalDirectionEstimateBelow")]
    pub const Below: Self = Self(3);
    #[doc(alias = "NINearbyObjectVerticalDirectionEstimateAboveOrBelow")]
    pub const AboveOrBelow: Self = Self(4);
}

unsafe impl Encode for NINearbyObjectVerticalDirectionEstimate {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NINearbyObjectVerticalDirectionEstimate {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NINearbyObject;

    unsafe impl ClassType for NINearbyObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NINearbyObject {}

unsafe impl NSCopying for NINearbyObject {}

unsafe impl NSObjectProtocol for NINearbyObject {}

unsafe impl NSSecureCoding for NINearbyObject {}

extern_methods!(
    unsafe impl NINearbyObject {
        #[cfg(feature = "NIConfiguration")]
        #[method_id(@__retain_semantics Other discoveryToken)]
        pub unsafe fn discoveryToken(&self) -> Retained<NIDiscoveryToken>;

        #[method(distance)]
        pub unsafe fn distance(&self) -> c_float;

        #[method(verticalDirectionEstimate)]
        pub unsafe fn verticalDirectionEstimate(&self) -> NINearbyObjectVerticalDirectionEstimate;

        #[method(horizontalAngle)]
        pub unsafe fn horizontalAngle(&self) -> c_float;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
