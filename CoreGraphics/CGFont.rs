//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgfontref?language=objc)
pub type CGFontRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgfontindex?language=objc)
pub type CGFontIndex = c_ushort;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgglyph?language=objc)
pub type CGGlyph = CGFontIndex;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgfontpostscriptformat?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGFontPostScriptFormat(pub i32);
impl CGFontPostScriptFormat {
    pub const kCGFontPostScriptFormatType1: Self = Self(1);
    pub const kCGFontPostScriptFormatType3: Self = Self(3);
    pub const kCGFontPostScriptFormatType42: Self = Self(42);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGFontPostScriptFormat {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGFontPostScriptFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgglyphmax?language=objc)
pub static kCGGlyphMax: CGFontIndex = kCGFontIndexMax;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[deprecated = "No longer supported"]
    pub fn CGFontCreateWithPlatformFont(platform_font_reference: *mut c_void) -> CGFontRef;
}

extern "C-unwind" {
    #[cfg(feature = "CGDataProvider")]
    pub fn CGFontCreateWithDataProvider(provider: CGDataProviderRef) -> CGFontRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontCreateWithFontName(name: CFStringRef) -> CGFontRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontCreateCopyWithVariations(
        font: CGFontRef,
        variations: CFDictionaryRef,
    ) -> CGFontRef;
}

extern "C-unwind" {
    pub fn CGFontRetain(font: CGFontRef) -> CGFontRef;
}

extern "C-unwind" {
    pub fn CGFontRelease(font: CGFontRef);
}

extern "C-unwind" {
    pub fn CGFontGetNumberOfGlyphs(font: CGFontRef) -> usize;
}

extern "C-unwind" {
    pub fn CGFontGetUnitsPerEm(font: CGFontRef) -> c_int;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontCopyPostScriptName(font: CGFontRef) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontCopyFullName(font: CGFontRef) -> CFStringRef;
}

extern "C-unwind" {
    pub fn CGFontGetAscent(font: CGFontRef) -> c_int;
}

extern "C-unwind" {
    pub fn CGFontGetDescent(font: CGFontRef) -> c_int;
}

extern "C-unwind" {
    pub fn CGFontGetLeading(font: CGFontRef) -> c_int;
}

extern "C-unwind" {
    pub fn CGFontGetCapHeight(font: CGFontRef) -> c_int;
}

extern "C-unwind" {
    pub fn CGFontGetXHeight(font: CGFontRef) -> c_int;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontGetFontBBox(font: CGFontRef) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontGetItalicAngle(font: CGFontRef) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontGetStemV(font: CGFontRef) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontCopyVariationAxes(font: CGFontRef) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontCopyVariations(font: CGFontRef) -> CFDictionaryRef;
}

extern "C-unwind" {
    pub fn CGFontGetGlyphAdvances(
        font: CGFontRef,
        glyphs: NonNull<CGGlyph>,
        count: usize,
        advances: NonNull<c_int>,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontGetGlyphBBoxes(
        font: CGFontRef,
        glyphs: NonNull<CGGlyph>,
        count: usize,
        bboxes: NonNull<CGRect>,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontGetGlyphWithGlyphName(font: CGFontRef, name: CFStringRef) -> CGGlyph;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontCopyGlyphNameForGlyph(font: CGFontRef, glyph: CGGlyph) -> CFStringRef;
}

extern "C-unwind" {
    pub fn CGFontCanCreatePostScriptSubset(font: CGFontRef, format: CGFontPostScriptFormat)
        -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontCopyTableTags(font: CGFontRef) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGFontCopyTableForTag(font: CGFontRef, tag: u32) -> CFDataRef;
}

extern "C" {
    /// * Keys for the font variation axis dictionary. **
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgfontvariationaxisname?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGFontVariationAxisName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgfontvariationaxisminvalue?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGFontVariationAxisMinValue: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgfontvariationaxismaxvalue?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGFontVariationAxisMaxValue: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgfontvariationaxisdefaultvalue?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGFontVariationAxisDefaultValue: CFStringRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgglyphdeprecatedenum?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGGlyphDeprecatedEnum(pub i32);
impl CGGlyphDeprecatedEnum {
    pub const CGGlyphMin: Self = Self(0);
    pub const CGGlyphMax: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGGlyphDeprecatedEnum {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGGlyphDeprecatedEnum {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
