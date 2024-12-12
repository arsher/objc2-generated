//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfarrayref?language=objc)
pub type CGPDFArrayRef = *mut c_void;

extern "C-unwind" {
    pub fn CGPDFArrayGetCount(array: CGPDFArrayRef) -> usize;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFObject")]
    pub fn CGPDFArrayGetObject(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFObjectRef,
    ) -> bool;
}

extern "C-unwind" {
    pub fn CGPDFArrayGetNull(array: CGPDFArrayRef, index: usize) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFObject")]
    pub fn CGPDFArrayGetBoolean(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFBoolean,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFObject")]
    pub fn CGPDFArrayGetInteger(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFInteger,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGPDFObject", feature = "objc2-core-foundation"))]
    pub fn CGPDFArrayGetNumber(array: CGPDFArrayRef, index: usize, value: *mut CGPDFReal) -> bool;
}

extern "C-unwind" {
    pub fn CGPDFArrayGetName(array: CGPDFArrayRef, index: usize, value: *mut *mut c_char) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFString")]
    pub fn CGPDFArrayGetString(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFStringRef,
    ) -> bool;
}

extern "C-unwind" {
    pub fn CGPDFArrayGetArray(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFArrayRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFDictionary")]
    pub fn CGPDFArrayGetDictionary(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFDictionaryRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFStream")]
    pub fn CGPDFArrayGetStream(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFStreamRef,
    ) -> bool;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfarrayapplierblock?language=objc)
#[cfg(all(feature = "CGPDFObject", feature = "block2"))]
pub type CGPDFArrayApplierBlock =
    *mut block2::Block<dyn Fn(usize, CGPDFObjectRef, *mut c_void) -> bool>;

extern "C-unwind" {
    #[cfg(all(feature = "CGPDFObject", feature = "block2"))]
    pub fn CGPDFArrayApplyBlock(
        array: CGPDFArrayRef,
        block: CGPDFArrayApplierBlock,
        info: *mut c_void,
    );
}