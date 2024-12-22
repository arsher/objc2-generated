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
    /// Returns the type identifier for Core Text font collection references.
    ///
    /// Returns: The identifier for the opaque types CTFontCollectionRef or CTMutableFontCollectionRef.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionGetTypeID() -> CFTypeID;
}

/// Collection sorting callback.
///
/// This callback can be specified to obtain the matching font descriptors of a collection in sorted order. Return the appropriate comparison result of first descriptor to second descriptor.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/ctfontcollectionsortdescriptorscallback?language=objc)
#[cfg(all(feature = "CTFontDescriptor", feature = "objc2-core-foundation"))]
pub type CTFontCollectionSortDescriptorsCallback = Option<
    unsafe extern "C-unwind" fn(
        CTFontDescriptorRef,
        CTFontDescriptorRef,
        NonNull<c_void>,
    ) -> CFComparisonResult,
>;

extern "C" {
    /// kCTFontCollectionRemoveDuplicatesOption
    ///
    /// Option key to specify filtering of duplicates.
    ///
    /// Specify this option key in the options dictionary with a non- zero value to enable automatic filtering of duplicate font descriptors.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontcollectionremoveduplicatesoption?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontCollectionRemoveDuplicatesOption: CFStringRef;
}

extern "C" {
    /// kCTFontCollectionIncludeDisabledFontsOption
    ///
    /// Option key to include disabled fonts in the matching results.
    ///
    /// Specify this option key in the options dictionary with a non-zero value to enable matching of disabled fonts. You can pass font descriptors specifying disabled fonts to CTFontManagerEnableFontDescriptors, but you cannot use such a font descriptor to query font attributes from the system database or create a CTFontRef.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontcollectionincludedisabledfontsoption?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontCollectionIncludeDisabledFontsOption: CFStringRef;
}

extern "C" {
    /// kCTFontCollectionDisallowAutoActivationOption
    ///
    /// Option key to avoid auto-activating fonts.
    ///
    /// Specify this option key in the options dictionary with a non-zero value to disallow searches for missing fonts (font descriptors returning no results).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontcollectiondisallowautoactivationoption?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontCollectionDisallowAutoActivationOption: CFStringRef;
}

extern "C-unwind" {
    /// Returns a new font collection matching all available fonts.
    ///
    ///
    /// Parameter `options`: The options dictionary. See constant option keys.
    ///
    ///
    /// Returns: This function creates a new collection containing all fonts available to the current application.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateFromAvailableFonts(
        options: CFDictionaryRef,
    ) -> CTFontCollectionRef;
}

extern "C-unwind" {
    /// Returns a new collection based on the array of font descriptors.
    ///
    ///
    /// Parameter `queryDescriptors`: An array of font descriptors to use for matching. May be NULL, in which case the matching descriptors will be NULL.
    ///
    ///
    /// Parameter `options`: The options dictionary. See constant option keys.
    ///
    ///
    /// Returns: This function creates a new collection based on the provided font descriptors. The contents of this collection is defined by matching the provided descriptors against all available font descriptors.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateWithFontDescriptors(
        query_descriptors: CFArrayRef,
        options: CFDictionaryRef,
    ) -> CTFontCollectionRef;
}

extern "C-unwind" {
    /// Returns a copy of the original collection augmented with the new font descriptors.
    ///
    ///
    /// Parameter `original`: The original font collection reference.
    ///
    ///
    /// Parameter `queryDescriptors`: An array of font descriptors to augment those of the original collection.
    ///
    ///
    /// Parameter `options`: The options dictionary. See constant option keys.
    ///
    ///
    /// Returns: This function creates a copy of the original font collection augmented by the new font descriptors and options. The new font descriptors are merged with the existing descriptors to create a single set.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateCopyWithFontDescriptors(
        original: CTFontCollectionRef,
        query_descriptors: CFArrayRef,
        options: CFDictionaryRef,
    ) -> CTFontCollectionRef;
}

extern "C-unwind" {
    /// Returns a mutable copy of the original collection.
    ///
    ///
    /// Parameter `original`: The original font collection reference.
    ///
    ///
    /// Returns: This function creates a mutable copy of the original font collection.
    pub fn CTFontCollectionCreateMutableCopy(
        original: CTFontCollectionRef,
    ) -> CTMutableFontCollectionRef;
}

extern "C-unwind" {
    /// Returns the array of descriptors to match.
    ///
    ///
    /// Parameter `collection`: The font collection reference.
    ///
    ///
    /// Returns: This function returns a retained reference to the array of descriptors to be used to query (match) the system font database. The return value is undefined if CTFontCollectionCreateFromAvailableFonts was used to create the collection.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCopyQueryDescriptors(collection: CTFontCollectionRef) -> CFArrayRef;
}

extern "C-unwind" {
    /// Replaces the array of descriptors to match.
    ///
    ///
    /// Parameter `collection`: The font collection reference.
    ///
    ///
    /// Parameter `descriptors`: An array of CTFontDescriptorRef. May be NULL to represent an empty collection, in which case the matching descriptors will also be NULL.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionSetQueryDescriptors(
        collection: CTMutableFontCollectionRef,
        descriptors: CFArrayRef,
    );
}

