//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-metal")]
#[cfg(not(target_os = "watchos"))]
use objc2_metal::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvmetalbuffercachemaximumbufferagekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVMetalBufferCacheMaximumBufferAgeKey: CFStringRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvmetalbuffercacheref?language=objc)
pub type CVMetalBufferCacheRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVMetalBufferCacheGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    /// Creates a new Buffer Cache.
    ///
    /// Parameter `allocator`: The CFAllocatorRef to use for allocating the cache.  May be NULL.
    ///
    /// Parameter `cacheAttributes`: A CFDictionaryRef containing the attributes of the cache itself. May be NULL.
    ///
    /// Parameter `metalDevice`: The Metal device for which the buffer objects will be created.
    ///
    /// Parameter `cacheOut`: The newly created buffer cache will be placed here
    ///
    /// Returns: Returns kCVReturnSuccess on success
    #[cfg(all(
        feature = "CVReturn",
        feature = "objc2",
        feature = "objc2-core-foundation",
        feature = "objc2-metal"
    ))]
    #[cfg(not(target_os = "watchos"))]
    pub fn CVMetalBufferCacheCreate(
        allocator: CFAllocatorRef,
        cache_attributes: CFDictionaryRef,
        metal_device: &ProtocolObject<dyn MTLDevice>,
        cache_out: NonNull<CVMetalBufferCacheRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    /// Creates a CVMetalBuffer object from an existing CVImageBuffer
    ///
    /// Parameter `allocator`: The CFAllocatorRef to use for allocating the CVMetalBuffer object. May be NULL.
    ///
    /// Parameter `bufferCache`: The buffer cache object that will manage the buffer.
    ///
    /// Parameter `buffer`: The CVImageBuffer that you want to create a CVMetalBuffer from.
    ///
    /// Parameter `bufferOut`: The newly created buffer object will be placed here.
    ///
    /// Returns: Returns kCVReturnSuccess on success
    ///
    /// Creates or returns a cached CVMetalBuffer object mapped to the CVImageBuffer.
    /// This creates a live binding between the CVImageBuffer and underlying CVMetalBuffer buffer object.
    ///
    /// IMPORTANT NOTE: Clients should retain CVMetalBuffer objects until they are done using the images in them.
    /// Retaining a CVMetalBuffer is your way to indicate that you're still using the image in the buffer, and that it should not be recycled yet.
    #[cfg(all(
        feature = "CVBuffer",
        feature = "CVImageBuffer",
        feature = "CVMetalBuffer",
        feature = "CVReturn",
        feature = "objc2-core-foundation"
    ))]
    pub fn CVMetalBufferCacheCreateBufferFromImage(
        allocator: CFAllocatorRef,
        buffer_cache: CVMetalBufferCacheRef,
        image_buffer: CVImageBufferRef,
        buffer_out: NonNull<CVMetalBufferRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    /// Performs internal housekeeping/recycling operations
    ///
    /// This call must be made periodically to give the buffer cache a chance to do internal housekeeping operations.
    ///
    /// Parameter `bufferCache`: The buffer cache object to flush
    ///
    /// Parameter `options`: Currently unused, set to 0.
    #[cfg(feature = "CVBase")]
    pub fn CVMetalBufferCacheFlush(buffer_cache: CVMetalBufferCacheRef, options: CVOptionFlags);
}
