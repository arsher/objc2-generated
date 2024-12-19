//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtliopriority?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIOPriority(pub NSInteger);
impl MTLIOPriority {
    #[doc(alias = "MTLIOPriorityHigh")]
    pub const High: Self = Self(0);
    #[doc(alias = "MTLIOPriorityNormal")]
    pub const Normal: Self = Self(1);
    #[doc(alias = "MTLIOPriorityLow")]
    pub const Low: Self = Self(2);
}

unsafe impl Encode for MTLIOPriority {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLIOPriority {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtliocommandqueuetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIOCommandQueueType(pub NSInteger);
impl MTLIOCommandQueueType {
    #[doc(alias = "MTLIOCommandQueueTypeConcurrent")]
    pub const Concurrent: Self = Self(0);
    #[doc(alias = "MTLIOCommandQueueTypeSerial")]
    pub const Serial: Self = Self(1);
}

unsafe impl Encode for MTLIOCommandQueueType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLIOCommandQueueType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlioerrordomain?language=objc)
    pub static MTLIOErrorDomain: &'static NSErrorDomain;
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlioerror?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIOError(pub NSInteger);
impl MTLIOError {
    #[doc(alias = "MTLIOErrorURLInvalid")]
    pub const URLInvalid: Self = Self(1);
    #[doc(alias = "MTLIOErrorInternal")]
    pub const Internal: Self = Self(2);
}

unsafe impl Encode for MTLIOError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLIOError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtliocommandqueue?language=objc)
    pub unsafe trait MTLIOCommandQueue: NSObjectProtocol {
        #[method(enqueueBarrier)]
        unsafe fn enqueueBarrier(&self);

        #[cfg(feature = "MTLIOCommandBuffer")]
        #[method_id(@__retain_semantics Other commandBuffer)]
        unsafe fn commandBuffer(&self) -> Retained<ProtocolObject<dyn MTLIOCommandBuffer>>;

        #[cfg(feature = "MTLIOCommandBuffer")]
        #[method_id(@__retain_semantics Other commandBufferWithUnretainedReferences)]
        unsafe fn commandBufferWithUnretainedReferences(
            &self,
        ) -> Retained<ProtocolObject<dyn MTLIOCommandBuffer>>;

        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        unsafe fn setLabel(&self, label: Option<&NSString>);
    }

    unsafe impl ProtocolType for dyn MTLIOCommandQueue {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlioscratchbuffer?language=objc)
    pub unsafe trait MTLIOScratchBuffer: NSObjectProtocol {
        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method_id(@__retain_semantics Other buffer)]
        unsafe fn buffer(&self) -> Retained<ProtocolObject<dyn MTLBuffer>>;
    }

    unsafe impl ProtocolType for dyn MTLIOScratchBuffer {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator?language=objc)
    pub unsafe trait MTLIOScratchBufferAllocator: NSObjectProtocol {
        #[method_id(@__retain_semantics New newScratchBufferWithMinimumSize:)]
        unsafe fn newScratchBufferWithMinimumSize(
            &self,
            minimum_size: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLIOScratchBuffer>>>;
    }

    unsafe impl ProtocolType for dyn MTLIOScratchBufferAllocator {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIOCommandQueueDescriptor;
);

unsafe impl NSCopying for MTLIOCommandQueueDescriptor {}

unsafe impl CopyingHelper for MTLIOCommandQueueDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLIOCommandQueueDescriptor {}

extern_methods!(
    unsafe impl MTLIOCommandQueueDescriptor {
        #[method(maxCommandBufferCount)]
        pub unsafe fn maxCommandBufferCount(&self) -> NSUInteger;

        #[method(setMaxCommandBufferCount:)]
        pub unsafe fn setMaxCommandBufferCount(&self, max_command_buffer_count: NSUInteger);

        #[method(priority)]
        pub unsafe fn priority(&self) -> MTLIOPriority;

        #[method(setPriority:)]
        pub unsafe fn setPriority(&self, priority: MTLIOPriority);

        #[method(type)]
        pub unsafe fn r#type(&self) -> MTLIOCommandQueueType;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: MTLIOCommandQueueType);

        #[method(maxCommandsInFlight)]
        pub unsafe fn maxCommandsInFlight(&self) -> NSUInteger;

        #[method(setMaxCommandsInFlight:)]
        pub unsafe fn setMaxCommandsInFlight(&self, max_commands_in_flight: NSUInteger);

        #[method_id(@__retain_semantics Other scratchBufferAllocator)]
        pub unsafe fn scratchBufferAllocator(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLIOScratchBufferAllocator>>>;

        #[method(setScratchBufferAllocator:)]
        pub unsafe fn setScratchBufferAllocator(
            &self,
            scratch_buffer_allocator: Option<&ProtocolObject<dyn MTLIOScratchBufferAllocator>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLIOCommandQueueDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtliofilehandle?language=objc)
    pub unsafe trait MTLIOFileHandle: NSObjectProtocol {
        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        unsafe fn setLabel(&self, label: Option<&NSString>);
    }

    unsafe impl ProtocolType for dyn MTLIOFileHandle {}
);
