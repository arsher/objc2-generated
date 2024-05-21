//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIOStatus(pub NSInteger);
impl MTLIOStatus {
    #[doc(alias = "MTLIOStatusPending")]
    pub const Pending: Self = Self(0);
    #[doc(alias = "MTLIOStatusCancelled")]
    pub const Cancelled: Self = Self(1);
    #[doc(alias = "MTLIOStatusError")]
    pub const Error: Self = Self(2);
    #[doc(alias = "MTLIOStatusComplete")]
    pub const Complete: Self = Self(3);
}

unsafe impl Encode for MTLIOStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLIOStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "block2")]
pub type MTLIOCommandBufferHandler =
    *mut block2::Block<dyn Fn(NonNull<ProtocolObject<dyn MTLIOCommandBuffer>>)>;

extern_protocol!(
    pub unsafe trait MTLIOCommandBuffer: NSObjectProtocol + IsRetainable {
        #[cfg(feature = "block2")]
        #[method(addCompletedHandler:)]
        unsafe fn addCompletedHandler(&self, block: MTLIOCommandBufferHandler);

        #[cfg(feature = "MTLIOCommandQueue")]
        #[method(loadBytes:size:sourceHandle:sourceHandleOffset:)]
        unsafe fn loadBytes_size_sourceHandle_sourceHandleOffset(
            &self,
            pointer: NonNull<c_void>,
            size: NSUInteger,
            source_handle: &ProtocolObject<dyn MTLIOFileHandle>,
            source_handle_offset: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLBuffer",
            feature = "MTLIOCommandQueue",
            feature = "MTLResource"
        ))]
        #[method(loadBuffer:offset:size:sourceHandle:sourceHandleOffset:)]
        unsafe fn loadBuffer_offset_size_sourceHandle_sourceHandleOffset(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            size: NSUInteger,
            source_handle: &ProtocolObject<dyn MTLIOFileHandle>,
            source_handle_offset: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLIOCommandQueue",
            feature = "MTLResource",
            feature = "MTLTexture",
            feature = "MTLTypes"
        ))]
        #[method(loadTexture:slice:level:size:sourceBytesPerRow:sourceBytesPerImage:destinationOrigin:sourceHandle:sourceHandleOffset:)]
        unsafe fn loadTexture_slice_level_size_sourceBytesPerRow_sourceBytesPerImage_destinationOrigin_sourceHandle_sourceHandleOffset(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            slice: NSUInteger,
            level: NSUInteger,
            size: MTLSize,
            source_bytes_per_row: NSUInteger,
            source_bytes_per_image: NSUInteger,
            destination_origin: MTLOrigin,
            source_handle: &ProtocolObject<dyn MTLIOFileHandle>,
            source_handle_offset: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(copyStatusToBuffer:offset:)]
        unsafe fn copyStatusToBuffer_offset(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
        );

        #[method(commit)]
        unsafe fn commit(&self);

        #[method(waitUntilCompleted)]
        unsafe fn waitUntilCompleted(&self);

        #[method(tryCancel)]
        unsafe fn tryCancel(&self);

        #[method(addBarrier)]
        unsafe fn addBarrier(&self);

        #[method(pushDebugGroup:)]
        unsafe fn pushDebugGroup(&self, string: &NSString);

        #[method(popDebugGroup)]
        unsafe fn popDebugGroup(&self);

        #[method(enqueue)]
        unsafe fn enqueue(&self);

        #[cfg(feature = "MTLEvent")]
        #[method(waitForEvent:value:)]
        unsafe fn waitForEvent_value(&self, event: &ProtocolObject<dyn MTLSharedEvent>, value: u64);

        #[cfg(feature = "MTLEvent")]
        #[method(signalEvent:value:)]
        unsafe fn signalEvent_value(&self, event: &ProtocolObject<dyn MTLSharedEvent>, value: u64);

        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method(status)]
        unsafe fn status(&self) -> MTLIOStatus;

        #[method_id(@__retain_semantics Other error)]
        unsafe fn error(&self) -> Option<Retained<NSError>>;
    }

    unsafe impl ProtocolType for dyn MTLIOCommandBuffer {}
);
