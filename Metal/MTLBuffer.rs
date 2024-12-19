//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlbuffer?language=objc)
    #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
    pub unsafe trait MTLBuffer: MTLResource {
        #[method(length)]
        fn length(&self) -> NSUInteger;

        #[method(contents)]
        fn contents(&self) -> NonNull<c_void>;

        #[method(didModifyRange:)]
        fn didModifyRange(&self, range: NSRange);

        #[cfg(feature = "MTLTexture")]
        #[method_id(@__retain_semantics New newTextureWithDescriptor:offset:bytesPerRow:)]
        fn newTextureWithDescriptor_offset_bytesPerRow(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: NSUInteger,
            bytes_per_row: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[method(addDebugMarker:range:)]
        fn addDebugMarker_range(&self, marker: &NSString, range: NSRange);

        #[method(removeAllDebugMarkers)]
        fn removeAllDebugMarkers(&self);

        #[method_id(@__retain_semantics Other remoteStorageBuffer)]
        fn remoteStorageBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics New newRemoteBufferViewForDevice:)]
        fn newRemoteBufferViewForDevice(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[method(gpuAddress)]
        fn gpuAddress(&self) -> u64;
    }

    #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
    unsafe impl ProtocolType for dyn MTLBuffer {}
);
