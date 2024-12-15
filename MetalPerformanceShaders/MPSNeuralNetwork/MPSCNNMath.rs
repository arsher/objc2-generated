//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnarithmeticgradientstate?language=objc)
    #[unsafe(super(MPSNNBinaryGradientState, MPSState, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
    pub struct MPSCNNArithmeticGradientState;
);

#[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
unsafe impl NSObjectProtocol for MPSCNNArithmeticGradientState {}

extern_methods!(
    #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
    unsafe impl MPSCNNArithmeticGradientState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSState`
    #[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
    unsafe impl MPSCNNArithmeticGradientState {
        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:bufferSize:)]
        pub unsafe fn temporaryStateWithCommandBuffer_bufferSize(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:textureDescriptor:)]
        pub unsafe fn temporaryStateWithCommandBuffer_textureDescriptor(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

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

        #[method_id(@__retain_semantics Init initWithResource:)]
        pub unsafe fn initWithResource(
            this: Allocated<Self>,
            resource: Option<&ProtocolObject<dyn MTLResource>>,
        ) -> Retained<Self>;

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
    unsafe impl MPSCNNArithmeticGradientState {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnarithmeticgradientstatebatch?language=objc)
#[cfg(all(feature = "MPSNNGradientState", feature = "MPSState"))]
pub type MPSCNNArithmeticGradientStateBatch = NSArray<MPSCNNArithmeticGradientState>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnarithmetic?language=objc)
    #[unsafe(super(MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNArithmetic;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNArithmetic {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNArithmetic {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNArithmetic {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNArithmetic {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNArithmetic {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNArithmetic {
        #[method(primaryScale)]
        pub unsafe fn primaryScale(&self) -> c_float;

        #[method(setPrimaryScale:)]
        pub unsafe fn setPrimaryScale(&self, primary_scale: c_float);

        #[method(secondaryScale)]
        pub unsafe fn secondaryScale(&self) -> c_float;

        #[method(setSecondaryScale:)]
        pub unsafe fn setSecondaryScale(&self, secondary_scale: c_float);

        #[method(bias)]
        pub unsafe fn bias(&self) -> c_float;

        #[method(setBias:)]
        pub unsafe fn setBias(&self, bias: c_float);

        #[method(primaryStrideInFeatureChannels)]
        pub unsafe fn primaryStrideInFeatureChannels(&self) -> NSUInteger;

        #[method(setPrimaryStrideInFeatureChannels:)]
        pub unsafe fn setPrimaryStrideInFeatureChannels(
            &self,
            primary_stride_in_feature_channels: NSUInteger,
        );

        #[method(secondaryStrideInFeatureChannels)]
        pub unsafe fn secondaryStrideInFeatureChannels(&self) -> NSUInteger;

        #[method(setSecondaryStrideInFeatureChannels:)]
        pub unsafe fn setSecondaryStrideInFeatureChannels(
            &self,
            secondary_stride_in_feature_channels: NSUInteger,
        );

        #[method(minimumValue)]
        pub unsafe fn minimumValue(&self) -> c_float;

        #[method(setMinimumValue:)]
        pub unsafe fn setMinimumValue(&self, minimum_value: c_float);

        #[method(maximumValue)]
        pub unsafe fn maximumValue(&self) -> c_float;

        #[method(setMaximumValue:)]
        pub unsafe fn setMaximumValue(&self, maximum_value: c_float);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "MPSImage",
            feature = "MPSNNGradientState",
            feature = "MPSState"
        ))]
        #[method(encodeToCommandBuffer:primaryImage:secondaryImage:destinationState:destinationImage:)]
        pub unsafe fn encodeToCommandBuffer_primaryImage_secondaryImage_destinationState_destinationImage(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            primary_image: &MPSImage,
            secondary_image: &MPSImage,
            destination_state: &MPSCNNArithmeticGradientState,
            destination_image: &MPSImage,
        );

        #[cfg(all(
            feature = "MPSImage",
            feature = "MPSNDArray",
            feature = "MPSNNGradientState",
            feature = "MPSState"
        ))]
        #[method(encodeBatchToCommandBuffer:primaryImages:secondaryImages:destinationStates:destinationImages:)]
        pub unsafe fn encodeBatchToCommandBuffer_primaryImages_secondaryImages_destinationStates_destinationImages(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            primary_images: &MPSImageBatch,
            secondary_images: &MPSImageBatch,
            destination_states: &MPSCNNArithmeticGradientStateBatch,
            destination_images: &MPSImageBatch,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNBinaryKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNArithmetic {
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
    unsafe impl MPSCNNArithmetic {
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
    unsafe impl MPSCNNArithmetic {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnadd?language=objc)
    #[unsafe(super(MPSCNNArithmetic, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNAdd;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNAdd {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNAdd {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNAdd {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNAdd {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNAdd {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNAdd {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNBinaryKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNAdd {
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
    unsafe impl MPSCNNAdd {
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
    unsafe impl MPSCNNAdd {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnsubtract?language=objc)
    #[unsafe(super(MPSCNNArithmetic, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNSubtract;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNSubtract {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNSubtract {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNSubtract {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNSubtract {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNSubtract {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNSubtract {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNBinaryKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNSubtract {
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
    unsafe impl MPSCNNSubtract {
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
    unsafe impl MPSCNNSubtract {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnmultiply?language=objc)
    #[unsafe(super(MPSCNNArithmetic, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNMultiply;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNMultiply {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNMultiply {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNMultiply {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNMultiply {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNMultiply {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNMultiply {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNBinaryKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNMultiply {
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
    unsafe impl MPSCNNMultiply {
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
    unsafe impl MPSCNNMultiply {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnndivide?language=objc)
    #[unsafe(super(MPSCNNArithmetic, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNDivide;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNDivide {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNDivide {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNDivide {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNDivide {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNDivide {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDivide {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNBinaryKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDivide {
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
    unsafe impl MPSCNNDivide {
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
    unsafe impl MPSCNNDivide {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnncomparisontype?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSNNComparisonType(pub NSUInteger);
bitflags::bitflags! {
    impl MPSNNComparisonType: NSUInteger {
        #[doc(alias = "MPSNNComparisonTypeEqual")]
        const Equal = 0;
        #[doc(alias = "MPSNNComparisonTypeNotEqual")]
        const NotEqual = 1;
        #[doc(alias = "MPSNNComparisonTypeLess")]
        const Less = 2;
        #[doc(alias = "MPSNNComparisonTypeLessOrEqual")]
        const LessOrEqual = 3;
        #[doc(alias = "MPSNNComparisonTypeGreater")]
        const Greater = 4;
        #[doc(alias = "MPSNNComparisonTypeGreaterOrEqual")]
        const GreaterOrEqual = 5;
    }
}

unsafe impl Encode for MPSNNComparisonType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSNNComparisonType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnncompare?language=objc)
    #[unsafe(super(MPSCNNArithmetic, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSNNCompare;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSNNCompare {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSNNCompare {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSNNCompare {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSNNCompare {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSNNCompare {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNCompare {
        #[method(comparisonType)]
        pub unsafe fn comparisonType(&self) -> MPSNNComparisonType;

        #[method(setComparisonType:)]
        pub unsafe fn setComparisonType(&self, comparison_type: MPSNNComparisonType);

        #[method(threshold)]
        pub unsafe fn threshold(&self) -> c_float;

        #[method(setThreshold:)]
        pub unsafe fn setThreshold(&self, threshold: c_float);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNBinaryKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNCompare {
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
    unsafe impl MPSNNCompare {
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
    unsafe impl MPSNNCompare {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnarithmeticgradient?language=objc)
    #[unsafe(super(MPSCNNGradientKernel, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNArithmeticGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNArithmeticGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNArithmeticGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNArithmeticGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNArithmeticGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNArithmeticGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNArithmeticGradient {
        #[method(primaryScale)]
        pub unsafe fn primaryScale(&self) -> c_float;

        #[method(setPrimaryScale:)]
        pub unsafe fn setPrimaryScale(&self, primary_scale: c_float);

        #[method(secondaryScale)]
        pub unsafe fn secondaryScale(&self) -> c_float;

        #[method(setSecondaryScale:)]
        pub unsafe fn setSecondaryScale(&self, secondary_scale: c_float);

        #[method(bias)]
        pub unsafe fn bias(&self) -> c_float;

        #[method(setBias:)]
        pub unsafe fn setBias(&self, bias: c_float);

        #[method(secondaryStrideInFeatureChannels)]
        pub unsafe fn secondaryStrideInFeatureChannels(&self) -> NSUInteger;

        #[method(setSecondaryStrideInFeatureChannels:)]
        pub unsafe fn setSecondaryStrideInFeatureChannels(
            &self,
            secondary_stride_in_feature_channels: NSUInteger,
        );

        #[method(minimumValue)]
        pub unsafe fn minimumValue(&self) -> c_float;

        #[method(setMinimumValue:)]
        pub unsafe fn setMinimumValue(&self, minimum_value: c_float);

        #[method(maximumValue)]
        pub unsafe fn maximumValue(&self) -> c_float;

        #[method(setMaximumValue:)]
        pub unsafe fn setMaximumValue(&self, maximum_value: c_float);

        #[method(isSecondarySourceFilter)]
        pub unsafe fn isSecondarySourceFilter(&self) -> bool;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:isSecondarySourceFilter:)]
        pub unsafe fn initWithDevice_isSecondarySourceFilter(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            is_secondary_source_filter: bool,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNGradientKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNArithmeticGradient {
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
    unsafe impl MPSCNNArithmeticGradient {
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
    unsafe impl MPSCNNArithmeticGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnaddgradient?language=objc)
    #[unsafe(super(
        MPSCNNArithmeticGradient,
        MPSCNNGradientKernel,
        MPSCNNBinaryKernel,
        MPSKernel,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNAddGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNAddGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNAddGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNAddGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNAddGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNAddGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNAddGradient {
        #[method_id(@__retain_semantics Init initWithDevice:isSecondarySourceFilter:)]
        pub unsafe fn initWithDevice_isSecondarySourceFilter(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            is_secondary_source_filter: bool,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNArithmeticGradient`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNAddGradient {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNGradientKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNAddGradient {
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
    unsafe impl MPSCNNAddGradient {
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
    unsafe impl MPSCNNAddGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnsubtractgradient?language=objc)
    #[unsafe(super(
        MPSCNNArithmeticGradient,
        MPSCNNGradientKernel,
        MPSCNNBinaryKernel,
        MPSKernel,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNSubtractGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNSubtractGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNSubtractGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNSubtractGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNSubtractGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNSubtractGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNSubtractGradient {
        #[method_id(@__retain_semantics Init initWithDevice:isSecondarySourceFilter:)]
        pub unsafe fn initWithDevice_isSecondarySourceFilter(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            is_secondary_source_filter: bool,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNArithmeticGradient`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNSubtractGradient {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNGradientKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNSubtractGradient {
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
    unsafe impl MPSCNNSubtractGradient {
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
    unsafe impl MPSCNNSubtractGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnmultiplygradient?language=objc)
    #[unsafe(super(
        MPSCNNArithmeticGradient,
        MPSCNNGradientKernel,
        MPSCNNBinaryKernel,
        MPSKernel,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNMultiplyGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNMultiplyGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNMultiplyGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNMultiplyGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNMultiplyGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNMultiplyGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNMultiplyGradient {
        #[method_id(@__retain_semantics Init initWithDevice:isSecondarySourceFilter:)]
        pub unsafe fn initWithDevice_isSecondarySourceFilter(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            is_secondary_source_filter: bool,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNArithmeticGradient`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNMultiplyGradient {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNGradientKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNMultiplyGradient {
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
    unsafe impl MPSCNNMultiplyGradient {
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
    unsafe impl MPSCNNMultiplyGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);