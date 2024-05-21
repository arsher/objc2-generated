//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCommandBufferStatus(pub NSUInteger);
impl MTLCommandBufferStatus {
    #[doc(alias = "MTLCommandBufferStatusNotEnqueued")]
    pub const NotEnqueued: Self = Self(0);
    #[doc(alias = "MTLCommandBufferStatusEnqueued")]
    pub const Enqueued: Self = Self(1);
    #[doc(alias = "MTLCommandBufferStatusCommitted")]
    pub const Committed: Self = Self(2);
    #[doc(alias = "MTLCommandBufferStatusScheduled")]
    pub const Scheduled: Self = Self(3);
    #[doc(alias = "MTLCommandBufferStatusCompleted")]
    pub const Completed: Self = Self(4);
    #[doc(alias = "MTLCommandBufferStatusError")]
    pub const Error: Self = Self(5);
}

unsafe impl Encode for MTLCommandBufferStatus {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLCommandBufferStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static MTLCommandBufferErrorDomain: &'static NSErrorDomain;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCommandBufferError(pub NSUInteger);
impl MTLCommandBufferError {
    #[doc(alias = "MTLCommandBufferErrorNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MTLCommandBufferErrorInternal")]
    pub const Internal: Self = Self(1);
    #[doc(alias = "MTLCommandBufferErrorTimeout")]
    pub const Timeout: Self = Self(2);
    #[doc(alias = "MTLCommandBufferErrorPageFault")]
    pub const PageFault: Self = Self(3);
    #[deprecated]
    #[doc(alias = "MTLCommandBufferErrorBlacklisted")]
    pub const Blacklisted: Self = Self(4);
    #[doc(alias = "MTLCommandBufferErrorAccessRevoked")]
    pub const AccessRevoked: Self = Self(4);
    #[doc(alias = "MTLCommandBufferErrorNotPermitted")]
    pub const NotPermitted: Self = Self(7);
    #[doc(alias = "MTLCommandBufferErrorOutOfMemory")]
    pub const OutOfMemory: Self = Self(8);
    #[doc(alias = "MTLCommandBufferErrorInvalidResource")]
    pub const InvalidResource: Self = Self(9);
    #[doc(alias = "MTLCommandBufferErrorMemoryless")]
    pub const Memoryless: Self = Self(10);
    #[doc(alias = "MTLCommandBufferErrorDeviceRemoved")]
    pub const DeviceRemoved: Self = Self(11);
    #[doc(alias = "MTLCommandBufferErrorStackOverflow")]
    pub const StackOverflow: Self = Self(12);
}

unsafe impl Encode for MTLCommandBufferError {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLCommandBufferError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static MTLCommandBufferEncoderInfoErrorKey: &'static NSErrorUserInfoKey;
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCommandBufferErrorOption(pub NSUInteger);
bitflags::bitflags! {
    impl MTLCommandBufferErrorOption: NSUInteger {
        #[doc(alias = "MTLCommandBufferErrorOptionNone")]
        const None = 0;
        #[doc(alias = "MTLCommandBufferErrorOptionEncoderExecutionStatus")]
        const EncoderExecutionStatus = 1<<0;
    }
}

unsafe impl Encode for MTLCommandBufferErrorOption {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLCommandBufferErrorOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCommandEncoderErrorState(pub NSInteger);
impl MTLCommandEncoderErrorState {
    #[doc(alias = "MTLCommandEncoderErrorStateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "MTLCommandEncoderErrorStateCompleted")]
    pub const Completed: Self = Self(1);
    #[doc(alias = "MTLCommandEncoderErrorStateAffected")]
    pub const Affected: Self = Self(2);
    #[doc(alias = "MTLCommandEncoderErrorStatePending")]
    pub const Pending: Self = Self(3);
    #[doc(alias = "MTLCommandEncoderErrorStateFaulted")]
    pub const Faulted: Self = Self(4);
}

unsafe impl Encode for MTLCommandEncoderErrorState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLCommandEncoderErrorState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCommandBufferDescriptor;

