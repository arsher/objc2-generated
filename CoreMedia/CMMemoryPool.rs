//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmmemorypoolref?language=objc)
pub type CMMemoryPoolRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmmemorypoolerror_allocationfailed?language=objc)
pub const kCMMemoryPoolError_AllocationFailed: OSStatus = -15490;
/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmmemorypoolerror_invalidparameter?language=objc)
pub const kCMMemoryPoolError_InvalidParameter: OSStatus = -15491;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMMemoryPoolGetTypeID() -> CFTypeID;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmmemorypooloption_ageoutperiod?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMMemoryPoolOption_AgeOutPeriod: CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMMemoryPoolCreate(options: CFDictionaryRef) -> CMMemoryPoolRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMMemoryPoolGetAllocator(pool: CMMemoryPoolRef) -> CFAllocatorRef;
}

extern "C-unwind" {
    pub fn CMMemoryPoolFlush(pool: CMMemoryPoolRef);
}

extern "C-unwind" {
    pub fn CMMemoryPoolInvalidate(pool: CMMemoryPoolRef);
}