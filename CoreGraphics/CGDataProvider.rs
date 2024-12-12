//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataproviderref?language=objc)
pub type CGDataProviderRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataprovidergetbytescallback?language=objc)
pub type CGDataProviderGetBytesCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void, NonNull<c_void>, usize) -> usize>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataproviderskipforwardcallback?language=objc)
#[cfg(feature = "libc")]
pub type CGDataProviderSkipForwardCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void, libc::off_t) -> libc::off_t>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataproviderrewindcallback?language=objc)
pub type CGDataProviderRewindCallback = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataproviderreleaseinfocallback?language=objc)
pub type CGDataProviderReleaseInfoCallback = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataprovidersequentialcallbacks?language=objc)
#[cfg(feature = "libc")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGDataProviderSequentialCallbacks {
    pub version: c_uint,
    pub getBytes: CGDataProviderGetBytesCallback,
    pub skipForward: CGDataProviderSkipForwardCallback,
    pub rewind: CGDataProviderRewindCallback,
    pub releaseInfo: CGDataProviderReleaseInfoCallback,
}

#[cfg(feature = "libc")]
unsafe impl Encode for CGDataProviderSequentialCallbacks {
    const ENCODING: Encoding = Encoding::Struct(
        "CGDataProviderSequentialCallbacks",
        &[
            <c_uint>::ENCODING,
            <CGDataProviderGetBytesCallback>::ENCODING,
            <CGDataProviderSkipForwardCallback>::ENCODING,
            <CGDataProviderRewindCallback>::ENCODING,
            <CGDataProviderReleaseInfoCallback>::ENCODING,
        ],
    );
}

#[cfg(feature = "libc")]
unsafe impl RefEncode for CGDataProviderSequentialCallbacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataprovidergetbytepointercallback?language=objc)
pub type CGDataProviderGetBytePointerCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void) -> *mut c_void>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataproviderreleasebytepointercallback?language=objc)
pub type CGDataProviderReleaseBytePointerCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void, NonNull<c_void>)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataprovidergetbytesatpositioncallback?language=objc)
#[cfg(feature = "libc")]
pub type CGDataProviderGetBytesAtPositionCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void, NonNull<c_void>, libc::off_t, usize) -> usize>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataproviderdirectcallbacks?language=objc)
#[cfg(feature = "libc")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGDataProviderDirectCallbacks {
    pub version: c_uint,
    pub getBytePointer: CGDataProviderGetBytePointerCallback,
    pub releaseBytePointer: CGDataProviderReleaseBytePointerCallback,
    pub getBytesAtPosition: CGDataProviderGetBytesAtPositionCallback,
    pub releaseInfo: CGDataProviderReleaseInfoCallback,
}

#[cfg(feature = "libc")]
unsafe impl Encode for CGDataProviderDirectCallbacks {
    const ENCODING: Encoding = Encoding::Struct(
        "CGDataProviderDirectCallbacks",
        &[
            <c_uint>::ENCODING,
            <CGDataProviderGetBytePointerCallback>::ENCODING,
            <CGDataProviderReleaseBytePointerCallback>::ENCODING,
            <CGDataProviderGetBytesAtPositionCallback>::ENCODING,
            <CGDataProviderReleaseInfoCallback>::ENCODING,
        ],
    );
}

#[cfg(feature = "libc")]
unsafe impl RefEncode for CGDataProviderDirectCallbacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGDataProviderGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(feature = "libc")]
    pub fn CGDataProviderCreateSequential(
        info: *mut c_void,
        callbacks: *mut CGDataProviderSequentialCallbacks,
    ) -> CGDataProviderRef;
}

extern "C-unwind" {
    #[cfg(feature = "libc")]
    pub fn CGDataProviderCreateDirect(
        info: *mut c_void,
        size: libc::off_t,
        callbacks: *mut CGDataProviderDirectCallbacks,
    ) -> CGDataProviderRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataproviderreleasedatacallback?language=objc)
pub type CGDataProviderReleaseDataCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void, NonNull<c_void>, usize)>;

extern "C-unwind" {
    pub fn CGDataProviderCreateWithData(
        info: *mut c_void,
        data: *mut c_void,
        size: usize,
        release_data: CGDataProviderReleaseDataCallback,
    ) -> CGDataProviderRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGDataProviderCreateWithCFData(data: CFDataRef) -> CGDataProviderRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGDataProviderCreateWithURL(url: CFURLRef) -> CGDataProviderRef;
}

extern "C-unwind" {
    pub fn CGDataProviderCreateWithFilename(filename: *mut c_char) -> CGDataProviderRef;
}

extern "C-unwind" {
    pub fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
}

extern "C-unwind" {
    pub fn CGDataProviderRelease(provider: CGDataProviderRef);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGDataProviderCopyData(provider: CGDataProviderRef) -> CFDataRef;
}

extern "C-unwind" {
    pub fn CGDataProviderGetInfo(provider: CGDataProviderRef) -> *mut c_void;
}