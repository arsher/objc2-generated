//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctfontcollectionref?language=objc)
pub type CTFontCollectionRef = *const c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctmutablefontcollectionref?language=objc)
pub type CTMutableFontCollectionRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionGetTypeID() -> CFTypeID;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctfontcollectionsortdescriptorscallback?language=objc)
#[cfg(all(feature = "CTFontDescriptor", feature = "objc2-core-foundation"))]
pub type CTFontCollectionSortDescriptorsCallback = Option<
    unsafe extern "C-unwind" fn(
        CTFontDescriptorRef,
        CTFontDescriptorRef,
        NonNull<c_void>,
    ) -> CFComparisonResult,
>;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontcollectionremoveduplicatesoption?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontCollectionRemoveDuplicatesOption: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontcollectionincludedisabledfontsoption?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontCollectionIncludeDisabledFontsOption: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontcollectiondisallowautoactivationoption?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontCollectionDisallowAutoActivationOption: CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateFromAvailableFonts(
        options: CFDictionaryRef,
    ) -> CTFontCollectionRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateWithFontDescriptors(
        query_descriptors: CFArrayRef,
        options: CFDictionaryRef,
    ) -> CTFontCollectionRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateCopyWithFontDescriptors(
        original: CTFontCollectionRef,
        query_descriptors: CFArrayRef,
        options: CFDictionaryRef,
    ) -> CTFontCollectionRef;
}

extern "C-unwind" {
    pub fn CTFontCollectionCreateMutableCopy(
        original: CTFontCollectionRef,
    ) -> CTMutableFontCollectionRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCopyQueryDescriptors(collection: CTFontCollectionRef) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionSetQueryDescriptors(
        collection: CTMutableFontCollectionRef,
        descriptors: CFArrayRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCopyExclusionDescriptors(collection: CTFontCollectionRef) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionSetExclusionDescriptors(
        collection: CTMutableFontCollectionRef,
        descriptors: CFArrayRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateMatchingFontDescriptors(
        collection: CTFontCollectionRef,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CTFontDescriptor", feature = "objc2-core-foundation"))]
    pub fn CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback(
        collection: CTFontCollectionRef,
        sort_callback: CTFontCollectionSortDescriptorsCallback,
        ref_con: *mut c_void,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateMatchingFontDescriptorsWithOptions(
        collection: CTFontCollectionRef,
        options: CFDictionaryRef,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateMatchingFontDescriptorsForFamily(
        collection: CTFontCollectionRef,
        family_name: CFStringRef,
        options: CFDictionaryRef,
    ) -> CFArrayRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctfontcollectioncopyoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTFontCollectionCopyOptions(pub u32);
bitflags::bitflags! {
    impl CTFontCollectionCopyOptions: u32 {
        const kCTFontCollectionCopyDefaultOptions = 0;
        const kCTFontCollectionCopyUnique = 1<<0;
        const kCTFontCollectionCopyStandardSort = 1<<1;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CTFontCollectionCopyOptions {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CTFontCollectionCopyOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCopyFontAttribute(
        collection: CTFontCollectionRef,
        attribute_name: CFStringRef,
        options: CTFontCollectionCopyOptions,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCopyFontAttributes(
        collection: CTFontCollectionRef,
        attribute_names: CFSetRef,
        options: CTFontCollectionCopyOptions,
    ) -> CFArrayRef;
}
