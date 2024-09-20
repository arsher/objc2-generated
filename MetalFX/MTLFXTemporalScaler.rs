//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFXTemporalScalerDescriptor;

    unsafe impl ClassType for MTLFXTemporalScalerDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MTLFXTemporalScalerDescriptor {}

unsafe impl CopyingHelper for MTLFXTemporalScalerDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLFXTemporalScalerDescriptor {}

extern_methods!(
    unsafe impl MTLFXTemporalScalerDescriptor {
        #[method(colorTextureFormat)]
        pub unsafe fn colorTextureFormat(&self) -> MTLPixelFormat;

        #[method(setColorTextureFormat:)]
        pub unsafe fn setColorTextureFormat(&self, color_texture_format: MTLPixelFormat);

        #[method(depthTextureFormat)]
        pub unsafe fn depthTextureFormat(&self) -> MTLPixelFormat;

        #[method(setDepthTextureFormat:)]
        pub unsafe fn setDepthTextureFormat(&self, depth_texture_format: MTLPixelFormat);

        #[method(motionTextureFormat)]
        pub unsafe fn motionTextureFormat(&self) -> MTLPixelFormat;

        #[method(setMotionTextureFormat:)]
        pub unsafe fn setMotionTextureFormat(&self, motion_texture_format: MTLPixelFormat);

        #[method(outputTextureFormat)]
        pub unsafe fn outputTextureFormat(&self) -> MTLPixelFormat;

        #[method(setOutputTextureFormat:)]
        pub unsafe fn setOutputTextureFormat(&self, output_texture_format: MTLPixelFormat);

        #[method(inputWidth)]
        pub unsafe fn inputWidth(&self) -> NSUInteger;

        #[method(setInputWidth:)]
        pub unsafe fn setInputWidth(&self, input_width: NSUInteger);

        #[method(inputHeight)]
        pub unsafe fn inputHeight(&self) -> NSUInteger;

        #[method(setInputHeight:)]
        pub unsafe fn setInputHeight(&self, input_height: NSUInteger);

        #[method(outputWidth)]
        pub unsafe fn outputWidth(&self) -> NSUInteger;

        #[method(setOutputWidth:)]
        pub unsafe fn setOutputWidth(&self, output_width: NSUInteger);

        #[method(outputHeight)]
        pub unsafe fn outputHeight(&self) -> NSUInteger;

        #[method(setOutputHeight:)]
        pub unsafe fn setOutputHeight(&self, output_height: NSUInteger);

        #[method(isAutoExposureEnabled)]
        pub unsafe fn isAutoExposureEnabled(&self) -> bool;

        #[method(setAutoExposureEnabled:)]
        pub unsafe fn setAutoExposureEnabled(&self, auto_exposure_enabled: bool);

        #[method(requiresSynchronousInitialization)]
        pub unsafe fn requiresSynchronousInitialization(&self) -> bool;

        #[method(setRequiresSynchronousInitialization:)]
        pub unsafe fn setRequiresSynchronousInitialization(
            &self,
            requires_synchronous_initialization: bool,
        );

        #[method(isInputContentPropertiesEnabled)]
        pub unsafe fn isInputContentPropertiesEnabled(&self) -> bool;

        #[method(setInputContentPropertiesEnabled:)]
        pub unsafe fn setInputContentPropertiesEnabled(
            &self,
            input_content_properties_enabled: bool,
        );

        #[method(inputContentMinScale)]
        pub unsafe fn inputContentMinScale(&self) -> c_float;

        #[method(setInputContentMinScale:)]
        pub unsafe fn setInputContentMinScale(&self, input_content_min_scale: c_float);

        #[method(inputContentMaxScale)]
        pub unsafe fn inputContentMaxScale(&self) -> c_float;

        #[method(setInputContentMaxScale:)]
        pub unsafe fn setInputContentMaxScale(&self, input_content_max_scale: c_float);

        #[method(isReactiveMaskTextureEnabled)]
        pub unsafe fn isReactiveMaskTextureEnabled(&self) -> bool;

        #[method(setReactiveMaskTextureEnabled:)]
        pub unsafe fn setReactiveMaskTextureEnabled(&self, reactive_mask_texture_enabled: bool);

        #[method(reactiveMaskTextureFormat)]
        pub unsafe fn reactiveMaskTextureFormat(&self) -> MTLPixelFormat;

        #[method(setReactiveMaskTextureFormat:)]
        pub unsafe fn setReactiveMaskTextureFormat(
            &self,
            reactive_mask_texture_format: MTLPixelFormat,
        );

        #[method_id(@__retain_semantics New newTemporalScalerWithDevice:)]
        pub unsafe fn newTemporalScalerWithDevice(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<ProtocolObject<dyn MTLFXTemporalScaler>>>;

        #[method(supportedInputContentMinScaleForDevice:)]
        pub unsafe fn supportedInputContentMinScaleForDevice(
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> c_float;

        #[method(supportedInputContentMaxScaleForDevice:)]
        pub unsafe fn supportedInputContentMaxScaleForDevice(
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> c_float;

        #[method(supportsDevice:)]
        pub unsafe fn supportsDevice(device: &ProtocolObject<dyn MTLDevice>) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLFXTemporalScalerDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLFXTemporalScaler: NSObjectProtocol {
        #[method(colorTextureUsage)]
        unsafe fn colorTextureUsage(&self) -> MTLTextureUsage;

        #[method(depthTextureUsage)]
        unsafe fn depthTextureUsage(&self) -> MTLTextureUsage;

        #[method(motionTextureUsage)]
        unsafe fn motionTextureUsage(&self) -> MTLTextureUsage;

        #[method(reactiveTextureUsage)]
        unsafe fn reactiveTextureUsage(&self) -> MTLTextureUsage;

        #[method(outputTextureUsage)]
        unsafe fn outputTextureUsage(&self) -> MTLTextureUsage;

        #[method(inputContentWidth)]
        unsafe fn inputContentWidth(&self) -> NSUInteger;

        #[method(setInputContentWidth:)]
        unsafe fn setInputContentWidth(&self, input_content_width: NSUInteger);

        #[method(inputContentHeight)]
        unsafe fn inputContentHeight(&self) -> NSUInteger;

        #[method(setInputContentHeight:)]
        unsafe fn setInputContentHeight(&self, input_content_height: NSUInteger);

        #[method_id(@__retain_semantics Other colorTexture)]
        unsafe fn colorTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[method(setColorTexture:)]
        unsafe fn setColorTexture(&self, color_texture: Option<&ProtocolObject<dyn MTLTexture>>);

        #[method_id(@__retain_semantics Other depthTexture)]
        unsafe fn depthTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[method(setDepthTexture:)]
        unsafe fn setDepthTexture(&self, depth_texture: Option<&ProtocolObject<dyn MTLTexture>>);

        #[method_id(@__retain_semantics Other motionTexture)]
        unsafe fn motionTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[method(setMotionTexture:)]
        unsafe fn setMotionTexture(&self, motion_texture: Option<&ProtocolObject<dyn MTLTexture>>);

        #[method_id(@__retain_semantics Other outputTexture)]
        unsafe fn outputTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[method(setOutputTexture:)]
        unsafe fn setOutputTexture(&self, output_texture: Option<&ProtocolObject<dyn MTLTexture>>);

        #[method_id(@__retain_semantics Other exposureTexture)]
        unsafe fn exposureTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[method(setExposureTexture:)]
        unsafe fn setExposureTexture(
            &self,
            exposure_texture: Option<&ProtocolObject<dyn MTLTexture>>,
        );

        #[method_id(@__retain_semantics Other reactiveMaskTexture)]
        unsafe fn reactiveMaskTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[method(setReactiveMaskTexture:)]
        unsafe fn setReactiveMaskTexture(
            &self,
            reactive_mask_texture: Option<&ProtocolObject<dyn MTLTexture>>,
        );

        #[method(preExposure)]
        unsafe fn preExposure(&self) -> c_float;

        #[method(setPreExposure:)]
        unsafe fn setPreExposure(&self, pre_exposure: c_float);

        #[method(jitterOffsetX)]
        unsafe fn jitterOffsetX(&self) -> c_float;

        #[method(setJitterOffsetX:)]
        unsafe fn setJitterOffsetX(&self, jitter_offset_x: c_float);

        #[method(jitterOffsetY)]
        unsafe fn jitterOffsetY(&self) -> c_float;

        #[method(setJitterOffsetY:)]
        unsafe fn setJitterOffsetY(&self, jitter_offset_y: c_float);

        #[method(motionVectorScaleX)]
        unsafe fn motionVectorScaleX(&self) -> c_float;

        #[method(setMotionVectorScaleX:)]
        unsafe fn setMotionVectorScaleX(&self, motion_vector_scale_x: c_float);

        #[method(motionVectorScaleY)]
        unsafe fn motionVectorScaleY(&self) -> c_float;

        #[method(setMotionVectorScaleY:)]
        unsafe fn setMotionVectorScaleY(&self, motion_vector_scale_y: c_float);

        #[method(reset)]
        unsafe fn reset(&self) -> bool;

        #[method(setReset:)]
        unsafe fn setReset(&self, reset: bool);

        #[method(isDepthReversed)]
        unsafe fn isDepthReversed(&self) -> bool;

        #[method(setDepthReversed:)]
        unsafe fn setDepthReversed(&self, depth_reversed: bool);

        #[method(colorTextureFormat)]
        unsafe fn colorTextureFormat(&self) -> MTLPixelFormat;

        #[method(depthTextureFormat)]
        unsafe fn depthTextureFormat(&self) -> MTLPixelFormat;

        #[method(motionTextureFormat)]
        unsafe fn motionTextureFormat(&self) -> MTLPixelFormat;

        #[method(outputTextureFormat)]
        unsafe fn outputTextureFormat(&self) -> MTLPixelFormat;

        #[method(inputWidth)]
        unsafe fn inputWidth(&self) -> NSUInteger;

        #[method(inputHeight)]
        unsafe fn inputHeight(&self) -> NSUInteger;

        #[method(outputWidth)]
        unsafe fn outputWidth(&self) -> NSUInteger;

        #[method(outputHeight)]
        unsafe fn outputHeight(&self) -> NSUInteger;

        #[method(inputContentMinScale)]
        unsafe fn inputContentMinScale(&self) -> c_float;

        #[method(inputContentMaxScale)]
        unsafe fn inputContentMaxScale(&self) -> c_float;

        #[method_id(@__retain_semantics Other fence)]
        unsafe fn fence(&self) -> Option<Retained<ProtocolObject<dyn MTLFence>>>;

        #[method(setFence:)]
        unsafe fn setFence(&self, fence: Option<&ProtocolObject<dyn MTLFence>>);

        #[method(encodeToCommandBuffer:)]
        unsafe fn encodeToCommandBuffer(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
        );
    }

    unsafe impl ProtocolType for dyn MTLFXTemporalScaler {}
);
