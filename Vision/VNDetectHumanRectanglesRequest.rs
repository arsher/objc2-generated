//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetecthumanrectanglesrequest?language=objc)
    #[unsafe(super(VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNDetectHumanRectanglesRequest;
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNDetectHumanRectanglesRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNDetectHumanRectanglesRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNDetectHumanRectanglesRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNDetectHumanRectanglesRequest {
        #[method(upperBodyOnly)]
        pub unsafe fn upperBodyOnly(&self) -> bool;

        #[method(setUpperBodyOnly:)]
        pub unsafe fn setUpperBodyOnly(&self, upper_body_only: bool);

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNHumanObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNDetectHumanRectanglesRequest {
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
    unsafe impl VNDetectHumanRectanglesRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetecthumanrectanglesrequestrevision1?language=objc)
pub static VNDetectHumanRectanglesRequestRevision1: NSUInteger = 1;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetecthumanrectanglesrequestrevision2?language=objc)
pub static VNDetectHumanRectanglesRequestRevision2: NSUInteger = 2;
