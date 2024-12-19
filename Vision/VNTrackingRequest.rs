//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnrequesttrackinglevel?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VNRequestTrackingLevel(pub NSUInteger);
impl VNRequestTrackingLevel {
    #[doc(alias = "VNRequestTrackingLevelAccurate")]
    pub const Accurate: Self = Self(0);
    #[doc(alias = "VNRequestTrackingLevelFast")]
    pub const Fast: Self = Self(1);
}

unsafe impl Encode for VNRequestTrackingLevel {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for VNRequestTrackingLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vntrackingrequest?language=objc)
    #[unsafe(super(VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNTrackingRequest;
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNTrackingRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNTrackingRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNTrackingRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNTrackingRequest {
        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other inputObservation)]
        pub unsafe fn inputObservation(&self) -> Retained<VNDetectedObjectObservation>;

        #[cfg(feature = "VNObservation")]
        #[method(setInputObservation:)]
        pub unsafe fn setInputObservation(&self, input_observation: &VNDetectedObjectObservation);

        #[method(trackingLevel)]
        pub unsafe fn trackingLevel(&self) -> VNRequestTrackingLevel;

        #[method(setTrackingLevel:)]
        pub unsafe fn setTrackingLevel(&self, tracking_level: VNRequestTrackingLevel);

        #[method(isLastFrame)]
        pub unsafe fn isLastFrame(&self) -> bool;

        #[method(setLastFrame:)]
        pub unsafe fn setLastFrame(&self, last_frame: bool);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNTrackingRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
