//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// MPSGraphActivationOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reLUWithTensor:name:)]
        pub unsafe fn reLUWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reLUGradientWithIncomingGradient:sourceTensor:name:)]
        pub unsafe fn reLUGradientWithIncomingGradient_sourceTensor_name(
            &self,
            gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other sigmoidWithTensor:name:)]
        pub unsafe fn sigmoidWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other sigmoidGradientWithIncomingGradient:sourceTensor:name:)]
        pub unsafe fn sigmoidGradientWithIncomingGradient_sourceTensor_name(
            &self,
            gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other softMaxWithTensor:axis:name:)]
        pub unsafe fn softMaxWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other softMaxGradientWithIncomingGradient:sourceTensor:axis:name:)]
        pub unsafe fn softMaxGradientWithIncomingGradient_sourceTensor_axis_name(
            &self,
            gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other leakyReLUWithTensor:alpha:name:)]
        pub unsafe fn leakyReLUWithTensor_alpha_name(
            &self,
            tensor: &MPSGraphTensor,
            alpha: c_double,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other leakyReLUWithTensor:alphaTensor:name:)]
        pub unsafe fn leakyReLUWithTensor_alphaTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            alpha_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other leakyReLUGradientWithIncomingGradient:sourceTensor:alphaTensor:name:)]
        pub unsafe fn leakyReLUGradientWithIncomingGradient_sourceTensor_alphaTensor_name(
            &self,
            gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            alpha_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
