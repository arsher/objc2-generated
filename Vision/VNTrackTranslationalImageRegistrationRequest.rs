//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vntracktranslationalimageregistrationrequest?language=objc)
    #[unsafe(super(VNStatefulRequest, VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    pub struct VNTrackTranslationalImageRegistrationRequest;
);

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl NSCopying for VNTrackTranslationalImageRegistrationRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl CopyingHelper for VNTrackTranslationalImageRegistrationRequest {
    type Result = Self;
}

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl NSObjectProtocol for VNTrackTranslationalImageRegistrationRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl VNTrackTranslationalImageRegistrationRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(
            &self,
        ) -> Option<Retained<NSArray<VNImageTranslationAlignmentObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNStatefulRequest`
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl VNTrackTranslationalImageRegistrationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vntracktranslationalimageregistrationrequestrevision1?language=objc)
pub static VNTrackTranslationalImageRegistrationRequestRevision1: NSUInteger = 1;
