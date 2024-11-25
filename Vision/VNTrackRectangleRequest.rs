//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vntrackrectanglerequest?language=objc)
    #[unsafe(super(VNTrackingRequest, VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
    pub struct VNTrackRectangleRequest;
);

#[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
unsafe impl NSCopying for VNTrackRectangleRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
unsafe impl CopyingHelper for VNTrackRectangleRequest {
    type Result = Self;
}

#[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
unsafe impl NSObjectProtocol for VNTrackRectangleRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
    unsafe impl VNTrackRectangleRequest {
        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Init initWithRectangleObservation:)]
        pub unsafe fn initWithRectangleObservation(
            this: Allocated<Self>,
            observation: &VNRectangleObservation,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNObservation", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithRectangleObservation:completionHandler:)]
        pub unsafe fn initWithRectangleObservation_completionHandler(
            this: Allocated<Self>,
            observation: &VNRectangleObservation,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

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
    #[cfg(all(feature = "VNRequest", feature = "VNTrackingRequest"))]
    unsafe impl VNTrackRectangleRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vntrackrectanglerequestrevision1?language=objc)
pub static VNTrackRectangleRequestRevision1: NSUInteger = 1;