    unsafe impl ClassType for MTLCommandBufferDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MTLCommandBufferDescriptor {}

unsafe impl NSObjectProtocol for MTLCommandBufferDescriptor {}

extern_methods!(
    unsafe impl MTLCommandBufferDescriptor {
        #[method(retainedReferences)]
        pub unsafe fn retainedReferences(&self) -> bool;

        #[method(setRetainedReferences:)]
        pub unsafe fn setRetainedReferences(&self, retained_references: bool);

        #[method(errorOptions)]
        pub unsafe fn errorOptions(&self) -> MTLCommandBufferErrorOption;

        #[method(setErrorOptions:)]
        pub unsafe fn setErrorOptions(&self, error_options: MTLCommandBufferErrorOption);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLCommandBufferDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLCommandBufferEncoderInfo: NSObjectProtocol + IsRetainable {
        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other debugSignposts)]
        unsafe fn debugSignposts(&self) -> Retained<NSArray<NSString>>;

        #[method(errorState)]
        unsafe fn errorState(&self) -> MTLCommandEncoderErrorState;
    }

    unsafe impl ProtocolType for dyn MTLCommandBufferEncoderInfo {}
);

#[cfg(feature = "block2")]
pub type MTLCommandBufferHandler =
    *mut block2::Block<dyn Fn(NonNull<ProtocolObject<dyn MTLCommandBuffer>>)>;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLDispatchType(pub NSUInteger);
impl MTLDispatchType {
    #[doc(alias = "MTLDispatchTypeSerial")]
    pub const Serial: Self = Self(0);
    #[doc(alias = "MTLDispatchTypeConcurrent")]
    pub const Concurrent: Self = Self(1);
}

unsafe impl Encode for MTLDispatchType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLDispatchType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait MTLCommandBuffer: NSObjectProtocol + IsRetainable {
        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "MTLCommandQueue")]
        #[method_id(@__retain_semantics Other commandQueue)]
        unsafe fn commandQueue(&self) -> Retained<ProtocolObject<dyn MTLCommandQueue>>;

        #[method(retainedReferences)]
        unsafe fn retainedReferences(&self) -> bool;

        #[method(errorOptions)]
        unsafe fn errorOptions(&self) -> MTLCommandBufferErrorOption;

        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[method(kernelStartTime)]
        unsafe fn kernelStartTime(&self) -> CFTimeInterval;

        #[method(kernelEndTime)]
        unsafe fn kernelEndTime(&self) -> CFTimeInterval;

        #[cfg(feature = "MTLFunctionLog")]
        #[method_id(@__retain_semantics Other logs)]
        unsafe fn logs(&self) -> Retained<ProtocolObject<dyn MTLLogContainer>>;

        #[method(GPUStartTime)]
        unsafe fn GPUStartTime(&self) -> CFTimeInterval;

        #[method(GPUEndTime)]
        unsafe fn GPUEndTime(&self) -> CFTimeInterval;

        #[method(enqueue)]
        fn enqueue(&self);

        #[method(commit)]
        fn commit(&self);

        #[cfg(feature = "block2")]
        #[method(addScheduledHandler:)]
        unsafe fn addScheduledHandler(&self, block: MTLCommandBufferHandler);

        #[cfg(feature = "MTLDrawable")]
        #[method(presentDrawable:)]
        fn presentDrawable(&self, drawable: &ProtocolObject<dyn MTLDrawable>);

        #[cfg(feature = "MTLDrawable")]
        #[method(presentDrawable:atTime:)]
        unsafe fn presentDrawable_atTime(
            &self,
            drawable: &ProtocolObject<dyn MTLDrawable>,
            presentation_time: CFTimeInterval,
        );

        #[cfg(feature = "MTLDrawable")]
        #[method(presentDrawable:afterMinimumDuration:)]
        unsafe fn presentDrawable_afterMinimumDuration(
            &self,
            drawable: &ProtocolObject<dyn MTLDrawable>,
            duration: CFTimeInterval,
        );

        #[method(waitUntilScheduled)]
        fn waitUntilScheduled(&self);

        #[cfg(feature = "block2")]
        #[method(addCompletedHandler:)]
        unsafe fn addCompletedHandler(&self, block: MTLCommandBufferHandler);

        #[method(waitUntilCompleted)]
        unsafe fn waitUntilCompleted(&self);

        #[method(status)]
        fn status(&self) -> MTLCommandBufferStatus;

        #[method_id(@__retain_semantics Other error)]
        unsafe fn error(&self) -> Option<Retained<NSError>>;

        #[cfg(all(feature = "MTLBlitCommandEncoder", feature = "MTLCommandEncoder"))]
        #[method_id(@__retain_semantics Other blitCommandEncoder)]
        fn blitCommandEncoder(&self)
            -> Option<Retained<ProtocolObject<dyn MTLBlitCommandEncoder>>>;

