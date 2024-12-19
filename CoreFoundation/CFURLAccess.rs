//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

extern "C-unwind" {
    #[cfg(all(
        feature = "CFArray",
        feature = "CFBase",
        feature = "CFData",
        feature = "CFDictionary",
        feature = "CFURL"
    ))]
    #[deprecated = "For resource data, use the CFReadStream API. For file resource properties, use CFURLCopyResourcePropertiesForKeys."]
    pub fn CFURLCreateDataAndPropertiesFromResource(
        alloc: CFAllocatorRef,
        url: CFURLRef,
        resource_data: *mut CFDataRef,
        properties: *mut CFDictionaryRef,
        desired_properties: CFArrayRef,
        error_code: *mut i32,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFData", feature = "CFDictionary", feature = "CFURL"))]
    #[deprecated = "For resource data, use the CFWriteStream API. For file resource properties, use CFURLSetResourcePropertiesForKeys."]
    pub fn CFURLWriteDataAndPropertiesToResource(
        url: CFURLRef,
        data_to_write: CFDataRef,
        properties_to_write: CFDictionaryRef,
        error_code: *mut i32,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFURL")]
    #[deprecated = "Use CFURLGetFileSystemRepresentation and removefile(3) instead."]
    pub fn CFURLDestroyResource(url: CFURLRef, error_code: *mut i32) -> Boolean;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFURL"))]
    #[deprecated = "For file resource properties, use CFURLCopyResourcePropertyForKey."]
    pub fn CFURLCreatePropertyFromResource(
        alloc: CFAllocatorRef,
        url: CFURLRef,
        property: CFStringRef,
        error_code: *mut i32,
    ) -> CFTypeRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfurlerror?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[deprecated = "Use CFError codes instead"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFURLError(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFURLError {
    #[deprecated = "Use CFError codes instead"]
    pub const kCFURLUnknownError: Self = Self(-10);
    #[deprecated = "Use CFError codes instead"]
    pub const kCFURLUnknownSchemeError: Self = Self(-11);
    #[deprecated = "Use CFError codes instead"]
    pub const kCFURLResourceNotFoundError: Self = Self(-12);
    #[deprecated = "Use CFError codes instead"]
    pub const kCFURLResourceAccessViolationError: Self = Self(-13);
    #[deprecated = "Use CFError codes instead"]
    pub const kCFURLRemoteHostUnavailableError: Self = Self(-14);
    #[deprecated = "Use CFError codes instead"]
    pub const kCFURLImproperArgumentsError: Self = Self(-15);
    #[deprecated = "Use CFError codes instead"]
    pub const kCFURLUnknownPropertyKeyError: Self = Self(-16);
    #[deprecated = "Use CFError codes instead"]
    pub const kCFURLPropertyKeyUnavailableError: Self = Self(-17);
    #[deprecated = "Use CFError codes instead"]
    pub const kCFURLTimeoutError: Self = Self(-18);
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFURLError {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFURLError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfurlfileexists?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFURLFileExists: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfurlfiledirectorycontents?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFURLFileDirectoryContents: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfurlfilelength?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFURLFileLength: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfurlfilelastmodificationtime?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFURLFileLastModificationTime: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfurlfileposixmode?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFURLFilePOSIXMode: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfurlfileownerid?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFURLFileOwnerID: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfurlhttpstatuscode?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFURLHTTPStatusCode: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfurlhttpstatusline?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFURLHTTPStatusLine: CFStringRef;
}
