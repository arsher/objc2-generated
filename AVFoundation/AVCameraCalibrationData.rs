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
    /// AVCameraCalibrationData is a model object describing a camera's calibration information.
    ///
    ///
    /// When rendering effects to images produced by cameras, or performing computer vision tasks such as correcting images for geometric distortions, it is necessary to characterize the camera's calibration information, such as its pixel focal length, principal point, lens distortion characteristics, etc. AVCameraCalibrationData provides this information.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcameracalibrationdata?language=objc)
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
        /// The reference frame dimensions used in calculating a camera's principal point.
        ///
        ///
        /// A camera's intrinsic matrix expresses values in pixels with respect to a frame of this width and height.
        #[method(intrinsicMatrixReferenceDimensions)]
        pub unsafe fn intrinsicMatrixReferenceDimensions(&self) -> CGSize;

        /// The size of one pixel at intrinsicMatrixReferenceDimensions in millimeters.
        #[method(pixelSize)]
        pub unsafe fn pixelSize(&self) -> c_float;

        /// An NSData of floats describing the camera lens' radial distortions.
        ///
        ///
        /// Images captured by a camera are geometrically warped by radial distortions in the lens. In order to project from the 2D image plane back into the 3D world, the images must be distortion corrected, or made rectilinear. Lens distortion is modeled using a one-dimensional lookup table of 32-bit float values evenly distributed along a radius from the center of the distortion to the farthest corner, with each value representing an elongation or compression of the radius (0.0 for any given point indicates no elongation). This model assumes radially symmetric lens distortion. When dealing with AVDepthData, the disparity / depth map representations are geometrically distorted to align with images produced by the camera. For more information, see the reference implementation below.
        ///
        /// If the camera lacks the calibration data needed to accurately characterize lens distortions, this property's value is nil.
        #[method_id(@__retain_semantics Other lensDistortionLookupTable)]
        pub unsafe fn lensDistortionLookupTable(&self) -> Option<Retained<NSData>>;

        /// An NSData of floats describing the inverse lookup table required to reapply the camera lens' radial distortions to a rectified image.
        ///
        ///
        /// See lensDistortionLookupTable. If you've rectified an image by removing the distortions characterized by the lensDistortionLookupTable, and now wish to go back to geometrically distorted, you may use the inverseLensDistortionLookupTable. For more information, see the reference implementation below.
        ///
        /// If the camera lacks the calibration data needed to accurately characterize lens distortions, this property's value is nil.
        #[method_id(@__retain_semantics Other inverseLensDistortionLookupTable)]
        pub unsafe fn inverseLensDistortionLookupTable(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// A CGPoint describing the offset of the lens’ distortion center from the top left in intrinsicMatrixReferenceDimensions.
        ///
        ///
        /// Due to geometric distortions in the image, the center of the distortion may not be equal to the optical center (principal point) of the lens. When making an image rectilinear, the distortion center should be used rather than the optical center of the image. For more information, see the reference implementation below.
        ///
        /// If the camera lacks the calibration data needed to accurately characterize lens distortions, this property's value is set to CGPointZero and should not be used.
        #[method(lensDistortionCenter)]
        pub unsafe fn lensDistortionCenter(&self) -> CGPoint;
    }
);
