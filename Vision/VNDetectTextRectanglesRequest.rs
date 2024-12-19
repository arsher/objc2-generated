//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetecttextrectanglesrequest?language=objc)
    #[unsafe(super(VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNDetectTextRectanglesRequest;
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNDetectTextRectanglesRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNDetectTextRectanglesRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNDetectTextRectanglesRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNDetectTextRectanglesRequest {
        #[method(reportCharacterBoxes)]
        pub unsafe fn reportCharacterBoxes(&self) -> bool;

        #[method(setReportCharacterBoxes:)]
        pub unsafe fn setReportCharacterBoxes(&self, report_character_boxes: bool);

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNTextObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNDetectTextRectanglesRequest {
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
    unsafe impl VNDetectTextRectanglesRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetecttextrectanglesrequestrevision1?language=objc)
pub static VNDetectTextRectanglesRequestRevision1: NSUInteger = 1;
