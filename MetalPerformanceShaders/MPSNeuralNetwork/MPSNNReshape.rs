//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnreshape?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSNNReshape;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSNNReshape {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSNNReshape {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSNNReshape {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSNNReshape {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSNNReshape {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNReshape {
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

        #[cfg(feature = "MPSImage")]
        #[method_id(@__retain_semantics Other encodeToCommandBuffer:sourceImage:reshapedWidth:reshapedHeight:reshapedFeatureChannels:)]
        pub unsafe fn encodeToCommandBuffer_sourceImage_reshapedWidth_reshapedHeight_reshapedFeatureChannels(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImage,
            reshaped_width: NSUInteger,
            reshaped_height: NSUInteger,
            reshaped_feature_channels: NSUInteger,
        ) -> Retained<MPSImage>;

        #[cfg(all(feature = "MPSImage", feature = "MPSState"))]
        #[method_id(@__retain_semantics Other encodeToCommandBuffer:sourceImage:destinationState:destinationStateIsTemporary:reshapedWidth:reshapedHeight:reshapedFeatureChannels:)]
        pub unsafe fn encodeToCommandBuffer_sourceImage_destinationState_destinationStateIsTemporary_reshapedWidth_reshapedHeight_reshapedFeatureChannels(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImage,
            out_state: &mut Option<Retained<MPSState>>,
            is_temporary: bool,
            reshaped_width: NSUInteger,
            reshaped_height: NSUInteger,
            reshaped_feature_channels: NSUInteger,
        ) -> Retained<MPSImage>;

        #[cfg(all(feature = "MPSImage", feature = "MPSNDArray"))]
        #[method_id(@__retain_semantics Other encodeBatchToCommandBuffer:sourceImages:reshapedWidth:reshapedHeight:reshapedFeatureChannels:)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceImages_reshapedWidth_reshapedHeight_reshapedFeatureChannels(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_images: &MPSImageBatch,
            reshaped_width: NSUInteger,
            reshaped_height: NSUInteger,
            reshaped_feature_channels: NSUInteger,
        ) -> Retained<MPSImageBatch>;

        #[cfg(all(feature = "MPSImage", feature = "MPSNDArray", feature = "MPSState"))]
        #[method_id(@__retain_semantics Other encodeBatchToCommandBuffer:sourceImages:destinationStates:destinationStateIsTemporary:reshapedWidth:reshapedHeight:reshapedFeatureChannels:)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceImages_destinationStates_destinationStateIsTemporary_reshapedWidth_reshapedHeight_reshapedFeatureChannels(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_images: &MPSImageBatch,
            out_states: &mut Option<Retained<MPSStateBatch>>,
            is_temporary: bool,
            reshaped_width: NSUInteger,
            reshaped_height: NSUInteger,
            reshaped_feature_channels: NSUInteger,
        ) -> Retained<MPSImageBatch>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNReshape {
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
    unsafe impl MPSNNReshape {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnreshapegradient?language=objc)
    #[unsafe(super(MPSCNNGradientKernel, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSNNReshapeGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSNNReshapeGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSNNReshapeGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSNNReshapeGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSNNReshapeGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSNNReshapeGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNReshapeGradient {
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
    unsafe impl MPSNNReshapeGradient {
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
    unsafe impl MPSNNReshapeGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnpad?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSNNPad;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSNNPad {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSNNPad {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSNNPad {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSNNPad {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSNNPad {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNPad {
        #[cfg(feature = "MPSCoreTypes")]
        #[method(paddingSizeBefore)]
        pub unsafe fn paddingSizeBefore(&self) -> MPSImageCoordinate;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(setPaddingSizeBefore:)]
        pub unsafe fn setPaddingSizeBefore(&self, padding_size_before: MPSImageCoordinate);

        #[cfg(feature = "MPSCoreTypes")]
        #[method(paddingSizeAfter)]
        pub unsafe fn paddingSizeAfter(&self) -> MPSImageCoordinate;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(setPaddingSizeAfter:)]
        pub unsafe fn setPaddingSizeAfter(&self, padding_size_after: MPSImageCoordinate);

        #[method(fillValue)]
        pub unsafe fn fillValue(&self) -> c_float;

        #[method(setFillValue:)]
        pub unsafe fn setFillValue(&self, fill_value: c_float);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSCoreTypes")]
        #[method_id(@__retain_semantics Init initWithDevice:paddingSizeBefore:paddingSizeAfter:)]
        pub unsafe fn initWithDevice_paddingSizeBefore_paddingSizeAfter(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            padding_size_before: MPSImageCoordinate,
            padding_size_after: MPSImageCoordinate,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSCoreTypes")]
        #[method_id(@__retain_semantics Init initWithDevice:paddingSizeBefore:paddingSizeAfter:fillValueArray:)]
        pub unsafe fn initWithDevice_paddingSizeBefore_paddingSizeAfter_fillValueArray(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            padding_size_before: MPSImageCoordinate,
            padding_size_after: MPSImageCoordinate,
            fill_value_array: Option<&NSData>,
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
    unsafe impl MPSNNPad {
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
    unsafe impl MPSNNPad {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnpadgradient?language=objc)
    #[unsafe(super(MPSCNNGradientKernel, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSNNPadGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSNNPadGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSNNPadGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSNNPadGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSNNPadGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSNNPadGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNPadGradient {
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
    unsafe impl MPSNNPadGradient {
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
    unsafe impl MPSNNPadGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
