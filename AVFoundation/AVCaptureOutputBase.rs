//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureoutput?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureOutput;
);

unsafe impl NSObjectProtocol for AVCaptureOutput {}

extern_methods!(
    unsafe impl AVCaptureOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVCaptureSession")]
        #[method_id(@__retain_semantics Other connections)]
        pub unsafe fn connections(&self) -> Retained<NSArray<AVCaptureConnection>>;

        #[cfg(all(feature = "AVCaptureSession", feature = "AVMediaFormat"))]
        #[method_id(@__retain_semantics Other connectionWithMediaType:)]
        pub unsafe fn connectionWithMediaType(
            &self,
            media_type: &AVMediaType,
        ) -> Option<Retained<AVCaptureConnection>>;

        #[cfg(all(feature = "AVCaptureSession", feature = "AVMetadataObject"))]
        #[method_id(@__retain_semantics Other transformedMetadataObjectForMetadataObject:connection:)]
        pub unsafe fn transformedMetadataObjectForMetadataObject_connection(
            &self,
            metadata_object: &AVMetadataObject,
            connection: &AVCaptureConnection,
        ) -> Option<Retained<AVMetadataObject>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(metadataOutputRectOfInterestForRect:)]
        pub unsafe fn metadataOutputRectOfInterestForRect(
            &self,
            rect_in_output_coordinates: CGRect,
        ) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rectForMetadataOutputRectOfInterest:)]
        pub unsafe fn rectForMetadataOutputRectOfInterest(
            &self,
            rect_in_metadata_output_coordinates: CGRect,
        ) -> CGRect;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureoutputdatadroppedreason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptureOutputDataDroppedReason(pub NSInteger);
impl AVCaptureOutputDataDroppedReason {
    #[doc(alias = "AVCaptureOutputDataDroppedReasonNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "AVCaptureOutputDataDroppedReasonLateData")]
    pub const LateData: Self = Self(1);
    #[doc(alias = "AVCaptureOutputDataDroppedReasonOutOfBuffers")]
    pub const OutOfBuffers: Self = Self(2);
    #[doc(alias = "AVCaptureOutputDataDroppedReasonDiscontinuity")]
    pub const Discontinuity: Self = Self(3);
}

unsafe impl Encode for AVCaptureOutputDataDroppedReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptureOutputDataDroppedReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
