//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphresizemode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSGraphResizeMode(pub NSUInteger);
impl MPSGraphResizeMode {
    pub const MPSGraphResizeNearest: Self = Self(0);
    pub const MPSGraphResizeBilinear: Self = Self(1);
}

unsafe impl Encode for MPSGraphResizeMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSGraphResizeMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphresizenearestroundingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSGraphResizeNearestRoundingMode(pub NSUInteger);
impl MPSGraphResizeNearestRoundingMode {
    #[doc(alias = "MPSGraphResizeNearestRoundingModeRoundPreferCeil")]
    pub const RoundPreferCeil: Self = Self(0);
    #[doc(alias = "MPSGraphResizeNearestRoundingModeRoundPreferFloor")]
    pub const RoundPreferFloor: Self = Self(1);
    #[doc(alias = "MPSGraphResizeNearestRoundingModeCeil")]
    pub const Ceil: Self = Self(2);
    #[doc(alias = "MPSGraphResizeNearestRoundingModeFloor")]
    pub const Floor: Self = Self(3);
    #[doc(alias = "MPSGraphResizeNearestRoundingModeRoundToEven")]
    pub const RoundToEven: Self = Self(4);
    #[doc(alias = "MPSGraphResizeNearestRoundingModeRoundToOdd")]
    pub const RoundToOdd: Self = Self(5);
}

