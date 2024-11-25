//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-image")]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnimageoption?language=objc)
// NS_TYPED_ENUM
pub type VNImageOption = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnimageoptionproperties?language=objc)
    pub static VNImageOptionProperties: &'static VNImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnimageoptioncameraintrinsics?language=objc)
    pub static VNImageOptionCameraIntrinsics: &'static VNImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnimageoptioncicontext?language=objc)
    pub static VNImageOptionCIContext: &'static VNImageOption;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnimagerequesthandler?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNImageRequestHandler;
);

unsafe impl NSObjectProtocol for VNImageRequestHandler {}

extern_methods!(
    unsafe impl VNImageRequestHandler {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-image")]
        #[method_id(@__retain_semantics Init initWithCIImage:options:)]
        pub unsafe fn initWithCIImage_options(
            this: Allocated<Self>,
            image: &CIImage,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithURL:options:)]
        pub unsafe fn initWithURL_options(
            this: Allocated<Self>,
            image_url: &NSURL,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithData:options:)]
        pub unsafe fn initWithData_options(
            this: Allocated<Self>,
            image_data: &NSData,
            options: &NSDictionary<VNImageOption, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(feature = "VNRequest")]
        #[method(performRequests:error:_)]
        pub unsafe fn performRequests_error(
            &self,
            requests: &NSArray<VNRequest>,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNImageRequestHandler {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnsequencerequesthandler?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNSequenceRequestHandler;
);

unsafe impl NSObjectProtocol for VNSequenceRequestHandler {}

extern_methods!(
    unsafe impl VNSequenceRequestHandler {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "VNRequest", feature = "objc2-core-image"))]
        #[method(performRequests:onCIImage:error:_)]
        pub unsafe fn performRequests_onCIImage_error(
            &self,
            requests: &NSArray<VNRequest>,
            image: &CIImage,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "VNRequest")]
        #[method(performRequests:onImageURL:error:_)]
        pub unsafe fn performRequests_onImageURL_error(
            &self,
            requests: &NSArray<VNRequest>,
            image_url: &NSURL,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "VNRequest")]
        #[method(performRequests:onImageData:error:_)]
        pub unsafe fn performRequests_onImageData_error(
            &self,
            requests: &NSArray<VNRequest>,
            image_data: &NSData,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNSequenceRequestHandler {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
