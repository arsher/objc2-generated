//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-image")]
use objc2_core_image::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnimageregistrationrequest?language=objc)
    #[unsafe(super(VNTargetedImageRequest, VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    pub struct VNImageRegistrationRequest;
);

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl NSCopying for VNImageRegistrationRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl CopyingHelper for VNImageRegistrationRequest {
    type Result = Self;
}

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl NSObjectProtocol for VNImageRegistrationRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNImageRegistrationRequest {}
);

extern_methods!(
    /// Methods declared on superclass `VNTargetedImageRequest`
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNImageRegistrationRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "objc2-core-video"))]
        #[method_id(@__retain_semantics Init initWithTargetedCVPixelBuffer:options:)]
        pub unsafe fn initWithTargetedCVPixelBuffer_options(
            this: Allocated<Self>,
            pixel_buffer: CVPixelBufferRef,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "VNRequestHandler",
            feature = "block2",
            feature = "objc2-core-video"
        ))]
        #[method_id(@__retain_semantics Init initWithTargetedCVPixelBuffer:options:completionHandler:)]
        pub unsafe fn initWithTargetedCVPixelBuffer_options_completionHandler(
            this: Allocated<Self>,
            pixel_buffer: CVPixelBufferRef,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "objc2-core-image"))]
        #[method_id(@__retain_semantics Init initWithTargetedCIImage:options:)]
        pub unsafe fn initWithTargetedCIImage_options(
            this: Allocated<Self>,
            ci_image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "VNRequestHandler",
            feature = "block2",
            feature = "objc2-core-image"
        ))]
        #[method_id(@__retain_semantics Init initWithTargetedCIImage:options:completionHandler:)]
        pub unsafe fn initWithTargetedCIImage_options_completionHandler(
            this: Allocated<Self>,
            ci_image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequestHandler")]
        #[method_id(@__retain_semantics Init initWithTargetedImageURL:options:)]
        pub unsafe fn initWithTargetedImageURL_options(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTargetedImageURL:options:completionHandler:)]
        pub unsafe fn initWithTargetedImageURL_options_completionHandler(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequestHandler")]
        #[method_id(@__retain_semantics Init initWithTargetedImageData:options:)]
        pub unsafe fn initWithTargetedImageData_options(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTargetedImageData:options:completionHandler:)]
        pub unsafe fn initWithTargetedImageData_options_completionHandler(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNImageRegistrationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vntranslationalimageregistrationrequest?language=objc)
    #[unsafe(super(
        VNImageRegistrationRequest,
        VNTargetedImageRequest,
        VNImageBasedRequest,
        VNRequest,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    pub struct VNTranslationalImageRegistrationRequest;
);

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl NSCopying for VNTranslationalImageRegistrationRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl CopyingHelper for VNTranslationalImageRegistrationRequest {
    type Result = Self;
}

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl NSObjectProtocol for VNTranslationalImageRegistrationRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNTranslationalImageRegistrationRequest {
        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(
            &self,
        ) -> Option<Retained<NSArray<VNImageTranslationAlignmentObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNTargetedImageRequest`
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNTranslationalImageRegistrationRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "objc2-core-video"))]
        #[method_id(@__retain_semantics Init initWithTargetedCVPixelBuffer:options:)]
        pub unsafe fn initWithTargetedCVPixelBuffer_options(
            this: Allocated<Self>,
            pixel_buffer: CVPixelBufferRef,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "VNRequestHandler",
            feature = "block2",
            feature = "objc2-core-video"
        ))]
        #[method_id(@__retain_semantics Init initWithTargetedCVPixelBuffer:options:completionHandler:)]
        pub unsafe fn initWithTargetedCVPixelBuffer_options_completionHandler(
            this: Allocated<Self>,
            pixel_buffer: CVPixelBufferRef,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "objc2-core-image"))]
        #[method_id(@__retain_semantics Init initWithTargetedCIImage:options:)]
        pub unsafe fn initWithTargetedCIImage_options(
            this: Allocated<Self>,
            ci_image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "VNRequestHandler",
            feature = "block2",
            feature = "objc2-core-image"
        ))]
        #[method_id(@__retain_semantics Init initWithTargetedCIImage:options:completionHandler:)]
        pub unsafe fn initWithTargetedCIImage_options_completionHandler(
            this: Allocated<Self>,
            ci_image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequestHandler")]
        #[method_id(@__retain_semantics Init initWithTargetedImageURL:options:)]
        pub unsafe fn initWithTargetedImageURL_options(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTargetedImageURL:options:completionHandler:)]
        pub unsafe fn initWithTargetedImageURL_options_completionHandler(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequestHandler")]
        #[method_id(@__retain_semantics Init initWithTargetedImageData:options:)]
        pub unsafe fn initWithTargetedImageData_options(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTargetedImageData:options:completionHandler:)]
        pub unsafe fn initWithTargetedImageData_options_completionHandler(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNTranslationalImageRegistrationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vntranslationalimageregistrationrequestrevision1?language=objc)
pub static VNTranslationalImageRegistrationRequestRevision1: NSUInteger = 1;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhomographicimageregistrationrequest?language=objc)
    #[unsafe(super(
        VNImageRegistrationRequest,
        VNTargetedImageRequest,
        VNImageBasedRequest,
        VNRequest,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    pub struct VNHomographicImageRegistrationRequest;
);

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl NSCopying for VNHomographicImageRegistrationRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl CopyingHelper for VNHomographicImageRegistrationRequest {
    type Result = Self;
}

#[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
unsafe impl NSObjectProtocol for VNHomographicImageRegistrationRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNHomographicImageRegistrationRequest {
        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(
            &self,
        ) -> Option<Retained<NSArray<VNImageHomographicAlignmentObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNTargetedImageRequest`
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNHomographicImageRegistrationRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "objc2-core-video"))]
        #[method_id(@__retain_semantics Init initWithTargetedCVPixelBuffer:options:)]
        pub unsafe fn initWithTargetedCVPixelBuffer_options(
            this: Allocated<Self>,
            pixel_buffer: CVPixelBufferRef,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "VNRequestHandler",
            feature = "block2",
            feature = "objc2-core-video"
        ))]
        #[method_id(@__retain_semantics Init initWithTargetedCVPixelBuffer:options:completionHandler:)]
        pub unsafe fn initWithTargetedCVPixelBuffer_options_completionHandler(
            this: Allocated<Self>,
            pixel_buffer: CVPixelBufferRef,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "objc2-core-image"))]
        #[method_id(@__retain_semantics Init initWithTargetedCIImage:options:)]
        pub unsafe fn initWithTargetedCIImage_options(
            this: Allocated<Self>,
            ci_image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "VNRequestHandler",
            feature = "block2",
            feature = "objc2-core-image"
        ))]
        #[method_id(@__retain_semantics Init initWithTargetedCIImage:options:completionHandler:)]
        pub unsafe fn initWithTargetedCIImage_options_completionHandler(
            this: Allocated<Self>,
            ci_image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequestHandler")]
        #[method_id(@__retain_semantics Init initWithTargetedImageURL:options:)]
        pub unsafe fn initWithTargetedImageURL_options(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTargetedImageURL:options:completionHandler:)]
        pub unsafe fn initWithTargetedImageURL_options_completionHandler(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequestHandler")]
        #[method_id(@__retain_semantics Init initWithTargetedImageData:options:)]
        pub unsafe fn initWithTargetedImageData_options(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "VNRequestHandler", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithTargetedImageData:options:completionHandler:)]
        pub unsafe fn initWithTargetedImageData_options_completionHandler(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "VNRequest", feature = "VNTargetedImageRequest"))]
    unsafe impl VNHomographicImageRegistrationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhomographicimageregistrationrequestrevision1?language=objc)
pub static VNHomographicImageRegistrationRequestRevision1: NSUInteger = 1;
