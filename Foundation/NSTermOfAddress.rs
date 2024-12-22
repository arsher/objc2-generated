//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nstermofaddress?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTermOfAddress;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSTermOfAddress {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSTermOfAddress {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSTermOfAddress {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSTermOfAddress {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSTermOfAddress {}

extern_methods!(
    unsafe impl NSTermOfAddress {
        /// Term of address that uses gender-neutral pronouns (e.g. they/them/theirs in
        /// English), and an epicene grammatical gender when inflecting verbs and
        /// adjectives referring to the person
        #[method_id(@__retain_semantics Other neutral)]
        pub unsafe fn neutral() -> Retained<Self>;

        /// Term of address that uses feminine pronouns (e.g. she/her/hers in English),
        /// and a feminine grammatical gender when inflecting verbs and adjectives
        /// referring to the person
        #[method_id(@__retain_semantics Other feminine)]
        pub unsafe fn feminine() -> Retained<Self>;

        /// Term of address that uses masculine pronouns (e.g. he/him/his in English),
        /// and a masculine grammatical gender when inflecting verbs and adjectives
        /// referring to the person
        #[method_id(@__retain_semantics Other masculine)]
        pub unsafe fn masculine() -> Retained<Self>;

        /// The term of address that should be used for addressing the user
        ///
        /// This term of address will only compare equal to another `+[NSTermOfAddress currentUser]`
        #[method_id(@__retain_semantics Other currentUser)]
        pub unsafe fn currentUser() -> Retained<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSMorphology", feature = "NSString"))]
        /// A term of address restricted to a given language
        ///
        /// Parameter `language`: ISO language code identifier for the language
        ///
        /// Parameter `pronouns`: A list of pronouns in the target language that can be used to
        /// refer to the person.
        #[method_id(@__retain_semantics Other localizedForLanguageIdentifier:withPronouns:)]
        pub unsafe fn localizedForLanguageIdentifier_withPronouns(
            language: &NSString,
            pronouns: &NSArray<NSMorphologyPronoun>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        /// The ISO language code if this is a localized term of address
        #[method_id(@__retain_semantics Other languageIdentifier)]
        pub unsafe fn languageIdentifier(&self) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSMorphology"))]
        /// A list of pronouns for a localized term of address
        #[method_id(@__retain_semantics Other pronouns)]
        pub unsafe fn pronouns(&self) -> Option<Retained<NSArray<NSMorphologyPronoun>>>;
    }
);
