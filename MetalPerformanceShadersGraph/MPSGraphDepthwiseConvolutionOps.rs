//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphdepthwiseconvolution2dopdescriptor?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphDepthwiseConvolution2DOpDescriptor;
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSCopying for MPSGraphDepthwiseConvolution2DOpDescriptor {}

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphDepthwiseConvolution2DOpDescriptor {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSObjectProtocol for MPSGraphDepthwiseConvolution2DOpDescriptor {}

extern_methods!(
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphDepthwiseConvolution2DOpDescriptor {
        #[method(strideInX)]
        pub unsafe fn strideInX(&self) -> NSUInteger;

        #[method(setStrideInX:)]
        pub unsafe fn setStrideInX(&self, stride_in_x: NSUInteger);

        #[method(strideInY)]
        pub unsafe fn strideInY(&self) -> NSUInteger;

        #[method(setStrideInY:)]
        pub unsafe fn setStrideInY(&self, stride_in_y: NSUInteger);

        #[method(dilationRateInX)]
        pub unsafe fn dilationRateInX(&self) -> NSUInteger;

        #[method(setDilationRateInX:)]
        pub unsafe fn setDilationRateInX(&self, dilation_rate_in_x: NSUInteger);

        #[method(dilationRateInY)]
        pub unsafe fn dilationRateInY(&self) -> NSUInteger;

        #[method(setDilationRateInY:)]
        pub unsafe fn setDilationRateInY(&self, dilation_rate_in_y: NSUInteger);

        #[method(paddingLeft)]
        pub unsafe fn paddingLeft(&self) -> NSUInteger;

        #[method(setPaddingLeft:)]
        pub unsafe fn setPaddingLeft(&self, padding_left: NSUInteger);

        #[method(paddingRight)]
        pub unsafe fn paddingRight(&self) -> NSUInteger;

        #[method(setPaddingRight:)]
        pub unsafe fn setPaddingRight(&self, padding_right: NSUInteger);

        #[method(paddingTop)]
        pub unsafe fn paddingTop(&self) -> NSUInteger;

        #[method(setPaddingTop:)]
        pub unsafe fn setPaddingTop(&self, padding_top: NSUInteger);

        #[method(paddingBottom)]
        pub unsafe fn paddingBottom(&self) -> NSUInteger;

        #[method(setPaddingBottom:)]
        pub unsafe fn setPaddingBottom(&self, padding_bottom: NSUInteger);

        #[method(paddingStyle)]
        pub unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle;

        #[method(setPaddingStyle:)]
        pub unsafe fn setPaddingStyle(&self, padding_style: MPSGraphPaddingStyle);

        #[method(dataLayout)]
        pub unsafe fn dataLayout(&self) -> MPSGraphTensorNamedDataLayout;

        #[method(setDataLayout:)]
        pub unsafe fn setDataLayout(&self, data_layout: MPSGraphTensorNamedDataLayout);

        #[method(weightsLayout)]
        pub unsafe fn weightsLayout(&self) -> MPSGraphTensorNamedDataLayout;

        #[method(setWeightsLayout:)]
        pub unsafe fn setWeightsLayout(&self, weights_layout: MPSGraphTensorNamedDataLayout);

        #[method_id(@__retain_semantics Other descriptorWithStrideInX:strideInY:dilationRateInX:dilationRateInY:paddingLeft:paddingRight:paddingTop:paddingBottom:paddingStyle:dataLayout:weightsLayout:)]
        pub unsafe fn descriptorWithStrideInX_strideInY_dilationRateInX_dilationRateInY_paddingLeft_paddingRight_paddingTop_paddingBottom_paddingStyle_dataLayout_weightsLayout(
            stride_in_x: NSUInteger,
            stride_in_y: NSUInteger,
            dilation_rate_in_x: NSUInteger,
            dilation_rate_in_y: NSUInteger,
            padding_left: NSUInteger,
            padding_right: NSUInteger,
            padding_top: NSUInteger,
            padding_bottom: NSUInteger,
            padding_style: MPSGraphPaddingStyle,
            data_layout: MPSGraphTensorNamedDataLayout,
            weights_layout: MPSGraphTensorNamedDataLayout,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other descriptorWithDataLayout:weightsLayout:)]
        pub unsafe fn descriptorWithDataLayout_weightsLayout(
            data_layout: MPSGraphTensorNamedDataLayout,
            weights_layout: MPSGraphTensorNamedDataLayout,
        ) -> Option<Retained<Self>>;

        #[method(setExplicitPaddingWithPaddingLeft:paddingRight:paddingTop:paddingBottom:)]
        pub unsafe fn setExplicitPaddingWithPaddingLeft_paddingRight_paddingTop_paddingBottom(
            &self,
            padding_left: NSUInteger,
            padding_right: NSUInteger,
            padding_top: NSUInteger,
            padding_bottom: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphDepthwiseConvolution2DOpDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphdepthwiseconvolution3dopdescriptor?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphDepthwiseConvolution3DOpDescriptor;
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSCopying for MPSGraphDepthwiseConvolution3DOpDescriptor {}

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphDepthwiseConvolution3DOpDescriptor {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSObjectProtocol for MPSGraphDepthwiseConvolution3DOpDescriptor {}

extern_methods!(
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphDepthwiseConvolution3DOpDescriptor {
        #[method_id(@__retain_semantics Other strides)]
        pub unsafe fn strides(&self) -> Retained<NSArray<NSNumber>>;

        #[method(setStrides:)]
        pub unsafe fn setStrides(&self, strides: &NSArray<NSNumber>);

        #[method_id(@__retain_semantics Other dilationRates)]
        pub unsafe fn dilationRates(&self) -> Retained<NSArray<NSNumber>>;

        #[method(setDilationRates:)]
        pub unsafe fn setDilationRates(&self, dilation_rates: &NSArray<NSNumber>);

        #[method_id(@__retain_semantics Other paddingValues)]
        pub unsafe fn paddingValues(&self) -> Retained<NSArray<NSNumber>>;

        #[method(setPaddingValues:)]
        pub unsafe fn setPaddingValues(&self, padding_values: &NSArray<NSNumber>);

        #[method(paddingStyle)]
        pub unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle;

        #[method(setPaddingStyle:)]
        pub unsafe fn setPaddingStyle(&self, padding_style: MPSGraphPaddingStyle);

        #[method(channelDimensionIndex)]
        pub unsafe fn channelDimensionIndex(&self) -> NSInteger;

        #[method(setChannelDimensionIndex:)]
        pub unsafe fn setChannelDimensionIndex(&self, channel_dimension_index: NSInteger);

        #[method_id(@__retain_semantics Other descriptorWithStrides:dilationRates:paddingValues:paddingStyle:)]
        pub unsafe fn descriptorWithStrides_dilationRates_paddingValues_paddingStyle(
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_values: &NSArray<NSNumber>,
            padding_style: MPSGraphPaddingStyle,
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
    unsafe impl MPSGraphDepthwiseConvolution3DOpDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// MPSGraphDepthwiseConvolutionOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other depthwiseConvolution2DWithSourceTensor:weightsTensor:descriptor:name:)]
        pub unsafe fn depthwiseConvolution2DWithSourceTensor_weightsTensor_descriptor_name(
            &self,
            source: &MPSGraphTensor,
            weights: &MPSGraphTensor,
            descriptor: &MPSGraphDepthwiseConvolution2DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other depthwiseConvolution2DDataGradientWithIncomingGradientTensor:weightsTensor:outputShape:descriptor:name:)]
        pub unsafe fn depthwiseConvolution2DDataGradientWithIncomingGradientTensor_weightsTensor_outputShape_descriptor_name(
            &self,
            incoming_gradient: &MPSGraphTensor,
            weights: &MPSGraphTensor,
            output_shape: &MPSShape,
            descriptor: &MPSGraphDepthwiseConvolution2DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other depthwiseConvolution2DWeightsGradientWithIncomingGradientTensor:sourceTensor:outputShape:descriptor:name:)]
        pub unsafe fn depthwiseConvolution2DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShape_descriptor_name(
            &self,
            incoming_gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            output_shape: &MPSShape,
            descriptor: &MPSGraphDepthwiseConvolution2DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other depthwiseConvolution3DWithSourceTensor:weightsTensor:descriptor:name:)]
        pub unsafe fn depthwiseConvolution3DWithSourceTensor_weightsTensor_descriptor_name(
            &self,
            source: &MPSGraphTensor,
            weights: &MPSGraphTensor,
            descriptor: &MPSGraphDepthwiseConvolution3DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other depthwiseConvolution3DDataGradientWithIncomingGradientTensor:weightsTensor:outputShape:descriptor:name:)]
        pub unsafe fn depthwiseConvolution3DDataGradientWithIncomingGradientTensor_weightsTensor_outputShape_descriptor_name(
            &self,
            incoming_gradient: &MPSGraphTensor,
            weights: &MPSGraphTensor,
            output_shape: Option<&MPSShape>,
            descriptor: &MPSGraphDepthwiseConvolution3DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other depthwiseConvolution3DWeightsGradientWithIncomingGradientTensor:sourceTensor:outputShape:descriptor:name:)]
        pub unsafe fn depthwiseConvolution3DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShape_descriptor_name(
            &self,
            incoming_gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            output_shape: &MPSShape,
            descriptor: &MPSGraphDepthwiseConvolution3DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
