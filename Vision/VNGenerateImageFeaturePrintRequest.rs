//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNGenerateImageFeaturePrintRequest;

    #[cfg(feature = "VNRequest")]
    unsafe impl ClassType for VNGenerateImageFeaturePrintRequest {
        #[inherits(VNRequest, NSObject)]
        type Super = VNImageBasedRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNGenerateImageFeaturePrintRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNGenerateImageFeaturePrintRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNGenerateImageFeaturePrintRequest {
        #[cfg(feature = "VNTypes")]
        #[method(imageCropAndScaleOption)]
        pub unsafe fn imageCropAndScaleOption(&self) -> VNImageCropAndScaleOption;

        #[cfg(feature = "VNTypes")]
        #[method(setImageCropAndScaleOption:)]
        pub unsafe fn setImageCropAndScaleOption(
            &self,
            image_crop_and_scale_option: VNImageCropAndScaleOption,
        );

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNFeaturePrintObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNGenerateImageFeaturePrintRequest {
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
    unsafe impl VNGenerateImageFeaturePrintRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

pub static VNGenerateImageFeaturePrintRequestRevision1: NSUInteger = 1;

pub static VNGenerateImageFeaturePrintRequestRevision2: NSUInteger = 2;
