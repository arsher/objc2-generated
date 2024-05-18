//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnninstancenormalizationgradientstate?language=objc)
    #[unsafe(super(MPSNNGradientState, MPSState, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
    pub struct MPSCNNInstanceNormalizationGradientState;
);

#[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
unsafe impl NSObjectProtocol for MPSCNNInstanceNormalizationGradientState {}

extern_methods!(
    #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
    unsafe impl MPSCNNInstanceNormalizationGradientState {
        #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
        #[method_id(@__retain_semantics Other instanceNormalization)]
        pub unsafe fn instanceNormalization(&self) -> Retained<MPSCNNInstanceNormalization>;

        #[method_id(@__retain_semantics Other gamma)]
        pub unsafe fn gamma(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[method_id(@__retain_semantics Other beta)]
        pub unsafe fn beta(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[method_id(@__retain_semantics Other gradientForGamma)]
        pub unsafe fn gradientForGamma(&self) -> Retained<ProtocolObject<dyn MTLBuffer>>;

        #[method_id(@__retain_semantics Other gradientForBeta)]
        pub unsafe fn gradientForBeta(&self) -> Retained<ProtocolObject<dyn MTLBuffer>>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:textureDescriptor:)]
        pub unsafe fn temporaryStateWithCommandBuffer_textureDescriptor(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:)]
        pub unsafe fn temporaryStateWithCommandBuffer(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:bufferSize:)]
        pub unsafe fn temporaryStateWithCommandBuffer_bufferSize(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:textureDescriptor:)]
        pub unsafe fn initWithDevice_textureDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithResource:)]
        pub unsafe fn initWithResource(
            this: Allocated<Self>,
            resource: Option<&ProtocolObject<dyn MTLResource>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:bufferSize:)]
        pub unsafe fn initWithDevice_bufferSize(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            buffer_size: usize,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSState`
    #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
    unsafe impl MPSCNNInstanceNormalizationGradientState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithDevice:resourceList:)]
        pub unsafe fn initWithDevice_resourceList(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:resourceList:)]
        pub unsafe fn temporaryStateWithCommandBuffer_resourceList(
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

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
    unsafe impl MPSCNNInstanceNormalizationGradientState {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnninstancenormalizationgradientstatebatch?language=objc)
#[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
pub type MPSCNNInstanceNormalizationGradientStateBatch =
    NSArray<MPSCNNInstanceNormalizationGradientState>;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnninstancenormalizationdatasource?language=objc)
    pub unsafe trait MPSCNNInstanceNormalizationDataSource:
        NSCopying + NSObjectProtocol
    {
        #[method(gamma)]
        unsafe fn gamma(&self) -> *mut c_float;

        #[method(beta)]
        unsafe fn beta(&self) -> *mut c_float;

        #[method(numberOfFeatureChannels)]
        unsafe fn numberOfFeatureChannels(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[cfg(all(
            feature = "MPSCNNNormalizationWeights",
            feature = "MPSNNGradientState",
            feature = "MPSState"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other updateGammaAndBetaWithCommandBuffer:instanceNormalizationStateBatch:)]
        unsafe fn updateGammaAndBetaWithCommandBuffer_instanceNormalizationStateBatch(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            instance_normalization_state_batch: &MPSCNNInstanceNormalizationGradientStateBatch,
        ) -> Option<Retained<MPSCNNNormalizationGammaAndBetaState>>;

        #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
        #[optional]
        #[method(updateGammaAndBetaWithInstanceNormalizationStateBatch:)]
        unsafe fn updateGammaAndBetaWithInstanceNormalizationStateBatch(
            &self,
            instance_normalization_state_batch: &MPSCNNInstanceNormalizationGradientStateBatch,
        ) -> bool;

        #[optional]
        #[method(epsilon)]
        unsafe fn epsilon(&self) -> c_float;

        #[optional]
        #[method(encodeWithCoder:)]
        unsafe fn encodeWithCoder(&self, a_coder: &NSCoder);

        #[optional]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[optional]
        #[method(supportsSecureCoding)]
        unsafe fn supportsSecureCoding() -> bool;

        #[optional]
        #[method_id(@__retain_semantics Copy copyWithZone:device:)]
        unsafe fn copyWithZone_device(
            &self,
            zone: *mut NSZone,
            device: Option<&ProtocolObject<dyn MTLDevice>>,
        ) -> Retained<Self>;

        #[optional]
        #[method(load)]
        unsafe fn load(&self) -> bool;

        #[optional]
        #[method(purge)]
        unsafe fn purge(&self);
    }

    unsafe impl ProtocolType for dyn MPSCNNInstanceNormalizationDataSource {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnninstancenormalization?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNInstanceNormalization;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNInstanceNormalization {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNInstanceNormalization {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNInstanceNormalization {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNInstanceNormalization {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNInstanceNormalization {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNInstanceNormalization {
        #[method(epsilon)]
        pub unsafe fn epsilon(&self) -> c_float;

        #[method(setEpsilon:)]
        pub unsafe fn setEpsilon(&self, epsilon: c_float);

        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(
            &self,
        ) -> Retained<ProtocolObject<dyn MPSCNNInstanceNormalizationDataSource>>;

        #[method_id(@__retain_semantics Init initWithDevice:dataSource:)]
        pub unsafe fn initWithDevice_dataSource(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            data_source: &ProtocolObject<dyn MPSCNNInstanceNormalizationDataSource>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[deprecated]
        #[method(reloadDataSource:)]
        pub unsafe fn reloadDataSource(
            &self,
            data_source: &ProtocolObject<dyn MPSCNNInstanceNormalizationDataSource>,
        );

        #[method(reloadGammaAndBetaFromDataSource)]
        pub unsafe fn reloadGammaAndBetaFromDataSource(&self);

        #[cfg(all(feature = "MPSCNNNormalizationWeights", feature = "MPSState"))]
        #[method(reloadGammaAndBetaWithCommandBuffer:gammaAndBetaState:)]
        pub unsafe fn reloadGammaAndBetaWithCommandBuffer_gammaAndBetaState(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            gamma_and_beta_state: &MPSCNNNormalizationGammaAndBetaState,
        );

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
        ) -> Option<Retained<MPSCNNInstanceNormalizationGradientState>>;

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
        ) -> Option<Retained<MPSCNNInstanceNormalizationGradientState>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNInstanceNormalization {
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
    unsafe impl MPSCNNInstanceNormalization {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnninstancenormalizationgradient?language=objc)
    #[unsafe(super(MPSCNNGradientKernel, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNInstanceNormalizationGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNInstanceNormalizationGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNInstanceNormalizationGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNInstanceNormalizationGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNInstanceNormalizationGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNInstanceNormalizationGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNInstanceNormalizationGradient {}
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNGradientKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNInstanceNormalizationGradient {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNInstanceNormalizationGradient {
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
    unsafe impl MPSCNNInstanceNormalizationGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
