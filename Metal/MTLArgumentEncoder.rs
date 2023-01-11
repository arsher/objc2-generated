//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_protocol!(
    pub struct MTLArgumentEncoder;

    unsafe impl ProtocolType for MTLArgumentEncoder {
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Id<MTLDevice, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method(encodedLength)]
        pub fn encodedLength(&self) -> NSUInteger;

        #[method(alignment)]
        pub fn alignment(&self) -> NSUInteger;

        #[method(setArgumentBuffer:offset:)]
        pub unsafe fn setArgumentBuffer_offset(
            &self,
            argumentBuffer: Option<&MTLBuffer>,
            offset: NSUInteger,
        );

        #[method(setArgumentBuffer:startOffset:arrayElement:)]
        pub unsafe fn setArgumentBuffer_startOffset_arrayElement(
            &self,
            argumentBuffer: Option<&MTLBuffer>,
            startOffset: NSUInteger,
            arrayElement: NSUInteger,
        );

        #[method(setBuffer:offset:atIndex:)]
        pub unsafe fn setBuffer_offset_atIndex(
            &self,
            buffer: Option<&MTLBuffer>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[method(setBuffers:offsets:withRange:)]
        pub unsafe fn setBuffers_offsets_withRange(
            &self,
            buffers: NonNull<*const MTLBuffer>,
            offsets: NonNull<NSUInteger>,
            range: NSRange,
        );

        #[method(setTexture:atIndex:)]
        pub unsafe fn setTexture_atIndex(&self, texture: Option<&MTLTexture>, index: NSUInteger);

        #[method(setTextures:withRange:)]
        pub unsafe fn setTextures_withRange(
            &self,
            textures: NonNull<*const MTLTexture>,
            range: NSRange,
        );

        #[method(setSamplerState:atIndex:)]
        pub unsafe fn setSamplerState_atIndex(
            &self,
            sampler: Option<&MTLSamplerState>,
            index: NSUInteger,
        );

        #[method(setSamplerStates:withRange:)]
        pub unsafe fn setSamplerStates_withRange(
            &self,
            samplers: NonNull<*const MTLSamplerState>,
            range: NSRange,
        );

        #[method(constantDataAtIndex:)]
        pub unsafe fn constantDataAtIndex(&self, index: NSUInteger) -> NonNull<c_void>;

        #[method(setRenderPipelineState:atIndex:)]
        pub unsafe fn setRenderPipelineState_atIndex(
            &self,
            pipeline: Option<&MTLRenderPipelineState>,
            index: NSUInteger,
        );

        #[method(setRenderPipelineStates:withRange:)]
        pub unsafe fn setRenderPipelineStates_withRange(
            &self,
            pipelines: NonNull<*const MTLRenderPipelineState>,
            range: NSRange,
        );

        #[method(setComputePipelineState:atIndex:)]
        pub unsafe fn setComputePipelineState_atIndex(
            &self,
            pipeline: Option<&MTLComputePipelineState>,
            index: NSUInteger,
        );

        #[method(setComputePipelineStates:withRange:)]
        pub unsafe fn setComputePipelineStates_withRange(
            &self,
            pipelines: NonNull<*const MTLComputePipelineState>,
            range: NSRange,
        );

        #[method(setIndirectCommandBuffer:atIndex:)]
        pub unsafe fn setIndirectCommandBuffer_atIndex(
            &self,
            indirectCommandBuffer: Option<&MTLIndirectCommandBuffer>,
            index: NSUInteger,
        );

        #[method(setIndirectCommandBuffers:withRange:)]
        pub unsafe fn setIndirectCommandBuffers_withRange(
            &self,
            buffers: NonNull<*const MTLIndirectCommandBuffer>,
            range: NSRange,
        );

        #[method(setAccelerationStructure:atIndex:)]
        pub unsafe fn setAccelerationStructure_atIndex(
            &self,
            accelerationStructure: Option<&MTLAccelerationStructure>,
            index: NSUInteger,
        );

        #[method_id(@__retain_semantics New newArgumentEncoderForBufferAtIndex:)]
        pub unsafe fn newArgumentEncoderForBufferAtIndex(
            &self,
            index: NSUInteger,
        ) -> Option<Id<MTLArgumentEncoder, Shared>>;

        #[method(setVisibleFunctionTable:atIndex:)]
        pub unsafe fn setVisibleFunctionTable_atIndex(
            &self,
            visibleFunctionTable: Option<&MTLVisibleFunctionTable>,
            index: NSUInteger,
        );

        #[method(setVisibleFunctionTables:withRange:)]
        pub unsafe fn setVisibleFunctionTables_withRange(
            &self,
            visibleFunctionTables: NonNull<*const MTLVisibleFunctionTable>,
            range: NSRange,
        );

        #[method(setIntersectionFunctionTable:atIndex:)]
        pub unsafe fn setIntersectionFunctionTable_atIndex(
            &self,
            intersectionFunctionTable: Option<&MTLIntersectionFunctionTable>,
            index: NSUInteger,
        );

        #[method(setIntersectionFunctionTables:withRange:)]
        pub unsafe fn setIntersectionFunctionTables_withRange(
            &self,
            intersectionFunctionTables: NonNull<*const MTLIntersectionFunctionTable>,
            range: NSRange,
        );
    }
);
