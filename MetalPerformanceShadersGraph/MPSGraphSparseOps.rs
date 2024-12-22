//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

/// The sparse storage options in the Metal Performance Shaders Graph framework.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphsparsestoragetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSGraphSparseStorageType(pub u64);
impl MPSGraphSparseStorageType {
    /// COO Storage
    pub const MPSGraphSparseStorageCOO: Self = Self(0);
    /// CSC Storage
    pub const MPSGraphSparseStorageCSC: Self = Self(1);
    /// CSR Storage
    pub const MPSGraphSparseStorageCSR: Self = Self(2);
}

unsafe impl Encode for MPSGraphSparseStorageType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MPSGraphSparseStorageType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A class that describes the properties of a create sparse operation.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphcreatesparseopdescriptor?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphCreateSparseOpDescriptor;
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSCopying for MPSGraphCreateSparseOpDescriptor {}

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphCreateSparseOpDescriptor {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSObjectProtocol for MPSGraphCreateSparseOpDescriptor {}

extern_methods!(
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphCreateSparseOpDescriptor {
        /// Defines the storage format of the sparse tensor.
        #[method(sparseStorageType)]
        pub unsafe fn sparseStorageType(&self) -> MPSGraphSparseStorageType;

        /// Setter for [`sparseStorageType`][Self::sparseStorageType].
        #[method(setSparseStorageType:)]
        pub unsafe fn setSparseStorageType(&self, sparse_storage_type: MPSGraphSparseStorageType);

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Defines the datatype of the sparse tensor.
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MPSDataType;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Setter for [`dataType`][Self::dataType].
        #[method(setDataType:)]
        pub unsafe fn setDataType(&self, data_type: MPSDataType);

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Creates a descriptor for a sparse tensor.
        ///
        /// - Parameters:
        /// - sparseStorageType: A sparseStorageType.
        /// - dataType: A dataType of the sparse tensor.
        /// - Returns: The descriptor.
        #[method_id(@__retain_semantics Other descriptorWithStorageType:dataType:)]
        pub unsafe fn descriptorWithStorageType_dataType(
            sparse_storage_type: MPSGraphSparseStorageType,
            data_type: MPSDataType,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphCreateSparseOpDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// MPSGraphSparseOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a sparse tensor representation.
        ///
        /// sparseVals corresponds to non zero values in matrix.
        /// indexTensor0 and indexTensor1 are indices used for indexing into sparse data structure.
        /// For COO, indexTensor0 is x index and indexTensor1 is y index.
        /// For CSC, indexTensor0 and indexTensor1 correspond to rowIndex and colStarts respectively.
        /// For CSR, indexTensor0 and indexTensor1 correspond to colIndex and rowStarts respectively.
        /// You must set input tensors appropriately for each sparse storage type.
        ///
        /// - Parameters:
        /// - sparseStorageType: A sparseStorageType.
        /// - inputTensorArray: An array of input tensors as [sparseVals, indexTensor0, indexTensor1].
        /// - shape: The shape of the sparse tensor.
        /// - dataType: The dataType of the sparse tensor.
        /// - name: A name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object.
        #[method_id(@__retain_semantics Other sparseTensorWithType:tensors:shape:dataType:name:)]
        pub unsafe fn sparseTensorWithType_tensors_shape_dataType_name(
            &self,
            sparse_storage_type: MPSGraphSparseStorageType,
            input_tensor_array: &NSArray<MPSGraphTensor>,
            shape: &MPSShape,
            data_type: MPSDataType,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a sparse tensor representation.
        ///
        /// sparseVals corresponds to non zero values in matrix.
        /// indexTensor0 and indexTensor1 are indices used for indexing into sparse data structure.
        /// For COO, indexTensor0 is x index and indexTensor1 is y index .
        /// For CSC, indexTensor0 and indexTensor1 correspond to rowIndex and colStarts respectively.
        /// For CSR, indexTensor0 and indexTensor1 correspond to colIndex and rowStarts respectively.
        /// You must set input tensors appropriately for each sparse storage type.
        ///
        /// - Parameters:
        /// - sparseDescriptor: A sparseDescriptor.
        /// - inputTensorArray: An array of input tensors as [sparseVals, indexTensor0, indexTensor1].
        /// - shape: The shape of the sparse tensor.
        /// - name: A name for the operation.
        /// - Returns: A valid ``MPSGraphTensor`` object
        #[method_id(@__retain_semantics Other sparseTensorWithDescriptor:tensors:shape:name:)]
        pub unsafe fn sparseTensorWithDescriptor_tensors_shape_name(
            &self,
            sparse_descriptor: &MPSGraphCreateSparseOpDescriptor,
            input_tensor_array: &NSArray<MPSGraphTensor>,
            shape: &MPSShape,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
