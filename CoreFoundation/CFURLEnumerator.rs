//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfurlenumeratorref?language=objc)
pub type CFURLEnumeratorRef = *const c_void;

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFURLEnumeratorGetTypeID() -> CFTypeID;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfurlenumeratoroptions?language=objc)
// NS_OPTIONS
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFURLEnumeratorOptions(pub CFOptionFlags);
#[cfg(feature = "CFBase")]
bitflags::bitflags! {
    impl CFURLEnumeratorOptions: CFOptionFlags {
        const kCFURLEnumeratorDefaultBehavior = 0;
        const kCFURLEnumeratorDescendRecursively = 1<<0;
        const kCFURLEnumeratorSkipInvisibles = 1<<1;
        const kCFURLEnumeratorGenerateFileReferenceURLs = 1<<2;
        const kCFURLEnumeratorSkipPackageContents = 1<<3;
        const kCFURLEnumeratorIncludeDirectoriesPreOrder = 1<<4;
        const kCFURLEnumeratorIncludeDirectoriesPostOrder = 1<<5;
        const kCFURLEnumeratorGenerateRelativePathURLs = 1<<6;
    }
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFURLEnumeratorOptions {
    const ENCODING: Encoding = CFOptionFlags::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFURLEnumeratorOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(all(feature = "CFArray", feature = "CFBase", feature = "CFURL"))]
    pub fn CFURLEnumeratorCreateForDirectoryURL(
        alloc: CFAllocatorRef,
        directory_url: CFURLRef,
        option: CFURLEnumeratorOptions,
        property_keys: CFArrayRef,
    ) -> CFURLEnumeratorRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFArray", feature = "CFBase"))]
    pub fn CFURLEnumeratorCreateForMountedVolumes(
        alloc: CFAllocatorRef,
        option: CFURLEnumeratorOptions,
        property_keys: CFArrayRef,
    ) -> CFURLEnumeratorRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfurlenumeratorresult?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFURLEnumeratorResult(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFURLEnumeratorResult {
    pub const kCFURLEnumeratorSuccess: Self = Self(1);
    pub const kCFURLEnumeratorEnd: Self = Self(2);
    pub const kCFURLEnumeratorError: Self = Self(3);
    pub const kCFURLEnumeratorDirectoryPostOrderSuccess: Self = Self(4);
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFURLEnumeratorResult {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFURLEnumeratorResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFError", feature = "CFURL"))]
    pub fn CFURLEnumeratorGetNextURL(
        enumerator: CFURLEnumeratorRef,
        url: *mut CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFURLEnumeratorResult;
}

extern "C-unwind" {
    pub fn CFURLEnumeratorSkipDescendents(enumerator: CFURLEnumeratorRef);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFURLEnumeratorGetDescendentLevel(enumerator: CFURLEnumeratorRef) -> CFIndex;
}

extern "C-unwind" {
    #[deprecated = "Use File System Events API instead"]
    pub fn CFURLEnumeratorGetSourceDidChange(enumerator: CFURLEnumeratorRef) -> Boolean;
}
