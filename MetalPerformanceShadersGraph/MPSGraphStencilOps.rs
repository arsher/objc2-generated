//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphstencilopdescriptor?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphStencilOpDescriptor;
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSCopying for MPSGraphStencilOpDescriptor {}

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphStencilOpDescriptor {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSObjectProtocol for MPSGraphStencilOpDescriptor {}

extern_methods!(
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphStencilOpDescriptor {
        #[method(reductionMode)]
        pub unsafe fn reductionMode(&self) -> MPSGraphReductionMode;

        #[method(setReductionMode:)]
        pub unsafe fn setReductionMode(&self, reduction_mode: MPSGraphReductionMode);

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method_id(@__retain_semantics Other offsets)]
        pub unsafe fn offsets(&self) -> Retained<MPSShape>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method(setOffsets:)]
        pub unsafe fn setOffsets(&self, offsets: &MPSShape);

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method_id(@__retain_semantics Other strides)]
        pub unsafe fn strides(&self) -> Retained<MPSShape>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method(setStrides:)]
        pub unsafe fn setStrides(&self, strides: &MPSShape);

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method_id(@__retain_semantics Other dilationRates)]
        pub unsafe fn dilationRates(&self) -> Retained<MPSShape>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method(setDilationRates:)]
        pub unsafe fn setDilationRates(&self, dilation_rates: &MPSShape);

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method_id(@__retain_semantics Other explicitPadding)]
        pub unsafe fn explicitPadding(&self) -> Retained<MPSShape>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method(setExplicitPadding:)]
        pub unsafe fn setExplicitPadding(&self, explicit_padding: &MPSShape);

        #[method(boundaryMode)]
        pub unsafe fn boundaryMode(&self) -> MPSGraphPaddingMode;

        #[method(setBoundaryMode:)]
        pub unsafe fn setBoundaryMode(&self, boundary_mode: MPSGraphPaddingMode);

        #[method(paddingStyle)]
        pub unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle;

        #[method(setPaddingStyle:)]
        pub unsafe fn setPaddingStyle(&self, padding_style: MPSGraphPaddingStyle);

        #[method(paddingConstant)]
        pub unsafe fn paddingConstant(&self) -> c_float;

        #[method(setPaddingConstant:)]
        pub unsafe fn setPaddingConstant(&self, padding_constant: c_float);

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method_id(@__retain_semantics Other descriptorWithReductionMode:offsets:strides:dilationRates:explicitPadding:boundaryMode:paddingStyle:paddingConstant:)]
        pub unsafe fn descriptorWithReductionMode_offsets_strides_dilationRates_explicitPadding_boundaryMode_paddingStyle_paddingConstant(
            reduction_mode: MPSGraphReductionMode,
            offsets: &MPSShape,
            strides: &MPSShape,
            dilation_rates: &MPSShape,
            explicit_padding: &MPSShape,
            boundary_mode: MPSGraphPaddingMode,
            padding_style: MPSGraphPaddingStyle,
            padding_constant: c_float,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method_id(@__retain_semantics Other descriptorWithOffsets:explicitPadding:)]
        pub unsafe fn descriptorWithOffsets_explicitPadding(
            offsets: &MPSShape,
            explicit_padding: &MPSShape,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        #[method_id(@__retain_semantics Other descriptorWithExplicitPadding:)]
        pub unsafe fn descriptorWithExplicitPadding(
            explicit_padding: &MPSShape,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other descriptorWithPaddingStyle:)]
        pub unsafe fn descriptorWithPaddingStyle(
            padding_style: MPSGraphPaddingStyle,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphStencilOpDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// MPSGraphStencilOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other stencilWithSourceTensor:weightsTensor:descriptor:name:)]
        pub unsafe fn stencilWithSourceTensor_weightsTensor_descriptor_name(
            &self,
            source: &MPSGraphTensor,
            weights: &MPSGraphTensor,
            descriptor: &MPSGraphStencilOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
