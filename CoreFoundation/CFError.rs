//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cferrordomain?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "CFBase")]
pub type CFErrorDomain = CFStringRef;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cferrorref?language=objc)
pub type CFErrorRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFErrorGetTypeID() -> CFTypeID;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrordomainposix?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorDomainPOSIX: CFErrorDomain;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrordomainosstatus?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorDomainOSStatus: CFErrorDomain;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrordomainmach?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorDomainMach: CFErrorDomain;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrordomaincocoa?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorDomainCocoa: CFErrorDomain;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrorlocalizeddescriptionkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorLocalizedDescriptionKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrorlocalizedfailurekey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorLocalizedFailureKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrorlocalizedfailurereasonkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorLocalizedFailureReasonKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrorlocalizedrecoverysuggestionkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorLocalizedRecoverySuggestionKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrordescriptionkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorDescriptionKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrorunderlyingerrorkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorUnderlyingErrorKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrorurlkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorURLKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcferrorfilepathkey?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFErrorFilePathKey: CFStringRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFDictionary"))]
    pub fn CFErrorCreate(
        allocator: CFAllocatorRef,
        domain: CFErrorDomain,
        code: CFIndex,
        user_info: CFDictionaryRef,
    ) -> CFErrorRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFErrorCreateWithUserInfoKeysAndValues(
        allocator: CFAllocatorRef,
        domain: CFErrorDomain,
        code: CFIndex,
        user_info_keys: *const *const c_void,
        user_info_values: *const *const c_void,
        num_user_info_values: CFIndex,
    ) -> CFErrorRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFErrorGetDomain(err: CFErrorRef) -> CFErrorDomain;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFErrorGetCode(err: CFErrorRef) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "CFDictionary")]
    pub fn CFErrorCopyUserInfo(err: CFErrorRef) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFErrorCopyDescription(err: CFErrorRef) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFErrorCopyFailureReason(err: CFErrorRef) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFErrorCopyRecoverySuggestion(err: CFErrorRef) -> CFStringRef;
}
