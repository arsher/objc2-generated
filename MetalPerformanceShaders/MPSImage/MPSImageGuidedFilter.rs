//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimageguidedfilter?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSImageGuidedFilter;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSImageGuidedFilter {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSImageGuidedFilter {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSImageGuidedFilter {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSImageGuidedFilter {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSImageGuidedFilter {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSImageGuidedFilter {
        #[method(kernelDiameter)]
        pub unsafe fn kernelDiameter(&self) -> NSUInteger;

        #[method(epsilon)]
        pub unsafe fn epsilon(&self) -> c_float;

        #[method(setEpsilon:)]
        pub unsafe fn setEpsilon(&self, epsilon: c_float);

        #[method(reconstructScale)]
        pub unsafe fn reconstructScale(&self) -> c_float;

        #[method(setReconstructScale:)]
        pub unsafe fn setReconstructScale(&self, reconstruct_scale: c_float);

        #[method(reconstructOffset)]
        pub unsafe fn reconstructOffset(&self) -> c_float;

        #[method(setReconstructOffset:)]
        pub unsafe fn setReconstructOffset(&self, reconstruct_offset: c_float);

        #[method_id(@__retain_semantics Init initWithDevice:kernelDiameter:)]
        pub unsafe fn initWithDevice_kernelDiameter(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_diameter: NSUInteger,
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

        #[method(encodeRegressionToCommandBuffer:sourceTexture:guidanceTexture:weightsTexture:destinationCoefficientsTexture:)]
        pub unsafe fn encodeRegressionToCommandBuffer_sourceTexture_guidanceTexture_weightsTexture_destinationCoefficientsTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            guidance_texture: &ProtocolObject<dyn MTLTexture>,
            weights_texture: Option<&ProtocolObject<dyn MTLTexture>>,
            destination_coefficients_texture: &ProtocolObject<dyn MTLTexture>,
        );

        #[method(encodeReconstructionToCommandBuffer:guidanceTexture:coefficientsTexture:destinationTexture:)]
        pub unsafe fn encodeReconstructionToCommandBuffer_guidanceTexture_coefficientsTexture_destinationTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            guidance_texture: &ProtocolObject<dyn MTLTexture>,
            coefficients_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
        );

        #[method(encodeRegressionToCommandBuffer:sourceTexture:guidanceTexture:weightsTexture:destinationCoefficientsTextureA:destinationCoefficientsTextureB:)]
        pub unsafe fn encodeRegressionToCommandBuffer_sourceTexture_guidanceTexture_weightsTexture_destinationCoefficientsTextureA_destinationCoefficientsTextureB(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            guidance_texture: &ProtocolObject<dyn MTLTexture>,
            weights_texture: Option<&ProtocolObject<dyn MTLTexture>>,
            destination_coefficients_texture_a: &ProtocolObject<dyn MTLTexture>,
            destination_coefficients_texture_b: &ProtocolObject<dyn MTLTexture>,
        );

        #[method(encodeReconstructionToCommandBuffer:guidanceTexture:coefficientsTextureA:coefficientsTextureB:destinationTexture:)]
        pub unsafe fn encodeReconstructionToCommandBuffer_guidanceTexture_coefficientsTextureA_coefficientsTextureB_destinationTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            guidance_texture: &ProtocolObject<dyn MTLTexture>,
            coefficients_texture_a: &ProtocolObject<dyn MTLTexture>,
            coefficients_texture_b: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSImageGuidedFilter {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSImageGuidedFilter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);