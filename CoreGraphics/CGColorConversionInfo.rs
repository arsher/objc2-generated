//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgcolorconversioninforef?language=objc)
pub type CGColorConversionInfoRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorConversionInfoGetTypeID() -> CFTypeID;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgcolorconversioninfotransformtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGColorConversionInfoTransformType(pub u32);
impl CGColorConversionInfoTransformType {
    pub const kCGColorConversionTransformFromSpace: Self = Self(0);
    pub const kCGColorConversionTransformToSpace: Self = Self(1);
    pub const kCGColorConversionTransformApplySpace: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGColorConversionInfoTransformType {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGColorConversionInfoTransformType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "CGColorSpace")]
    pub fn CGColorConversionInfoCreate(
        src: CGColorSpaceRef,
        dst: CGColorSpaceRef,
    ) -> CGColorConversionInfoRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGColorSpace", feature = "objc2-core-foundation"))]
    pub fn CGColorConversionInfoCreateWithOptions(
        src: CGColorSpaceRef,
        dst: CGColorSpaceRef,
        options: CFDictionaryRef,
    ) -> CGColorConversionInfoRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CGColorSpace",
        feature = "CGToneMapping",
        feature = "objc2-core-foundation"
    ))]
    pub fn CGColorConversionInfoCreateForToneMapping(
        from: CGColorSpaceRef,
        source_headroom: c_float,
        to: CGColorSpaceRef,
        target_headroom: c_float,
        method: CGToneMapping,
        options: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> CGColorConversionInfoRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgcolorbufferformat?language=objc)
#[cfg(feature = "CGImage")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGColorBufferFormat {
    pub version: u32,
    pub bitmapInfo: CGBitmapInfo,
    pub bitsPerComponent: usize,
    pub bitsPerPixel: usize,
    pub bytesPerRow: usize,
}

#[cfg(all(feature = "CGImage", feature = "objc2"))]
unsafe impl Encode for CGColorBufferFormat {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u32>::ENCODING,
            <CGBitmapInfo>::ENCODING,
            <usize>::ENCODING,
            <usize>::ENCODING,
            <usize>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "CGImage", feature = "objc2"))]
unsafe impl RefEncode for CGColorBufferFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(all(feature = "CGImage", feature = "objc2-core-foundation"))]
    pub fn CGColorConversionInfoConvertData(
        info: CGColorConversionInfoRef,
        width: usize,
        height: usize,
        dst_data: NonNull<c_void>,
        dst_format: CGColorBufferFormat,
        src_data: NonNull<c_void>,
        src_format: CGColorBufferFormat,
        options: CFDictionaryRef,
    ) -> bool;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorconversionblackpointcompensation?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorConversionBlackPointCompensation: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorconversiontrcsize?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorConversionTRCSize: CFStringRef;
}
