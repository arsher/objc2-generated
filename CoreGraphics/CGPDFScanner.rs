//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfscannerref?language=objc)
pub type CGPDFScannerRef = *mut c_void;

extern "C-unwind" {
    #[cfg(all(feature = "CGPDFContentStream", feature = "CGPDFOperatorTable"))]
    pub fn CGPDFScannerCreate(
        cs: CGPDFContentStreamRef,
        table: CGPDFOperatorTableRef,
        info: *mut c_void,
    ) -> CGPDFScannerRef;
}

extern "C-unwind" {
    pub fn CGPDFScannerRetain(scanner: CGPDFScannerRef) -> CGPDFScannerRef;
}

extern "C-unwind" {
    pub fn CGPDFScannerRelease(scanner: CGPDFScannerRef);
}

extern "C-unwind" {
    pub fn CGPDFScannerScan(scanner: CGPDFScannerRef) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFContentStream")]
    pub fn CGPDFScannerGetContentStream(scanner: CGPDFScannerRef) -> CGPDFContentStreamRef;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFObject")]
    pub fn CGPDFScannerPopObject(scanner: CGPDFScannerRef, value: *mut CGPDFObjectRef) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFObject")]
    pub fn CGPDFScannerPopBoolean(scanner: CGPDFScannerRef, value: *mut CGPDFBoolean) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFObject")]
    pub fn CGPDFScannerPopInteger(scanner: CGPDFScannerRef, value: *mut CGPDFInteger) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGPDFObject", feature = "objc2-core-foundation"))]
    pub fn CGPDFScannerPopNumber(scanner: CGPDFScannerRef, value: *mut CGPDFReal) -> bool;
}

extern "C-unwind" {
    pub fn CGPDFScannerPopName(scanner: CGPDFScannerRef, value: *mut *mut c_char) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFString")]
    pub fn CGPDFScannerPopString(scanner: CGPDFScannerRef, value: *mut CGPDFStringRef) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFArray")]
    pub fn CGPDFScannerPopArray(scanner: CGPDFScannerRef, value: *mut CGPDFArrayRef) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFDictionary")]
    pub fn CGPDFScannerPopDictionary(
        scanner: CGPDFScannerRef,
        value: *mut CGPDFDictionaryRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFStream")]
    pub fn CGPDFScannerPopStream(scanner: CGPDFScannerRef, value: *mut CGPDFStreamRef) -> bool;
}

extern "C-unwind" {
    pub fn CGPDFScannerStop(s: CGPDFScannerRef);
}
