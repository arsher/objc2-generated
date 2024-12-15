//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// MPSGraphSampleGrid
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(all(feature = "MPSGraphResizeOps", feature = "MPSGraphTensor"))]
        #[method_id(@__retain_semantics Other sampleGridWithSourceTensor:coordinateTensor:layout:normalizeCoordinates:relativeCoordinates:alignCorners:paddingMode:samplingMode:constantValue:name:)]
        pub unsafe fn sampleGridWithSourceTensor_coordinateTensor_layout_normalizeCoordinates_relativeCoordinates_alignCorners_paddingMode_samplingMode_constantValue_name(
            &self,
            source: &MPSGraphTensor,
            coordinates: &MPSGraphTensor,
            layout: MPSGraphTensorNamedDataLayout,
            normalize_coordinates: bool,
            relative_coordinates: bool,
            align_corners: bool,
            padding_mode: MPSGraphPaddingMode,
            sampling_mode: MPSGraphResizeMode,
            constant_value: c_double,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(feature = "MPSGraphResizeOps", feature = "MPSGraphTensor"))]
        #[method_id(@__retain_semantics Other sampleGridWithSourceTensor:coordinateTensor:layout:normalizeCoordinates:relativeCoordinates:alignCorners:paddingMode:nearestRoundingMode:constantValue:name:)]
        pub unsafe fn sampleGridWithSourceTensor_coordinateTensor_layout_normalizeCoordinates_relativeCoordinates_alignCorners_paddingMode_nearestRoundingMode_constantValue_name(
            &self,
            source: &MPSGraphTensor,
            coordinates: &MPSGraphTensor,
            layout: MPSGraphTensorNamedDataLayout,
            normalize_coordinates: bool,
            relative_coordinates: bool,
            align_corners: bool,
            padding_mode: MPSGraphPaddingMode,
            nearest_rounding_mode: MPSGraphResizeNearestRoundingMode,
            constant_value: c_double,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);