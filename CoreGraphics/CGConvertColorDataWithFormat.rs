//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgcolordataformat?language=objc)
#[cfg(all(
    feature = "CGColorSpace",
    feature = "CGImage",
    feature = "objc2-core-foundation"
))]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGColorDataFormat {
    pub version: u32,
    pub colorspace_info: CFTypeRef,
    pub bitmap_info: CGBitmapInfo,
    pub bits_per_component: usize,
    pub bytes_per_row: usize,
    pub intent: CGColorRenderingIntent,
    pub decode: *mut CGFloat,
}

#[cfg(all(
    feature = "CGColorSpace",
    feature = "CGImage",
    feature = "objc2",
    feature = "objc2-core-foundation"
))]
unsafe impl Encode for CGColorDataFormat {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u32>::ENCODING,
            <CFTypeRef>::ENCODING,
            <CGBitmapInfo>::ENCODING,
            <usize>::ENCODING,
            <usize>::ENCODING,
            <CGColorRenderingIntent>::ENCODING,
            <*mut CGFloat>::ENCODING,
        ],
    );
}

#[cfg(all(
    feature = "CGColorSpace",
    feature = "CGImage",
    feature = "objc2",
    feature = "objc2-core-foundation"
))]
unsafe impl RefEncode for CGColorDataFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CGColorSpace",
        feature = "CGImage",
        feature = "objc2-core-foundation"
    ))]
    pub fn CGConvertColorDataWithFormat(
        width: usize,
        height: usize,
        dst_data: *mut c_void,
        dst_format: CGColorDataFormat,
        src_data: *mut c_void,
        src_format: CGColorDataFormat,
        options: CFDictionaryRef,
    ) -> bool;
}
