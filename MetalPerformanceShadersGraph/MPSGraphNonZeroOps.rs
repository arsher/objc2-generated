//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// NonZeroOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        /// Computes the indices of the non-zero elements of the input tensor.
        ///
        /// The indices are returned as a two-dimensional tensor of size `[number_of_nonzeros, input_rank]`.
        /// Each row in the result contains indices of a nonzero elements in input.
        /// For example:
        /// ```md
        /// tensor = [[ 1,  0, 3],
        /// [ 0, 10, 0]]
        /// indices = [[ 0, 0],
        /// [ 0, 2],
        /// [ 1, 1]]
        /// ```
        ///
        /// - Parameters:
        /// - tensor: An MPSGraphTensor of which to compute the non-zero indices.
        /// - Returns: A valid MPSGraphTensor containing indices in signed int32 data type.
        #[method_id(@__retain_semantics Other nonZeroIndicesOfTensor:name:)]
        pub unsafe fn nonZeroIndicesOfTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
