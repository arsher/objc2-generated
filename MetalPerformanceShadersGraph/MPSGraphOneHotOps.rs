//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

extern_methods!(
    /// MPSGraphOneHotOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a oneHot operation and returns the result tensor.
        ///
        /// Creates a tensor of rank equal to the indicesTensor rank + 1.
        /// Inserts a new axis at the axis specified, or the minor axis if axis is -1.
        /// The values at the indices in the indicesTensor will have the onValue,
        /// and all other values will be set to the offValue.
        ///
        /// - Parameters:
        /// - indicesTensor: Tensor of indices for on values
        /// - depth: Depth of the oneHot vector along the axis
        /// - axis: The axis to insert the new oneHot vector at. Defaults to -1, the minor axis
        /// - dataType: MPSDataType of the result tensor Defaults to MPSDataTypeFloat
        /// - onValue: The value for indices designated by the indicesTensor. This value must match the specified data type. Defaults to 1.0f
        /// - offValue: The value for indices not designated by the indicesTensor. This value must match the specified data type. Defaults to 0.0f
        /// - name: Name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[method_id(@__retain_semantics Other oneHotWithIndicesTensor:depth:axis:dataType:onValue:offValue:name:)]
        pub unsafe fn oneHotWithIndicesTensor_depth_axis_dataType_onValue_offValue_name(
            &self,
            indices_tensor: &MPSGraphTensor,
            depth: NSUInteger,
            axis: NSUInteger,
            data_type: MPSDataType,
            on_value: c_double,
            off_value: c_double,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a oneHot operation and returns the result tensor.
        ///
        /// Creates a tensor of rank equal to the rank of `indicesTensor` + 1.
        /// Inserts a new axis at the minor dimension.
        /// The values at the indices in the indicesTensor will have the onValue,
        /// and all other values will be set to the offValue.
        ///
        /// - Parameters:
        /// - indicesTensor: Tensor of indices for on values
        /// - depth: Depth of the oneHot vector along the axis
        /// - dataType: MPSDataType of the result tensor.
        /// - onValue: The value for indices designated by the indicesTensor. This value must match the specified data type.
        /// - offValue: The value for indices not designated by the indicesTensor. This value must match the specified data type.
        /// - name: Name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[method_id(@__retain_semantics Other oneHotWithIndicesTensor:depth:dataType:onValue:offValue:name:)]
        pub unsafe fn oneHotWithIndicesTensor_depth_dataType_onValue_offValue_name(
            &self,
            indices_tensor: &MPSGraphTensor,
            depth: NSUInteger,
            data_type: MPSDataType,
            on_value: c_double,
            off_value: c_double,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a oneHot operation and returns the result tensor.
        ///
        /// Creates a tensor of rank equal to the rank of `indicesTensor` + 1.
        /// Inserts a new axis at the axis specified, or the minor axis if `axis` is -1.
        /// The values at the indices in the indicesTensor will be set to 1,
        /// and all other values will be set to 0.
        ///
        /// - Parameters:
        /// - indicesTensor: Tensor of indices for on values
        /// - depth: Depth of the oneHot vector along the axis
        /// - axis: The axis to insert the new oneHot vector at
        /// - dataType: MPSDataType of the result tensor.
        /// - name: Name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[method_id(@__retain_semantics Other oneHotWithIndicesTensor:depth:axis:dataType:name:)]
        pub unsafe fn oneHotWithIndicesTensor_depth_axis_dataType_name(
            &self,
            indices_tensor: &MPSGraphTensor,
            depth: NSUInteger,
            axis: NSUInteger,
            data_type: MPSDataType,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a oneHot operation and returns the result tensor.
        ///
        /// Creates a tensor of rank equal to the rank of `indicesTensor` + 1, of type MPSDataTypeFloat32.
        /// Inserts a new axis at the axis specified, or the minor axis if `axis` is -1.
        /// The values at the indices in the indicesTensor will be set to 1,
        /// and all other values will be set to 0.
        ///
        /// - Parameters:
        /// - indicesTensor: Tensor of indices for on values
        /// - depth: Depth of the oneHot vector along the axis
        /// - axis: The axis to insert the new oneHot vector at
        /// - name: Name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[method_id(@__retain_semantics Other oneHotWithIndicesTensor:depth:axis:name:)]
        pub unsafe fn oneHotWithIndicesTensor_depth_axis_name(
            &self,
            indices_tensor: &MPSGraphTensor,
            depth: NSUInteger,
            axis: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a oneHot operation and returns the result tensor.
        ///
        /// Creates a tensor of rank equal to the rank of `indicesTensor` + 1.
        /// Inserts a new axis at the minor dimension.
        /// The values at the indices in the indicesTensor will be set to 1,
        /// and all other values will be set to 0.
        ///
        /// - Parameters:
        /// - indicesTensor: Tensor of indices for on values
        /// - depth: Depth of the oneHot vector along the axis
        /// - dataType: MPSDataType of the result tensor.
        /// - name: Name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[method_id(@__retain_semantics Other oneHotWithIndicesTensor:depth:dataType:name:)]
        pub unsafe fn oneHotWithIndicesTensor_depth_dataType_name(
            &self,
            indices_tensor: &MPSGraphTensor,
            depth: NSUInteger,
            data_type: MPSDataType,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a oneHot operation and returns the result tensor.
        ///
        /// Creates a tensor of rank equal to the rank of `indicesTensor` + 1, of type MPSDataTypeFloat32.
        /// Inserts a new axis at the minor dimension.
        /// The values at the indices in the indicesTensor will be set to 1,
        /// and all other values will be set to 0.
        ///
        /// - Parameters:
        /// - indicesTensor: Tensor of indices for on values
        /// - depth: Depth of the oneHot vector along the axis
        /// - name: Name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[method_id(@__retain_semantics Other oneHotWithIndicesTensor:depth:name:)]
        pub unsafe fn oneHotWithIndicesTensor_depth_name(
            &self,
            indices_tensor: &MPSGraphTensor,
            depth: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