extern "C-unwind" {
    /// Returns the array of descriptors to exclude from the match.
    ///
    ///
    /// Parameter `collection`: The font collection reference.
    ///
    ///
    /// Returns: This function returns a retained reference to the array of descriptors to be used to query (match) the system font database.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCopyExclusionDescriptors(collection: CTFontCollectionRef) -> CFArrayRef;
}

extern "C-unwind" {
    /// Replaces the array of descriptors to exclude from the match.
    ///
    ///
    /// Parameter `collection`: The font collection reference.
    ///
    ///
    /// Parameter `descriptors`: An array of CTFontDescriptorRef. May be NULL.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionSetExclusionDescriptors(
        collection: CTMutableFontCollectionRef,
        descriptors: CFArrayRef,
    );
}

extern "C-unwind" {
    /// Returns an array of font descriptors matching the collection.
    ///
    ///
    /// Parameter `collection`: The font collection reference.
    ///
    ///
    /// Returns: An array of CTFontDescriptors matching the collection definition or NULL if there are none.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateMatchingFontDescriptors(
        collection: CTFontCollectionRef,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    /// Returns the array of matching font descriptors sorted with the callback function.
    ///
    ///
    /// Parameter `collection`: The collection reference.
    ///
    ///
    /// Parameter `sortCallback`: The sorting callback function that defines the sort order.
    ///
    ///
    /// Parameter `refCon`: Pointer to client data define context for the callback.
    ///
    ///
    /// Returns: An array of CTFontDescriptors matching the criteria of the collection, sorted by the results of the sorting callback function, or NULL if there are none.
    #[cfg(all(feature = "CTFontDescriptor", feature = "objc2-core-foundation"))]
    pub fn CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback(
        collection: CTFontCollectionRef,
        sort_callback: CTFontCollectionSortDescriptorsCallback,
        ref_con: *mut c_void,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    /// Returns an array of font descriptors matching the collection.
    ///
    ///
    /// Parameter `collection`: The font collection reference.
    ///
    ///
    /// Parameter `options`: The options dictionary. See constant option keys. May be NULL, in which case this call returns the same results as CTFontCollectionCreateMatchingFontDescriptors, using the options passed in when the collection was created.
    ///
    ///
    /// Returns: An array of CTFontDescriptors matching the collection definition or NULL if there are none.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateMatchingFontDescriptorsWithOptions(
        collection: CTFontCollectionRef,
        options: CFDictionaryRef,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    /// Returns an array of font descriptors matching the specified family, one descriptor for each style in the collection.
    ///
    ///
    /// Parameter `collection`: The font collection reference.
    ///
    ///
    /// Parameter `familyName`: The font family name
    ///
    ///
    /// Returns: An array of CTFontDescriptors matching the specified family in the collection or NULL if there are none.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCreateMatchingFontDescriptorsForFamily(
        collection: CTFontCollectionRef,
        family_name: CFStringRef,
        options: CFDictionaryRef,
    ) -> CFArrayRef;
}

/// Option bits for use with CTFontCollectionCopyFontAttribute(s).
///
///
/// Passing this option indicates that the return values should be sorted in standard UI order, suitable for display to the user. This is the same sorting behavior used by NSFontPanel and Font Book.
///
///
/// Passing this option indicates that duplicate values should be removed from the results.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/ctfontcollectioncopyoptions?language=objc)
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
    /// Returns an array of font descriptor attribute values.
    ///
    ///
    /// Parameter `collection`: The font collection reference.
    ///
    ///
    /// Parameter `attributeName`: The attribute to retrieve for each descriptor in the collection.
    ///
    ///
    /// Parameter `options`: Options to alter the return value.
    ///
    ///
    /// Returns: An array containing one value for each descriptor. With kCTFontCollectionCopyDefaultOptions, the values will be in the same order as the results from CTFontCollectionCreateMatchingFontDescriptors and NULL values will be transformed to kCFNull. When the kCTFontCollectionCopyUnique is set, duplicate values will be removed. When kCTFontCollectionCopyStandardSort is set, the values will be sorted in standard UI order.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCopyFontAttribute(
        collection: CTFontCollectionRef,
        attribute_name: CFStringRef,
        options: CTFontCollectionCopyOptions,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    /// Returns an array of dictionaries containing font descriptor attribute values.
    ///
    ///
    /// Parameter `collection`: The font collection reference.
    ///
    ///
    /// Parameter `attributeNames`: The attributes to retrieve for each descriptor in the collection.
    ///
    ///
    /// Parameter `options`: Options to alter the return value.
    ///
    ///
    /// Returns: An array containing one CFDictionary value for each descriptor mapping the requested attribute names. With kCTFontCollectionCopyDefaultOptions, the values will be in the same order as the results from CTFontCollectionCreateMatchingFontDescriptors. When the kCTFontCollectionCopyUnique is set, duplicate values will be removed. When kCTFontCollectionCopyStandardSort is set, the values will be sorted in standard UI order.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTFontCollectionCopyFontAttributes(
        collection: CTFontCollectionRef,
        attribute_names: CFSetRef,
        options: CTFontCollectionCopyOptions,
    ) -> CFArrayRef;
}
