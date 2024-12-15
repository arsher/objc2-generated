//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcameracalibrationdata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCameraCalibrationData;
);

unsafe impl NSObjectProtocol for AVCameraCalibrationData {}

extern_methods!(
    unsafe impl AVCameraCalibrationData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(intrinsicMatrixReferenceDimensions)]
        pub unsafe fn intrinsicMatrixReferenceDimensions(&self) -> CGSize;

        #[method(pixelSize)]
        pub unsafe fn pixelSize(&self) -> c_float;

        #[method_id(@__retain_semantics Other lensDistortionLookupTable)]
        pub unsafe fn lensDistortionLookupTable(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other inverseLensDistortionLookupTable)]
        pub unsafe fn inverseLensDistortionLookupTable(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(lensDistortionCenter)]
        pub unsafe fn lensDistortionCenter(&self) -> CGPoint;
    }
);