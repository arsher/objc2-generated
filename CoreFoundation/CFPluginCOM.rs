//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/hresult?language=objc)
pub type HRESULT = i32;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/ulong?language=objc)
pub type ULONG = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/lpvoid?language=objc)
pub type LPVOID = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/refiid?language=objc)
#[cfg(feature = "CFUUID")]
pub type REFIID = CFUUIDBytes;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/iunknownvtbl?language=objc)
#[cfg(feature = "CFUUID")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IUnknownVTbl {
    pub(crate) _reserved: *mut c_void,
    pub QueryInterface:
        Option<unsafe extern "C-unwind" fn(*mut c_void, REFIID, *mut LPVOID) -> HRESULT>,
    pub AddRef: Option<unsafe extern "C-unwind" fn(*mut c_void) -> ULONG>,
    pub Release: Option<unsafe extern "C-unwind" fn(*mut c_void) -> ULONG>,
}

#[cfg(feature = "CFUUID")]
unsafe impl Encode for IUnknownVTbl {
    const ENCODING: Encoding = Encoding::Struct("IUnknownVTbl", &[<*mut c_void>::ENCODING,<Option<unsafe extern "C-unwind" fn(*mut c_void,REFIID,*mut LPVOID,) -> HRESULT>>::ENCODING,<Option<unsafe extern "C-unwind" fn(*mut c_void,) -> ULONG>>::ENCODING,<Option<unsafe extern "C-unwind" fn(*mut c_void,) -> ULONG>>::ENCODING,]);
}

#[cfg(feature = "CFUUID")]
unsafe impl RefEncode for IUnknownVTbl {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
