//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsndarrayidentity?language=objc)
    #[unsafe(super(
        MPSNDArrayUnaryKernel,
        MPSNDArrayMultiaryKernel,
        MPSNDArrayMultiaryBase,
        MPSKernel,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSKernel", feature = "MPSNDArrayKernel"))]
    pub struct MPSNDArrayIdentity;
);

#[cfg(all(feature = "MPSKernel", feature = "MPSNDArrayKernel"))]
unsafe impl NSCoding for MPSNDArrayIdentity {}

#[cfg(all(feature = "MPSKernel", feature = "MPSNDArrayKernel"))]
unsafe impl NSCopying for MPSNDArrayIdentity {}

#[cfg(all(feature = "MPSKernel", feature = "MPSNDArrayKernel"))]
unsafe impl CopyingHelper for MPSNDArrayIdentity {
    type Result = Self;
}

#[cfg(all(feature = "MPSKernel", feature = "MPSNDArrayKernel"))]
unsafe impl NSObjectProtocol for MPSNDArrayIdentity {}

#[cfg(all(feature = "MPSKernel", feature = "MPSNDArrayKernel"))]
unsafe impl NSSecureCoding for MPSNDArrayIdentity {}

extern_methods!(
    #[cfg(all(feature = "MPSKernel", feature = "MPSNDArrayKernel"))]
    unsafe impl MPSNDArrayIdentity {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MPSCoreTypes", feature = "MPSNDArray"))]
        #[method_id(@__retain_semantics Other reshapeWithCommandBuffer:sourceArray:shape:destinationArray:)]
        pub unsafe fn reshapeWithCommandBuffer_sourceArray_shape_destinationArray(
            &self,
            cmd_buf: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
            source_array: &MPSNDArray,
            shape: &MPSShape,
            destination_array: Option<&MPSNDArray>,
        ) -> Option<Retained<MPSNDArray>>;

        #[cfg(feature = "MPSNDArray")]
        #[method_id(@__retain_semantics Other reshapeWithCommandBuffer:sourceArray:dimensionCount:dimensionSizes:destinationArray:)]
        pub unsafe fn reshapeWithCommandBuffer_sourceArray_dimensionCount_dimensionSizes_destinationArray(
            &self,
            cmd_buf: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
            source_array: &MPSNDArray,
            number_of_dimensions: NSUInteger,
            dimension_sizes: NonNull<NSUInteger>,
            destination_array: Option<&MPSNDArray>,
        ) -> Option<Retained<MPSNDArray>>;

        #[cfg(all(feature = "MPSCoreTypes", feature = "MPSNDArray"))]
        #[method_id(@__retain_semantics Other reshapeWithCommandEncoder:commandBuffer:sourceArray:shape:destinationArray:)]
        pub unsafe fn reshapeWithCommandEncoder_commandBuffer_sourceArray_shape_destinationArray(
            &self,
            encoder: Option<&ProtocolObject<dyn MTLComputeCommandEncoder>>,
            cmd_buf: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
            source_array: &MPSNDArray,
            shape: &MPSShape,
            destination_array: Option<&MPSNDArray>,
        ) -> Option<Retained<MPSNDArray>>;

        #[cfg(feature = "MPSNDArray")]
        #[method_id(@__retain_semantics Other reshapeWithCommandEncoder:commandBuffer:sourceArray:dimensionCount:dimensionSizes:destinationArray:)]
        pub unsafe fn reshapeWithCommandEncoder_commandBuffer_sourceArray_dimensionCount_dimensionSizes_destinationArray(
            &self,
            encoder: Option<&ProtocolObject<dyn MTLComputeCommandEncoder>>,
            cmd_buf: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
            source_array: &MPSNDArray,
            number_of_dimensions: NSUInteger,
            dimension_sizes: NonNull<NSUInteger>,
            destination_array: Option<&MPSNDArray>,
        ) -> Option<Retained<MPSNDArray>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSNDArrayUnaryKernel`
    #[cfg(all(feature = "MPSKernel", feature = "MPSNDArrayKernel"))]
    unsafe impl MPSNDArrayIdentity {
        #[method_id(@__retain_semantics Init initWithDevice:sourceCount:)]
        pub unsafe fn initWithDevice_sourceCount(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            count: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            coder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSKernel", feature = "MPSNDArrayKernel"))]
    unsafe impl MPSNDArrayIdentity {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSKernel", feature = "MPSNDArrayKernel"))]
    unsafe impl MPSNDArrayIdentity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
