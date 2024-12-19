//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctlineref?language=objc)
pub type CTLineRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctlineboundsoptions?language=objc)
// NS_OPTIONS
#[cfg(feature = "objc2-core-foundation")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTLineBoundsOptions(pub CFOptionFlags);
#[cfg(feature = "objc2-core-foundation")]
bitflags::bitflags! {
    impl CTLineBoundsOptions: CFOptionFlags {
        const kCTLineBoundsExcludeTypographicLeading = 1<<0;
        const kCTLineBoundsExcludeTypographicShifts = 1<<1;
        const kCTLineBoundsUseHangingPunctuation = 1<<2;
        const kCTLineBoundsUseGlyphPathBounds = 1<<3;
        const kCTLineBoundsUseOpticalBounds = 1<<4;
        const kCTLineBoundsIncludeLanguageExtents = 1<<5;
    }
}

#[cfg(all(feature = "objc2", feature = "objc2-core-foundation"))]
unsafe impl Encode for CTLineBoundsOptions {
    const ENCODING: Encoding = CFOptionFlags::ENCODING;
}

#[cfg(all(feature = "objc2", feature = "objc2-core-foundation"))]
unsafe impl RefEncode for CTLineBoundsOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctlinetruncationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTLineTruncationType(pub u32);
impl CTLineTruncationType {
    pub const kCTLineTruncationStart: Self = Self(0);
    pub const kCTLineTruncationEnd: Self = Self(1);
    pub const kCTLineTruncationMiddle: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CTLineTruncationType {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CTLineTruncationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineCreateWithAttributedString(attr_string: CFAttributedStringRef) -> CTLineRef;
}

extern "C-unwind" {
    pub fn CTLineCreateTruncatedLine(
        line: CTLineRef,
        width: c_double,
        truncation_type: CTLineTruncationType,
        truncation_token: CTLineRef,
    ) -> CTLineRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineCreateJustifiedLine(
        line: CTLineRef,
        justification_factor: CGFloat,
        justification_width: c_double,
    ) -> CTLineRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetGlyphCount(line: CTLineRef) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetGlyphRuns(line: CTLineRef) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetStringRange(line: CTLineRef) -> CFRange;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetPenOffsetForFlush(
        line: CTLineRef,
        flush_factor: CGFloat,
        flush_width: c_double,
    ) -> c_double;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-graphics")]
    pub fn CTLineDraw(line: CTLineRef, context: CGContextRef);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetTypographicBounds(
        line: CTLineRef,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> c_double;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetBoundsWithOptions(line: CTLineRef, options: CTLineBoundsOptions) -> CGRect;
}

extern "C-unwind" {
    pub fn CTLineGetTrailingWhitespaceWidth(line: CTLineRef) -> c_double;
}

extern "C-unwind" {
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
    pub fn CTLineGetImageBounds(line: CTLineRef, context: CGContextRef) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetStringIndexForPosition(line: CTLineRef, position: CGPoint) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetOffsetForStringIndex(
        line: CTLineRef,
        char_index: CFIndex,
        secondary_offset: *mut CGFloat,
    ) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
    pub fn CTLineEnumerateCaretOffsets(
        line: CTLineRef,
        block: &block2::Block<dyn Fn(c_double, CFIndex, bool, NonNull<bool>)>,
    );
}
