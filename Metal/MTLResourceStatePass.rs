//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptor")]
    pub struct MTLResourceStatePassSampleBufferAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptor")]
    unsafe impl ClassType for MTLResourceStatePassSampleBufferAttachmentDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLResourceStatePassSampleBufferAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptor")]
    unsafe impl MTLResourceStatePassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Other sampleBuffer)]
        pub unsafe fn sampleBuffer(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLCounterSampleBuffer>, Shared>>;

        #[method(setSampleBuffer:)]
        pub unsafe fn setSampleBuffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
        );

        #[method(startOfEncoderSampleIndex)]
        pub unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfEncoderSampleIndex:)]
        pub unsafe fn setStartOfEncoderSampleIndex(
            &self,
            start_of_encoder_sample_index: NSUInteger,
        );

        #[method(endOfEncoderSampleIndex)]
        pub unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfEncoderSampleIndex:)]
        pub unsafe fn setEndOfEncoderSampleIndex(&self, end_of_encoder_sample_index: NSUInteger);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptorArray")]
    pub struct MTLResourceStatePassSampleBufferAttachmentDescriptorArray;

    #[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptorArray")]
    unsafe impl ClassType for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptorArray")]
unsafe impl NSObjectProtocol for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptorArray")]
    unsafe impl MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
        #[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Id<MTLResourceStatePassSampleBufferAttachmentDescriptor, Shared>;

        #[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLResourceStatePassSampleBufferAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLResourceStatePassDescriptor")]
    pub struct MTLResourceStatePassDescriptor;

    #[cfg(feature = "Metal_MTLResourceStatePassDescriptor")]
    unsafe impl ClassType for MTLResourceStatePassDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLResourceStatePassDescriptor")]
unsafe impl NSObjectProtocol for MTLResourceStatePassDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLResourceStatePassDescriptor")]
    unsafe impl MTLResourceStatePassDescriptor {
        #[method_id(@__retain_semantics Other resourceStatePassDescriptor)]
        pub unsafe fn resourceStatePassDescriptor() -> Id<MTLResourceStatePassDescriptor, Shared>;

        #[cfg(feature = "Metal_MTLResourceStatePassSampleBufferAttachmentDescriptorArray")]
        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub unsafe fn sampleBufferAttachments(
            &self,
        ) -> Id<MTLResourceStatePassSampleBufferAttachmentDescriptorArray, Shared>;
    }
);
