//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// MPSGraphActivationOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        /// Computes the ReLU (rectified linear activation unit) function with the input tensor.
        ///
        /// The operation is:  f(x) = max(x, 0).
        ///
        /// - Parameters:
        /// - tensor: The input tensor.
        /// - name: The name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object.
        #[method_id(@__retain_semantics Other reLUWithTensor:name:)]
        pub unsafe fn reLUWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Computes the gradient of the ReLU  (rectified linear activation unit) function using the incoming gradient.
        ///
        /// - Parameters:
        /// - gradient: The incoming gradient tensor.
        /// - source: The input tensor from forward pass.
        /// - name: The name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object.
        #[method_id(@__retain_semantics Other reLUGradientWithIncomingGradient:sourceTensor:name:)]
        pub unsafe fn reLUGradientWithIncomingGradient_sourceTensor_name(
            &self,
            gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Computes the sigmoid operation on an input tensor.
        ///
        /// - Parameters:
        /// - tensor: The input tensor.
        /// - name: The name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object.
        #[method_id(@__retain_semantics Other sigmoidWithTensor:name:)]
        pub unsafe fn sigmoidWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Computes the gradient of the sigmoid function using the incoming gradient tensor.
        ///
        /// - Parameters:
        /// - gradient: The incoming gradient tensor.
        /// - source: The input tensor.
        /// - name: The name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object
        #[method_id(@__retain_semantics Other sigmoidGradientWithIncomingGradient:sourceTensor:name:)]
        pub unsafe fn sigmoidGradientWithIncomingGradient_sourceTensor_name(
            &self,
            gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Computes the softmax function on the input tensor along the specified axis.
        ///
        /// - Parameters:
        /// - tensor: The input tensor.
        /// - axis: The axis along which softmax is computed.
        /// - name: The name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object
        #[method_id(@__retain_semantics Other softMaxWithTensor:axis:name:)]
        pub unsafe fn softMaxWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Computes the gradient of the softmax function along the specified axis using the incoming gradient tensor.
        ///
        /// - Parameters:
        /// - gradient: The incoming gradient tensor.
        /// - source: The input tensor.
        /// - axis: The axis along which softmax is computed.
        /// - name: The name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object
        #[method_id(@__retain_semantics Other softMaxGradientWithIncomingGradient:sourceTensor:axis:name:)]
        pub unsafe fn softMaxGradientWithIncomingGradient_sourceTensor_axis_name(
            &self,
            gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Computes the leaky rectified linear unit (ReLU) activation function on the input tensor.
        ///
        /// The operation is: f(x) = max(x, alpha).
        ///
        /// - Parameters:
        /// - tensor: An input tensor.
        /// - alpha: The scalar value alpha used by all elements in the input tensor.
        /// - name: The name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object
        #[method_id(@__retain_semantics Other leakyReLUWithTensor:alpha:name:)]
        pub unsafe fn leakyReLUWithTensor_alpha_name(
            &self,
            tensor: &MPSGraphTensor,
            alpha: c_double,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Computes the leaky rectified linear unit (ReLU) activation function on the input tensor.
        ///
        /// The operation is: f(x) = max(x, alpha).
        /// This operation supports broadcasting with the alpha tensor.
        ///
        /// - Parameters:
        /// - tensor: The input tensor.
        /// - alpha: The alpha tensor.
        /// - name: The name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object
        #[method_id(@__retain_semantics Other leakyReLUWithTensor:alphaTensor:name:)]
        pub unsafe fn leakyReLUWithTensor_alphaTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            alpha_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Computes the gradient of the leaky rectified linear unit (ReLU) activation.
        ///
        /// This operation supports broadcasting with the alpha tensor.
        ///
        /// - Parameters:
        /// - gradient: The incoming gradient tensor.
        /// - source: The input tensor in forward pass.
        /// - alpha: The alpha tensor
        /// - name: The name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object
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
