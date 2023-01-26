//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLMutability {
        MTLMutabilityDefault = 0,
        MTLMutabilityMutable = 1,
        MTLMutabilityImmutable = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLPipelineBufferDescriptor")]
    pub struct MTLPipelineBufferDescriptor;

    #[cfg(feature = "Metal_MTLPipelineBufferDescriptor")]
    unsafe impl ClassType for MTLPipelineBufferDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLPipelineBufferDescriptor")]
unsafe impl NSObjectProtocol for MTLPipelineBufferDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLPipelineBufferDescriptor")]
    unsafe impl MTLPipelineBufferDescriptor {
        #[method(mutability)]
        pub fn mutability(&self) -> MTLMutability;

        #[method(setMutability:)]
        pub fn setMutability(&self, mutability: MTLMutability);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLPipelineBufferDescriptorArray")]
    pub struct MTLPipelineBufferDescriptorArray;

    #[cfg(feature = "Metal_MTLPipelineBufferDescriptorArray")]
    unsafe impl ClassType for MTLPipelineBufferDescriptorArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLPipelineBufferDescriptorArray")]
unsafe impl NSObjectProtocol for MTLPipelineBufferDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLPipelineBufferDescriptorArray")]
    unsafe impl MTLPipelineBufferDescriptorArray {
        #[cfg(feature = "Metal_MTLPipelineBufferDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            buffer_index: NSUInteger,
        ) -> Id<MTLPipelineBufferDescriptor, Shared>;

        #[cfg(feature = "Metal_MTLPipelineBufferDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            buffer: Option<&MTLPipelineBufferDescriptor>,
            buffer_index: NSUInteger,
        );
    }
);
