//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// Dependencies: This depends on Metal.framework.
    ///
    /// The MPSCNNDropoutGradientState is used to hold the mask used by both
    /// MPSCNNDropout forward filter and MPSCNNDropoutGradient backward filter.
    /// The MPSCNNDropout forward filter populates the MPSCNNDropoutGradientState
    /// object and the MPSCNNDropoutGradient backward filter consumes the state
    /// object.
    ///
    /// While the mask is stored internally, the mask data is accessible by the
    /// user for debugging purposes via an accessor method.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnndropoutgradientstate?language=objc)
    #[unsafe(super(MPSNNGradientState, MPSState, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
    pub struct MPSCNNDropoutGradientState;
);

#[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
unsafe impl NSObjectProtocol for MPSCNNDropoutGradientState {}

extern_methods!(
    #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
    unsafe impl MPSCNNDropoutGradientState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Mask data accessor method.
        ///
        /// Returns: An autoreleased NSData object, containing the mask data.
        /// The mask data is populated in the -encode call, thus the contents
        /// are undefined until you -encode the filter.
        /// Use for debugging purposes only.
        ///
        /// In order to gaurantee that the mask data is correctly synchronized for CPU side access,
        /// it is the application's responsibility to call the [gradientState synchronizeOnCommandBuffer:]
        /// method before accessing the mask data.
        #[method_id(@__retain_semantics Other maskData)]
        pub unsafe fn maskData(&self) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSState`
    #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
    unsafe impl MPSCNNDropoutGradientState {
        /// Create a MPSState holding a temporary MTLBuffer
        ///
        /// Parameter `cmdBuf`: The command buffer against which the temporary resource is allocated
        ///
        /// Parameter `bufferSize`: The size of the buffer in bytes
        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:bufferSize:)]
        pub unsafe fn temporaryStateWithCommandBuffer_bufferSize(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            buffer_size: usize,
        ) -> Retained<Self>;

        /// Create a MPSState holding a temporary MTLTexture
        ///
        /// Parameter `cmdBuf`: The command buffer against which the temporary resource is allocated
        ///
        /// Parameter `descriptor`: A descriptor for the new temporary texture
        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:textureDescriptor:)]
        pub unsafe fn temporaryStateWithCommandBuffer_textureDescriptor(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        /// Create a new autoreleased temporary state object without underlying resource
        ///
        /// Parameter `cmdBuf`: The command buffer with which the temporary resource is associated
        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:)]
        pub unsafe fn temporaryStateWithCommandBuffer(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:bufferSize:)]
        pub unsafe fn initWithDevice_bufferSize(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:textureDescriptor:)]
        pub unsafe fn initWithDevice_textureDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        /// Create a MPSState with a non-temporary MTLResource
        ///
        /// Parameter `resource`: A MTLBuffer or MTLTexture. May be nil.
        #[method_id(@__retain_semantics Init initWithResource:)]
        pub unsafe fn initWithResource(
            this: Allocated<Self>,
            resource: Option<&ProtocolObject<dyn MTLResource>>,
        ) -> Retained<Self>;

        /// Initialize a non-temporary state to hold a number of textures and buffers
        ///
        /// The allocation of each resource will be deferred  until it is needed.
        /// This occurs when -resource or -resourceAtIndex: is called.
        ///
        /// Parameter `resourceList`: The list of resources to create.
        #[method_id(@__retain_semantics Init initWithDevice:resourceList:)]
        pub unsafe fn initWithDevice_resourceList(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        /// Initialize a temporary state to hold a number of textures and buffers
        ///
        /// The textures occur first in sequence
        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:resourceList:)]
        pub unsafe fn temporaryStateWithCommandBuffer_resourceList(
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        /// Create a state object with a list of MTLResources
        ///
        /// Because MPS prefers deferred allocation of resources
        /// your application should use -initWithTextures:bufferSizes:bufferCount:
        /// whenever possible. This method is useful for cases when the
        /// MTLResources must be initialized by the CPU.
        #[method_id(@__retain_semantics Init initWithResources:)]
        pub unsafe fn initWithResources(
            this: Allocated<Self>,
            resources: Option<&NSArray<ProtocolObject<dyn MTLResource>>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
    unsafe impl MPSCNNDropoutGradientState {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnndropoutgradientstatebatch?language=objc)
#[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
pub type MPSCNNDropoutGradientStateBatch = NSArray<MPSCNNDropoutGradientState>;

extern_class!(
    /// Dependencies: This depends on Metal.framework
    ///
    /// Dropout is a regularization technique used to prevent neural networks from
    /// overfitting during training. With probability keepProbability, this filter
    /// outputs the input element scaled by 1 / keepProbability. Otherwise, it
    /// outputs 0. Each input element is kept or dropped independently. The scaling
    /// is performed to keep the energy of the output unchanged.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnndropout?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNDropout;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNDropout {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNDropout {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNDropout {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNDropout {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNDropout {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDropout {
        /// The probability that each element in the input is kept.
        /// The valid range is (0.0f, 1.0f).
        #[method(keepProbability)]
        pub unsafe fn keepProbability(&self) -> c_float;

        /// The seed used to generate random numbers.
        #[method(seed)]
        pub unsafe fn seed(&self) -> NSUInteger;

        /// The mask stride in the x, y, and x dimensions, which
        /// allows for the broadcasting the mask data.
        ///
        /// The only valid values are 0 and 1 for each dimension.
        /// For no broadcasting, set the values for each dimension
        /// to 1. For broadcasting, set desired values to 0.
        #[method(maskStrideInPixels)]
        pub unsafe fn maskStrideInPixels(&self) -> MTLSize;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// <NSSecureCoding
        /// > support
        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        /// Standard init with default properties per filter type.
        ///
        /// Parameter `device`: The device that the filter will be used on.
        ///
        /// Parameter `keepProbability`: The probability that each element in the input is kept.
        /// The valid range is (0.0f, 1.0f).
        ///
        /// Parameter `seed`: The seed used to generate random numbers.
        ///
        /// Parameter `maskStrideInPixels`: The mask stride in the x, y, and z dimensions, which
        /// allows for the broadcasting of mask data. The only valid
        /// values are 0 and 1 for each dimension. For no
        /// broadcasting, set the values for each dimension to 1.
        /// For broadcasting, set desired values to 0.
        ///
        /// Returns: A valid MPSCNNDropout object or nil, if failure.
        #[method_id(@__retain_semantics Init initWithDevice:keepProbability:seed:maskStrideInPixels:)]
        pub unsafe fn initWithDevice_keepProbability_seed_maskStrideInPixels(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            keep_probability: c_float,
            seed: NSUInteger,
            mask_stride_in_pixels: MTLSize,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "MPSImage",
            feature = "MPSNNGradientState",
            feature = "MPSState"
        ))]
        #[method_id(@__retain_semantics Other resultStateForSourceImage:sourceStates:destinationImage:)]
        pub unsafe fn resultStateForSourceImage_sourceStates_destinationImage(
            &self,
            source_image: &MPSImage,
            source_states: Option<&NSArray<MPSState>>,
            destination_image: &MPSImage,
        ) -> Option<Retained<MPSCNNDropoutGradientState>>;

        #[cfg(all(
            feature = "MPSImage",
            feature = "MPSNDArray",
            feature = "MPSNNGradientState",
            feature = "MPSState"
        ))]
        #[method_id(@__retain_semantics Other resultStateBatchForSourceImage:sourceStates:destinationImage:)]
        pub unsafe fn resultStateBatchForSourceImage_sourceStates_destinationImage(
            &self,
            source_image: &MPSImageBatch,
            source_states: Option<&NSArray<MPSStateBatch>>,
            destination_image: &MPSImageBatch,
        ) -> Option<Retained<MPSCNNDropoutGradientState>>;

        #[cfg(all(
            feature = "MPSImage",
            feature = "MPSNNGradientState",
            feature = "MPSState"
        ))]
        #[method_id(@__retain_semantics Other temporaryResultStateForCommandBuffer:sourceImage:sourceStates:destinationImage:)]
        pub unsafe fn temporaryResultStateForCommandBuffer_sourceImage_sourceStates_destinationImage(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImage,
            source_states: Option<&NSArray<MPSState>>,
            destination_image: &MPSImage,
        ) -> Option<Retained<MPSCNNDropoutGradientState>>;

        #[cfg(all(
            feature = "MPSImage",
            feature = "MPSNDArray",
            feature = "MPSNNGradientState",
            feature = "MPSState"
        ))]
        #[method_id(@__retain_semantics Other temporaryResultStateBatchForCommandBuffer:sourceImage:sourceStates:destinationImage:)]
        pub unsafe fn temporaryResultStateBatchForCommandBuffer_sourceImage_sourceStates_destinationImage(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImageBatch,
            source_states: Option<&NSArray<MPSStateBatch>>,
            destination_image: &MPSImageBatch,
        ) -> Option<Retained<MPSCNNDropoutGradientStateBatch>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDropout {
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDropout {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Dependencies: This depends on Metal.framework
    ///
    /// This filter is the backward filter for the MPSCNNDropout forward filter.
    /// It requires the mask data, along with all the associated parameters used
    /// to generate the mask, from the forward pass. The mask is associated with
    /// a MPSCNNDropoutGradientState object.
    ///
    /// In this kernel, use the secondaryOffset to apply an offset to the mask data.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnndropoutgradient?language=objc)
    #[unsafe(super(MPSCNNGradientKernel, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNDropoutGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNDropoutGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNDropoutGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNDropoutGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNDropoutGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNDropoutGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDropoutGradient {
        /// The probability that each element in the input is kept.
        /// The valid range is (0.0f, 1.0f).
        #[method(keepProbability)]
        pub unsafe fn keepProbability(&self) -> c_float;

        /// The seed used to generate random numbers.
        #[method(seed)]
        pub unsafe fn seed(&self) -> NSUInteger;

        /// The mask stride in the x, y, and x dimensions, which
        /// allows for the broadcasting the mask data.
        ///
        /// The only valid values are 0 and 1 for each dimension.
        /// For no broadcasting, set the values for each dimension
        /// to 1. For broadcasting, set desired values to 0.
        #[method(maskStrideInPixels)]
        pub unsafe fn maskStrideInPixels(&self) -> MTLSize;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// <NSSecureCoding
        /// > support
        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        /// Standard init with default properties per filter type.
        ///
        /// Parameter `device`: The device that the filter will be used on.
        ///
        /// Parameter `keepProbability`: The probability that each element in the input is kept.
        /// The valid range is (0.0f, 1.0f).
        ///
        /// Parameter `seed`: The seed used to generate random numbers.
        ///
        /// Parameter `maskStrideInPixels`: The mask stride in the x, y, and z dimensions, which
        /// allows for the broadcasting of mask data. The only valid
        /// values are 0 and 1 for each dimension. For no
        /// broadcasting, set the values for each dimension to 1.
        /// For broadcasting, set desired values to 0.
        ///
        /// Returns: A valid MPSCNNDropoutGradient object or nil, if failure.
        #[method_id(@__retain_semantics Init initWithDevice:keepProbability:seed:maskStrideInPixels:)]
        pub unsafe fn initWithDevice_keepProbability_seed_maskStrideInPixels(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            keep_probability: c_float,
            seed: NSUInteger,
            mask_stride_in_pixels: MTLSize,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDropoutGradient {
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDropoutGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
