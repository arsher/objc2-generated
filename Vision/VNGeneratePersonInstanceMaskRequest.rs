//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A request that generates an instance mask of individual people found in the image.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vngeneratepersoninstancemaskrequest?language=objc)
    #[unsafe(super(VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNGeneratePersonInstanceMaskRequest;
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNGeneratePersonInstanceMaskRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNGeneratePersonInstanceMaskRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNGeneratePersonInstanceMaskRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNGeneratePersonInstanceMaskRequest {
        #[cfg(feature = "VNObservation")]
        /// VNObservation results.
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNInstanceMaskObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNGeneratePersonInstanceMaskRequest {
        /// Creates a new VNRequest with no completion handler.
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Creates a new VNRequest with an optional completion handler.
        ///
        ///
        /// Parameter `completionHandler`: The block to be invoked after the request has completed its processing. The completion handler gets executed on the same dispatch queue as the request being executed.
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
    unsafe impl VNGeneratePersonInstanceMaskRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vngeneratepersoninstancemaskrequestrevision1?language=objc)
pub static VNGeneratePersonInstanceMaskRequestRevision1: NSUInteger = 1;
