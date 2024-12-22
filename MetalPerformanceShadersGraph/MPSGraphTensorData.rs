//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

extern_class!(
    /// The representation of a compute data type.
    ///
    /// Pass data to a graph using a tensor data, a reference will be taken to your data and used just in time when the graph is run.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphtensordata?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphTensorData;
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSObjectProtocol for MPSGraphTensorData {}

extern_methods!(
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphTensorData {
        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// The shape of the tensor data.
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Retained<MPSShape>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// The data type of the tensor data.
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MPSDataType;

        #[cfg(feature = "MPSGraphDevice")]
        /// The device of the tensor data.
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Retained<MPSGraphDevice>;

        #[cfg(all(
            feature = "MPSGraphDevice",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Initializes the tensor data with an `NSData` on a device.
        ///
        /// - Parameters:
        /// - device: MPSDevice on which the MPSGraphTensorData exists
        /// - data: NSData from which to copy the contents
        /// - shape: shape of the output tensor
        /// - dataType: dataType of the placeholder tensor
        /// - Returns: A valid MPSGraphTensorData, or nil if allocation failure.
        #[method_id(@__retain_semantics Init initWithDevice:data:shape:dataType:)]
        pub unsafe fn initWithDevice_data_shape_dataType(
            this: Allocated<Self>,
            device: &MPSGraphDevice,
            data: &NSData,
            shape: &MPSShape,
            data_type: MPSDataType,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Initializes an tensor data with a metal buffer.
        ///
        /// The device of the MTLBuffer will be used to get the MPSDevice for this MPSGraphTensorData.
        ///
        /// - Parameters:
        /// - buffer: MTLBuffer to be used within the MPSGraphTensorData
        /// - shape: shape of the output tensor
        /// - dataType: dataType of the placeholder tensor
        /// - Returns: A valid MPSGraphTensorData, or nil if allocation failure.
        #[method_id(@__retain_semantics Init initWithMTLBuffer:shape:dataType:)]
        pub unsafe fn initWithMTLBuffer_shape_dataType(
            this: Allocated<Self>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            shape: &MPSShape,
            data_type: MPSDataType,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Initializes an tensor data with a metal buffer.
        ///
        /// The device of the MTLBuffer will be used to get the MPSDevice for this MPSGraphTensorData.
        ///
        /// - Parameters:
        /// - buffer: MTLBuffer to be used within the MPSGraphTensorData
        /// - shape: shape of the output tensor
        /// - dataType: dataType of the placeholder tensor
        /// - rowBytes: rowBytes for the fastest moving dimension, must be larger than or equal to sizeOf(dataType)shape[rank - 1] and must be a multiple of sizeOf(dataType)
        /// - Returns: A valid MPSGraphTensorData, or nil if allocation failure.
        #[method_id(@__retain_semantics Init initWithMTLBuffer:shape:dataType:rowBytes:)]
        pub unsafe fn initWithMTLBuffer_shape_dataType_rowBytes(
            this: Allocated<Self>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            shape: &MPSShape,
            data_type: MPSDataType,
            row_bytes: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Initializes a tensor data with an MPS matrix.
        ///
        /// The device of the MPSMatrix will be used to get the MPSDevice for this MPSGraphTensorData.
        ///
        /// - Parameters:
        /// - matrix: MPSMatrix to be used within the MPSGraphTensorData
        /// - Returns: A valid MPSGraphTensorData, or nil if allocation failure.
        #[method_id(@__retain_semantics Init initWithMPSMatrix:)]
        pub unsafe fn initWithMPSMatrix(
            this: Allocated<Self>,
            matrix: &MPSMatrix,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Initializes a tensor data with an MPS matrix enforcing rank of the result.
        ///
        /// The device of the MPSMatrix will be used to get the MPSDevice for this MPSGraphTensorData.
        ///
        /// - Parameters:
        /// - matrix: MPSMatrix to be used within the MPSGraphTensorData
        /// - rank: The rank of the resulting TensorData tensor. NOTE: must be within { 1, ... ,16 }.
        /// - Returns: A valid MPSGraphTensorData of given rank, or nil if allocation failure.
        #[method_id(@__retain_semantics Init initWithMPSMatrix:rank:)]
        pub unsafe fn initWithMPSMatrix_rank(
            this: Allocated<Self>,
            matrix: &MPSMatrix,
            rank: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Initializes a tensor data with an MPS vector.
        ///
        /// The device of the MPSVector will be used to get the MPSDevice for this MPSGraphTensorData.
        ///
        /// - Parameters:
        /// - vector: MPSVector to be used within the MPSGraphTensorData
        /// - Returns: A valid MPSGraphTensorData, or nil if allocation failure.
        #[method_id(@__retain_semantics Init initWithMPSVector:)]
        pub unsafe fn initWithMPSVector(
            this: Allocated<Self>,
            vector: &MPSVector,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Initializes a tensor data with an MPS vector enforcing rank of the result.
        ///
        /// The device of the MPSVector will be used to get the MPSDevice for this MPSGraphTensorData.
        ///
        /// - Parameters:
        /// - vector: MPSVector to be used within the MPSGraphTensorData
        /// - rank: The rank of the resulting TensorData tensor. NOTE: must be within { 1, ... ,16 }.
        /// - Returns: A valid MPSGraphTensorData, or nil if allocation failure.
        #[method_id(@__retain_semantics Init initWithMPSVector:rank:)]
        pub unsafe fn initWithMPSVector_rank(
            this: Allocated<Self>,
            vector: &MPSVector,
            rank: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Initializes an MPSGraphTensorData with an MPS ndarray.
        ///
        /// The device of the MPSNDArray will be used to get the MPSDevice for this MPSGraphTensorData.
        ///
        /// - Parameters:
        /// - ndarray: MPSNDArray to be used within the MPSGraphTensorData.
        /// - Returns: A valid MPSGraphTensorData, or nil if allocation failure.
        #[method_id(@__retain_semantics Init initWithMPSNDArray:)]
        pub unsafe fn initWithMPSNDArray(
            this: Allocated<Self>,
            ndarray: &MPSNDArray,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Initializes a tensor data with an MPS image batch.
        ///
        /// The dataLayout used will be NHWC, call a transpose or permute to change to a layout of your choice.
        ///
        /// - Parameters:
        /// - imageBatch: The device on which the kernel will run, unorm8 and unorm16 images will create a float32 tensorData
        /// - Returns: A valid MPSGraphTensorData, or nil if allocation failure.
        #[method_id(@__retain_semantics Init initWithMPSImageBatch:)]
        pub unsafe fn initWithMPSImageBatch(
            this: Allocated<Self>,
            image_batch: &MPSImageBatch,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Return an mpsndarray object will copy contents if the contents are not stored in an MPS ndarray.
        ///
        /// - Returns: A valid MPSNDArray, or nil if allocation fails.
        #[method_id(@__retain_semantics Other mpsndarray)]
        pub unsafe fn mpsndarray(&self) -> Retained<MPSNDArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphTensorData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