        #[cfg(all(
            feature = "MTLCommandEncoder",
            feature = "MTLRenderCommandEncoder",
            feature = "MTLRenderPass"
        ))]
        #[method_id(@__retain_semantics Other renderCommandEncoderWithDescriptor:)]
        fn renderCommandEncoderWithDescriptor(
            &self,
            render_pass_descriptor: &MTLRenderPassDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>>;

        #[cfg(all(
            feature = "MTLCommandEncoder",
            feature = "MTLComputeCommandEncoder",
            feature = "MTLComputePass"
        ))]
        #[method_id(@__retain_semantics Other computeCommandEncoderWithDescriptor:)]
        unsafe fn computeCommandEncoderWithDescriptor(
            &self,
            compute_pass_descriptor: &MTLComputePassDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLComputeCommandEncoder>>>;

        #[cfg(all(
            feature = "MTLBlitCommandEncoder",
            feature = "MTLBlitPass",
            feature = "MTLCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other blitCommandEncoderWithDescriptor:)]
        unsafe fn blitCommandEncoderWithDescriptor(
            &self,
            blit_pass_descriptor: &MTLBlitPassDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLBlitCommandEncoder>>>;

        #[cfg(all(feature = "MTLCommandEncoder", feature = "MTLComputeCommandEncoder"))]
        #[method_id(@__retain_semantics Other computeCommandEncoder)]
        fn computeCommandEncoder(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLComputeCommandEncoder>>>;

        #[cfg(all(feature = "MTLCommandEncoder", feature = "MTLComputeCommandEncoder"))]
        #[method_id(@__retain_semantics Other computeCommandEncoderWithDispatchType:)]
        fn computeCommandEncoderWithDispatchType(
            &self,
            dispatch_type: MTLDispatchType,
        ) -> Option<Retained<ProtocolObject<dyn MTLComputeCommandEncoder>>>;

        #[cfg(feature = "MTLEvent")]
        #[method(encodeWaitForEvent:value:)]
        fn encodeWaitForEvent_value(&self, event: &ProtocolObject<dyn MTLEvent>, value: u64);

        #[cfg(feature = "MTLEvent")]
        #[method(encodeSignalEvent:value:)]
        fn encodeSignalEvent_value(&self, event: &ProtocolObject<dyn MTLEvent>, value: u64);

        #[cfg(all(
            feature = "MTLCommandEncoder",
            feature = "MTLParallelRenderCommandEncoder",
            feature = "MTLRenderPass"
        ))]
        #[method_id(@__retain_semantics Other parallelRenderCommandEncoderWithDescriptor:)]
        fn parallelRenderCommandEncoderWithDescriptor(
            &self,
            render_pass_descriptor: &MTLRenderPassDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLParallelRenderCommandEncoder>>>;

        #[cfg(all(
            feature = "MTLCommandEncoder",
            feature = "MTLResourceStateCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other resourceStateCommandEncoder)]
        unsafe fn resourceStateCommandEncoder(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLResourceStateCommandEncoder>>>;

        #[cfg(all(
            feature = "MTLCommandEncoder",
            feature = "MTLResourceStateCommandEncoder",
            feature = "MTLResourceStatePass"
        ))]
        #[method_id(@__retain_semantics Other resourceStateCommandEncoderWithDescriptor:)]
        unsafe fn resourceStateCommandEncoderWithDescriptor(
            &self,
            resource_state_pass_descriptor: &MTLResourceStatePassDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLResourceStateCommandEncoder>>>;

        #[cfg(all(
            feature = "MTLAccelerationStructureCommandEncoder",
            feature = "MTLCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other accelerationStructureCommandEncoder)]
        fn accelerationStructureCommandEncoder(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructureCommandEncoder>>>;

        #[cfg(all(
            feature = "MTLAccelerationStructureCommandEncoder",
            feature = "MTLCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other accelerationStructureCommandEncoderWithDescriptor:)]
        unsafe fn accelerationStructureCommandEncoderWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructurePassDescriptor,
        ) -> Retained<ProtocolObject<dyn MTLAccelerationStructureCommandEncoder>>;

        #[method(pushDebugGroup:)]
        fn pushDebugGroup(&self, string: &NSString);

        #[method(popDebugGroup)]
        fn popDebugGroup(&self);
    }

    unsafe impl ProtocolType for dyn MTLCommandBuffer {}
);
