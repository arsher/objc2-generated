//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvbufferpropagatedattachmentskey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVBufferPropagatedAttachmentsKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvbuffernonpropagatedattachmentskey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVBufferNonPropagatedAttachmentsKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvbuffermovietimekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVBufferMovieTimeKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvbuffertimevaluekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVBufferTimeValueKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvbuffertimescalekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVBufferTimeScaleKey: CFStringRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvattachmentmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CVAttachmentMode(pub u32);
impl CVAttachmentMode {
    pub const kCVAttachmentMode_ShouldNotPropagate: Self = Self(0);
    pub const kCVAttachmentMode_ShouldPropagate: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CVAttachmentMode {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CVAttachmentMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvbufferref?language=objc)
pub type CVBufferRef = *mut c_void;

extern "C-unwind" {
    pub fn CVBufferRetain(buffer: CVBufferRef) -> CVBufferRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVBufferSetAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        value: CFTypeRef,
        attachment_mode: CVAttachmentMode,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated]
    pub fn CVBufferGetAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        attachment_mode: *mut CVAttachmentMode,
    ) -> CFTypeRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVBufferRemoveAttachment(buffer: CVBufferRef, key: CFStringRef);
}

extern "C-unwind" {
    pub fn CVBufferRemoveAllAttachments(buffer: CVBufferRef);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated]
    pub fn CVBufferGetAttachments(
        buffer: CVBufferRef,
        attachment_mode: CVAttachmentMode,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVBufferSetAttachments(
        buffer: CVBufferRef,
        the_attachments: CFDictionaryRef,
        attachment_mode: CVAttachmentMode,
    );
}

extern "C-unwind" {
    pub fn CVBufferPropagateAttachments(
        source_buffer: CVBufferRef,
        destination_buffer: CVBufferRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVBufferCopyAttachments(
        buffer: CVBufferRef,
        attachment_mode: CVAttachmentMode,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVBufferCopyAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        attachment_mode: *mut CVAttachmentMode,
    ) -> CFTypeRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVBufferHasAttachment(buffer: CVBufferRef, key: CFStringRef) -> Boolean;
}
