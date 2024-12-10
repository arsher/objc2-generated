//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvopengltexturecacheref?language=objc)
pub type CVOpenGLTextureCacheRef = *mut c_void;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopengltexturecachechromasamplingmodekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLTextureCacheChromaSamplingModeKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopengltexturecachechromasamplingmodeautomatic?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLTextureCacheChromaSamplingModeAutomatic: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopengltexturecachechromasamplingmodehighestquality?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLTextureCacheChromaSamplingModeHighestQuality: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopengltexturecachechromasamplingmodebestperformance?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLTextureCacheChromaSamplingModeBestPerformance: CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLTextureCacheGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLTextureCacheRetain(
        texture_cache: CVOpenGLTextureCacheRef,
    ) -> CVOpenGLTextureCacheRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CVBuffer",
        feature = "CVImageBuffer",
        feature = "CVOpenGLTexture",
        feature = "CVReturn",
        feature = "objc2-core-foundation"
    ))]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        texture_cache: CVOpenGLTextureCacheRef,
        source_image: CVImageBufferRef,
        attributes: CFDictionaryRef,
        texture_out: NonNull<CVOpenGLTextureRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(feature = "CVBase")]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLTextureCacheFlush(
        texture_cache: CVOpenGLTextureCacheRef,
        options: CVOptionFlags,
    );
}