//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgimageref?language=objc)
pub type CGImageRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgimagealphainfo?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGImageAlphaInfo(pub u32);
impl CGImageAlphaInfo {
    pub const kCGImageAlphaNone: Self = Self(0);
    pub const kCGImageAlphaPremultipliedLast: Self = Self(1);
    pub const kCGImageAlphaPremultipliedFirst: Self = Self(2);
    pub const kCGImageAlphaLast: Self = Self(3);
    pub const kCGImageAlphaFirst: Self = Self(4);
    pub const kCGImageAlphaNoneSkipLast: Self = Self(5);
    pub const kCGImageAlphaNoneSkipFirst: Self = Self(6);
    pub const kCGImageAlphaOnly: Self = Self(7);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGImageAlphaInfo {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGImageAlphaInfo {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgimagebyteorderinfo?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGImageByteOrderInfo(pub u32);
impl CGImageByteOrderInfo {
    pub const kCGImageByteOrderMask: Self = Self(0x7000);
    pub const kCGImageByteOrderDefault: Self = Self(0 << 12);
    pub const kCGImageByteOrder16Little: Self = Self(1 << 12);
    pub const kCGImageByteOrder32Little: Self = Self(2 << 12);
    pub const kCGImageByteOrder16Big: Self = Self(3 << 12);
    pub const kCGImageByteOrder32Big: Self = Self(4 << 12);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGImageByteOrderInfo {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGImageByteOrderInfo {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgimagepixelformatinfo?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGImagePixelFormatInfo(pub u32);
impl CGImagePixelFormatInfo {
    pub const kCGImagePixelFormatMask: Self = Self(0xF0000);
    pub const kCGImagePixelFormatPacked: Self = Self(0 << 16);
    pub const kCGImagePixelFormatRGB555: Self = Self(1 << 16);
    pub const kCGImagePixelFormatRGB565: Self = Self(2 << 16);
    pub const kCGImagePixelFormatRGB101010: Self = Self(3 << 16);
    pub const kCGImagePixelFormatRGBCIF10: Self = Self(4 << 16);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGImagePixelFormatInfo {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGImagePixelFormatInfo {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgbitmapinfo?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGBitmapInfo(pub u32);
bitflags::bitflags! {
    impl CGBitmapInfo: u32 {
        const kCGBitmapAlphaInfoMask = 0x1F;
        const kCGBitmapFloatInfoMask = 0xF00;
        const kCGBitmapFloatComponents = 1<<8;
        const kCGBitmapByteOrderMask = CGImageByteOrderInfo::kCGImageByteOrderMask.0;
        const kCGBitmapByteOrderDefault = CGImageByteOrderInfo::kCGImageByteOrderDefault.0;
        const kCGBitmapByteOrder16Little = CGImageByteOrderInfo::kCGImageByteOrder16Little.0;
        const kCGBitmapByteOrder32Little = CGImageByteOrderInfo::kCGImageByteOrder32Little.0;
        const kCGBitmapByteOrder16Big = CGImageByteOrderInfo::kCGImageByteOrder16Big.0;
        const kCGBitmapByteOrder32Big = CGImageByteOrderInfo::kCGImageByteOrder32Big.0;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGBitmapInfo {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGBitmapInfo {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgbitmapbyteorder16host?language=objc)
pub static kCGBitmapByteOrder16Host: CGBitmapInfo =
    CGBitmapInfo(CGBitmapInfo::kCGBitmapByteOrder16Little.0);

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgbitmapbyteorder32host?language=objc)
pub static kCGBitmapByteOrder32Host: CGBitmapInfo =
    CGBitmapInfo(CGBitmapInfo::kCGBitmapByteOrder32Little.0);

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGImageGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CGColorSpace",
        feature = "CGDataProvider",
        feature = "objc2-core-foundation"
    ))]
    pub fn CGImageCreate(
        width: usize,
        height: usize,
        bits_per_component: usize,
        bits_per_pixel: usize,
        bytes_per_row: usize,
        space: CGColorSpaceRef,
        bitmap_info: CGBitmapInfo,
        provider: CGDataProviderRef,
        decode: *mut CGFloat,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDataProvider", feature = "objc2-core-foundation"))]
    pub fn CGImageMaskCreate(
        width: usize,
        height: usize,
        bits_per_component: usize,
        bits_per_pixel: usize,
        bytes_per_row: usize,
        provider: CGDataProviderRef,
        decode: *mut CGFloat,
        should_interpolate: bool,
    ) -> CGImageRef;
}

extern "C-unwind" {
    pub fn CGImageCreateCopy(image: CGImageRef) -> CGImageRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CGColorSpace",
        feature = "CGDataProvider",
        feature = "objc2-core-foundation"
    ))]
    pub fn CGImageCreateWithJPEGDataProvider(
        source: CGDataProviderRef,
        decode: *mut CGFloat,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CGColorSpace",
        feature = "CGDataProvider",
        feature = "objc2-core-foundation"
    ))]
    pub fn CGImageCreateWithPNGDataProvider(
        source: CGDataProviderRef,
        decode: *mut CGFloat,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGImageCreateWithImageInRect(image: CGImageRef, rect: CGRect) -> CGImageRef;
}

extern "C-unwind" {
    pub fn CGImageCreateWithMask(image: CGImageRef, mask: CGImageRef) -> CGImageRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGImageCreateWithMaskingColors(
        image: CGImageRef,
        components: *mut CGFloat,
    ) -> CGImageRef;
}

extern "C-unwind" {
    #[cfg(feature = "CGColorSpace")]
    pub fn CGImageCreateCopyWithColorSpace(image: CGImageRef, space: CGColorSpaceRef)
        -> CGImageRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CGColorSpace",
        feature = "CGDataProvider",
        feature = "objc2-core-foundation"
    ))]
    pub fn CGImageCreateWithContentHeadroom(
        headroom: c_float,
        width: usize,
        height: usize,
        bits_per_component: usize,
        bits_per_pixel: usize,
        bytes_per_row: usize,
        space: CGColorSpaceRef,
        bitmap_info: CGBitmapInfo,
        provider: CGDataProviderRef,
        decode: *mut CGFloat,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
}

extern "C-unwind" {
    pub fn CGImageCreateCopyWithContentHeadroom(headroom: c_float, image: CGImageRef)
        -> CGImageRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdefaulthdrimagecontentheadroom?language=objc)
    pub static kCGDefaultHDRImageContentHeadroom: c_float;
}

extern "C-unwind" {
    pub fn CGImageGetContentHeadroom(image: CGImageRef) -> c_float;
}

extern "C-unwind" {
    pub fn CGImageRetain(image: CGImageRef) -> CGImageRef;
}

extern "C-unwind" {
    pub fn CGImageRelease(image: CGImageRef);
}

extern "C-unwind" {
    pub fn CGImageIsMask(image: CGImageRef) -> bool;
}

extern "C-unwind" {
    pub fn CGImageGetWidth(image: CGImageRef) -> usize;
}

extern "C-unwind" {
    pub fn CGImageGetHeight(image: CGImageRef) -> usize;
}

extern "C-unwind" {
    pub fn CGImageGetBitsPerComponent(image: CGImageRef) -> usize;
}

extern "C-unwind" {
    pub fn CGImageGetBitsPerPixel(image: CGImageRef) -> usize;
}

extern "C-unwind" {
    pub fn CGImageGetBytesPerRow(image: CGImageRef) -> usize;
}

extern "C-unwind" {
    #[cfg(feature = "CGColorSpace")]
    pub fn CGImageGetColorSpace(image: CGImageRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGImageGetAlphaInfo(image: CGImageRef) -> CGImageAlphaInfo;
}

extern "C-unwind" {
    #[cfg(feature = "CGDataProvider")]
    pub fn CGImageGetDataProvider(image: CGImageRef) -> CGDataProviderRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGImageGetDecode(image: CGImageRef) -> *mut CGFloat;
}

extern "C-unwind" {
    pub fn CGImageGetShouldInterpolate(image: CGImageRef) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGColorSpace")]
    pub fn CGImageGetRenderingIntent(image: CGImageRef) -> CGColorRenderingIntent;
}

extern "C-unwind" {
    pub fn CGImageGetBitmapInfo(image: CGImageRef) -> CGBitmapInfo;
}

extern "C-unwind" {
    pub fn CGImageGetByteOrderInfo(image: CGImageRef) -> CGImageByteOrderInfo;
}

extern "C-unwind" {
    pub fn CGImageGetPixelFormatInfo(image: CGImageRef) -> CGImagePixelFormatInfo;
}

extern "C-unwind" {
    pub fn CGImageShouldToneMap(image: CGImageRef) -> bool;
}

extern "C-unwind" {
    pub fn CGImageContainsImageSpecificToneMappingMetadata(image: CGImageRef) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGImageGetUTType(image: CGImageRef) -> CFStringRef;
}
