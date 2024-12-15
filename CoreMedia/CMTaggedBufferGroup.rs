//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtaggedbuffergrouperror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMTaggedBufferGroupError(pub OSStatus);
impl CMTaggedBufferGroupError {
    pub const kCMTaggedBufferGroupError_ParamErr: Self = Self(-15780);
    pub const kCMTaggedBufferGroupError_AllocationFailed: Self = Self(-15781);
    pub const kCMTaggedBufferGroupError_InternalError: Self = Self(-15782);
}

unsafe impl Encode for CMTaggedBufferGroupError {
    const ENCODING: Encoding = OSStatus::ENCODING;
}

unsafe impl RefEncode for CMTaggedBufferGroupError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtaggedbuffergroupref?language=objc)
pub type CMTaggedBufferGroupRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTaggedBufferGroupGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTaggedBufferGroupCreate(
        allocator: CFAllocatorRef,
        tag_collections: CFArrayRef,
        buffers: CFArrayRef,
        group_out: NonNull<CMTaggedBufferGroupRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTaggedBufferGroupCreateCombined(
        allocator: CFAllocatorRef,
        tagged_buffer_groups: CFArrayRef,
        group_out: NonNull<CMTaggedBufferGroupRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMBase", feature = "objc2-core-foundation"))]
    pub fn CMTaggedBufferGroupGetCount(group: CMTaggedBufferGroupRef) -> CMItemCount;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMTagCollection", feature = "objc2-core-foundation"))]
    pub fn CMTaggedBufferGroupGetTagCollectionAtIndex(
        group: CMTaggedBufferGroupRef,
        index: CFIndex,
    ) -> CMTagCollectionRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-video"))]
    pub fn CMTaggedBufferGroupGetCVPixelBufferAtIndex(
        group: CMTaggedBufferGroupRef,
        index: CFIndex,
    ) -> CVPixelBufferRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CMTag",
        feature = "objc2-core-foundation",
        feature = "objc2-core-video"
    ))]
    pub fn CMTaggedBufferGroupGetCVPixelBufferForTag(
        group: CMTaggedBufferGroupRef,
        tag: CMTag,
        index_out: *mut CFIndex,
    ) -> CVPixelBufferRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CMTagCollection",
        feature = "objc2-core-foundation",
        feature = "objc2-core-video"
    ))]
    pub fn CMTaggedBufferGroupGetCVPixelBufferForTagCollection(
        group: CMTaggedBufferGroupRef,
        tag_collection: CMTagCollectionRef,
        index_out: *mut CFIndex,
    ) -> CVPixelBufferRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMSampleBuffer", feature = "objc2-core-foundation"))]
    pub fn CMTaggedBufferGroupGetCMSampleBufferAtIndex(
        group: CMTaggedBufferGroupRef,
        index: CFIndex,
    ) -> CMSampleBufferRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CMSampleBuffer",
        feature = "CMTag",
        feature = "objc2-core-foundation"
    ))]
    pub fn CMTaggedBufferGroupGetCMSampleBufferForTag(
        group: CMTaggedBufferGroupRef,
        tag: CMTag,
        index_out: *mut CFIndex,
    ) -> CMSampleBufferRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CMSampleBuffer",
        feature = "CMTagCollection",
        feature = "objc2-core-foundation"
    ))]
    pub fn CMTaggedBufferGroupGetCMSampleBufferForTagCollection(
        group: CMTaggedBufferGroupRef,
        tag_collection: CMTagCollectionRef,
        index_out: *mut CFIndex,
    ) -> CMSampleBufferRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CMBase",
        feature = "CMTagCollection",
        feature = "objc2-core-foundation"
    ))]
    pub fn CMTaggedBufferGroupGetNumberOfMatchesForTagCollection(
        group: CMTaggedBufferGroupRef,
        tag_collection: CMTagCollectionRef,
    ) -> CMItemCount;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMFormatDescription", feature = "objc2-core-foundation"))]
    pub fn CMTaggedBufferGroupFormatDescriptionCreateForTaggedBufferGroup(
        allocator: CFAllocatorRef,
        tagged_buffer_group: CMTaggedBufferGroupRef,
        format_description_out: NonNull<CMTaggedBufferGroupFormatDescriptionRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "CMFormatDescription")]
    pub fn CMTaggedBufferGroupFormatDescriptionMatchesTaggedBufferGroup(
        desc: CMTaggedBufferGroupFormatDescriptionRef,
        tagged_buffer_group: CMTaggedBufferGroupRef,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CMFormatDescription",
        feature = "CMSampleBuffer",
        feature = "CMTime",
        feature = "objc2-core-foundation"
    ))]
    pub fn CMSampleBufferCreateForTaggedBufferGroup(
        allocator: CFAllocatorRef,
        tagged_buffer_group: CMTaggedBufferGroupRef,
        sbuf_pts: CMTime,
        sbuf_duration: CMTime,
        format_description: CMTaggedBufferGroupFormatDescriptionRef,
        s_buf_out: NonNull<CMSampleBufferRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "CMSampleBuffer")]
    pub fn CMSampleBufferGetTaggedBufferGroup(sbuf: CMSampleBufferRef) -> CMTaggedBufferGroupRef;
}