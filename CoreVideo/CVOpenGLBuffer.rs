//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopenglbufferwidth?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLBufferWidth: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopenglbufferheight?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLBufferHeight: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopenglbuffertarget?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLBufferTarget: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopenglbufferinternalformat?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLBufferInternalFormat: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopenglbuffermaximummipmaplevel?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLBufferMaximumMipmapLevel: CFStringRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvopenglbufferref?language=objc)
#[cfg(all(feature = "CVBuffer", feature = "CVImageBuffer"))]
pub type CVOpenGLBufferRef = CVImageBufferRef;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLBufferGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(all(feature = "CVBuffer", feature = "CVImageBuffer"))]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLBufferRetain(buffer: CVOpenGLBufferRef) -> CVOpenGLBufferRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CVBuffer",
        feature = "CVImageBuffer",
        feature = "CVReturn",
        feature = "objc2-core-foundation"
    ))]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLBufferCreate(
        allocator: CFAllocatorRef,
        width: usize,
        height: usize,
        attributes: CFDictionaryRef,
        buffer_out: NonNull<CVOpenGLBufferRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CVBuffer",
        feature = "CVImageBuffer",
        feature = "objc2-core-foundation"
    ))]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLBufferGetAttributes(open_gl_buffer: CVOpenGLBufferRef) -> CFDictionaryRef;
}
