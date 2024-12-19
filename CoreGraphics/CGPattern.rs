//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpatternref?language=objc)
pub type CGPatternRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpatterntiling?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGPatternTiling(pub i32);
impl CGPatternTiling {
    pub const kCGPatternTilingNoDistortion: Self = Self(0);
    pub const kCGPatternTilingConstantSpacingMinimalDistortion: Self = Self(1);
    pub const kCGPatternTilingConstantSpacing: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGPatternTiling {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGPatternTiling {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpatterndrawpatterncallback?language=objc)
#[cfg(feature = "CGContext")]
pub type CGPatternDrawPatternCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void, CGContextRef)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpatternreleaseinfocallback?language=objc)
pub type CGPatternReleaseInfoCallback = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpatterncallbacks?language=objc)
#[cfg(feature = "CGContext")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGPatternCallbacks {
    pub version: c_uint,
    pub drawPattern: CGPatternDrawPatternCallback,
    pub releaseInfo: CGPatternReleaseInfoCallback,
}

#[cfg(all(feature = "CGContext", feature = "objc2"))]
unsafe impl Encode for CGPatternCallbacks {
    const ENCODING: Encoding = Encoding::Struct(
        "CGPatternCallbacks",
        &[
            <c_uint>::ENCODING,
            <CGPatternDrawPatternCallback>::ENCODING,
            <CGPatternReleaseInfoCallback>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "CGContext", feature = "objc2"))]
unsafe impl RefEncode for CGPatternCallbacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPatternGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "objc2-core-foundation"))]
    pub fn CGPatternCreate(
        info: *mut c_void,
        bounds: CGRect,
        matrix: CGAffineTransform,
        x_step: CGFloat,
        y_step: CGFloat,
        tiling: CGPatternTiling,
        is_colored: bool,
        callbacks: *mut CGPatternCallbacks,
    ) -> CGPatternRef;
}

extern "C-unwind" {
    pub fn CGPatternRetain(pattern: CGPatternRef) -> CGPatternRef;
}

extern "C-unwind" {
    pub fn CGPatternRelease(pattern: CGPatternRef);
}