unsafe impl Encode for MPSGraphResizeNearestRoundingMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSGraphResizeNearestRoundingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// MPSGraphResizeOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other resizeTensor:size:mode:centerResult:alignCorners:layout:name:)]
        pub unsafe fn resizeTensor_size_mode_centerResult_alignCorners_layout_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSShape,
            mode: MPSGraphResizeMode,
            center_result: bool,
            align_corners: bool,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeTensor:sizeTensor:mode:centerResult:alignCorners:layout:name:)]
        pub unsafe fn resizeTensor_sizeTensor_mode_centerResult_alignCorners_layout_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            mode: MPSGraphResizeMode,
            center_result: bool,
            align_corners: bool,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeTensor:sizeTensor:mode:centerResult:alignCorners:name:)]
        pub unsafe fn resizeTensor_sizeTensor_mode_centerResult_alignCorners_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            mode: MPSGraphResizeMode,
            center_result: bool,
            align_corners: bool,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeNearestWithTensor:sizeTensor:nearestRoundingMode:centerResult:alignCorners:layout:name:)]
        pub unsafe fn resizeNearestWithTensor_sizeTensor_nearestRoundingMode_centerResult_alignCorners_layout_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            nearest_rounding_mode: MPSGraphResizeNearestRoundingMode,
            center_result: bool,
            align_corners: bool,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeNearestWithTensor:sizeTensor:nearestRoundingMode:centerResult:alignCorners:name:)]
        pub unsafe fn resizeNearestWithTensor_sizeTensor_nearestRoundingMode_centerResult_alignCorners_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            nearest_rounding_mode: MPSGraphResizeNearestRoundingMode,
            center_result: bool,
            align_corners: bool,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeBilinearWithTensor:sizeTensor:centerResult:alignCorners:layout:name:)]
        pub unsafe fn resizeBilinearWithTensor_sizeTensor_centerResult_alignCorners_layout_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            center_result: bool,
            align_corners: bool,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeBilinearWithTensor:sizeTensor:centerResult:alignCorners:name:)]
        pub unsafe fn resizeBilinearWithTensor_sizeTensor_centerResult_alignCorners_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            center_result: bool,
            align_corners: bool,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeTensor:sizeTensor:scaleOffsetTensor:mode:layout:name:)]
        pub unsafe fn resizeTensor_sizeTensor_scaleOffsetTensor_mode_layout_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            scale_offset: &MPSGraphTensor,
            mode: MPSGraphResizeMode,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeTensor:sizeTensor:scaleTensor:offsetTensor:mode:name:)]
        pub unsafe fn resizeTensor_sizeTensor_scaleTensor_offsetTensor_mode_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            scale: &MPSGraphTensor,
            offset: &MPSGraphTensor,
            mode: MPSGraphResizeMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeNearestWithTensor:sizeTensor:scaleOffsetTensor:nearestRoundingMode:layout:name:)]
        pub unsafe fn resizeNearestWithTensor_sizeTensor_scaleOffsetTensor_nearestRoundingMode_layout_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            scale_offset: &MPSGraphTensor,
            nearest_rounding_mode: MPSGraphResizeNearestRoundingMode,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeNearestWithTensor:sizeTensor:scaleTensor:offsetTensor:nearestRoundingMode:name:)]
        pub unsafe fn resizeNearestWithTensor_sizeTensor_scaleTensor_offsetTensor_nearestRoundingMode_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            scale: &MPSGraphTensor,
            offset: &MPSGraphTensor,
            nearest_rounding_mode: MPSGraphResizeNearestRoundingMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeBilinearWithTensor:sizeTensor:scaleOffsetTensor:layout:name:)]
        pub unsafe fn resizeBilinearWithTensor_sizeTensor_scaleOffsetTensor_layout_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            scale_offset: &MPSGraphTensor,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeBilinearWithTensor:sizeTensor:scaleTensor:offsetTensor:name:)]
        pub unsafe fn resizeBilinearWithTensor_sizeTensor_scaleTensor_offsetTensor_name(
            &self,
            images_tensor: &MPSGraphTensor,
            size: &MPSGraphTensor,
            scale: &MPSGraphTensor,
            offset: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeWithGradientTensor:input:mode:centerResult:alignCorners:layout:name:)]
        pub unsafe fn resizeWithGradientTensor_input_mode_centerResult_alignCorners_layout_name(
            &self,
            gradient: &MPSGraphTensor,
            input: &MPSGraphTensor,
            mode: MPSGraphResizeMode,
            center_result: bool,
            align_corners: bool,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeNearestWithGradientTensor:input:nearestRoundingMode:centerResult:alignCorners:layout:name:)]
        pub unsafe fn resizeNearestWithGradientTensor_input_nearestRoundingMode_centerResult_alignCorners_layout_name(
            &self,
            gradient: &MPSGraphTensor,
            input: &MPSGraphTensor,
            nearest_rounding_mode: MPSGraphResizeNearestRoundingMode,
            center_result: bool,
            align_corners: bool,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeBilinearWithGradientTensor:input:centerResult:alignCorners:layout:name:)]
        pub unsafe fn resizeBilinearWithGradientTensor_input_centerResult_alignCorners_layout_name(
            &self,
            gradient: &MPSGraphTensor,
            input: &MPSGraphTensor,
            center_result: bool,
            align_corners: bool,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeWithGradientTensor:input:scaleOffsetTensor:mode:layout:name:)]
        pub unsafe fn resizeWithGradientTensor_input_scaleOffsetTensor_mode_layout_name(
            &self,
            gradient: &MPSGraphTensor,
            input: &MPSGraphTensor,
            scale_offset: &MPSGraphTensor,
            mode: MPSGraphResizeMode,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeWithGradientTensor:input:scaleTensor:offsetTensor:mode:name:)]
        pub unsafe fn resizeWithGradientTensor_input_scaleTensor_offsetTensor_mode_name(
            &self,
            gradient: &MPSGraphTensor,
            input: &MPSGraphTensor,
            scale: &MPSGraphTensor,
            offset: &MPSGraphTensor,
            mode: MPSGraphResizeMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeNearestWithGradientTensor:input:scaleOffsetTensor:nearestRoundingMode:layout:name:)]
        pub unsafe fn resizeNearestWithGradientTensor_input_scaleOffsetTensor_nearestRoundingMode_layout_name(
            &self,
            gradient: &MPSGraphTensor,
            input: &MPSGraphTensor,
            scale_offset: &MPSGraphTensor,
            nearest_rounding_mode: MPSGraphResizeNearestRoundingMode,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeNearestWithGradientTensor:input:scaleTensor:offsetTensor:nearestRoundingMode:name:)]
        pub unsafe fn resizeNearestWithGradientTensor_input_scaleTensor_offsetTensor_nearestRoundingMode_name(
            &self,
            gradient: &MPSGraphTensor,
            input: &MPSGraphTensor,
            scale: &MPSGraphTensor,
            offset: &MPSGraphTensor,
            nearest_rounding_mode: MPSGraphResizeNearestRoundingMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeBilinearWithGradientTensor:input:scaleOffsetTensor:layout:name:)]
        pub unsafe fn resizeBilinearWithGradientTensor_input_scaleOffsetTensor_layout_name(
            &self,
            gradient: &MPSGraphTensor,
            input: &MPSGraphTensor,
            scale_offset: &MPSGraphTensor,
            layout: MPSGraphTensorNamedDataLayout,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other resizeBilinearWithGradientTensor:input:scaleTensor:offsetTensor:name:)]
        pub unsafe fn resizeBilinearWithGradientTensor_input_scaleTensor_offsetTensor_name(
            &self,
            gradient: &MPSGraphTensor,
            input: &MPSGraphTensor,
            scale: &MPSGraphTensor,
            offset: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
