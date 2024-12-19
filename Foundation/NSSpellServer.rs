//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsspellserver?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSpellServer;
);

unsafe impl NSObjectProtocol for NSSpellServer {}

extern_methods!(
    unsafe impl NSSpellServer {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSSpellServerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSpellServerDelegate>>,
        );

        #[cfg(feature = "NSString")]
        #[method(registerLanguage:byVendor:)]
        pub unsafe fn registerLanguage_byVendor(
            &self,
            language: Option<&NSString>,
            vendor: Option<&NSString>,
        ) -> bool;

        #[cfg(feature = "NSString")]
        #[method(isWordInUserDictionaries:caseSensitive:)]
        pub unsafe fn isWordInUserDictionaries_caseSensitive(
            &self,
            word: &NSString,
            flag: bool,
        ) -> bool;

        #[method(run)]
        pub unsafe fn run(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSpellServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsgrammarrange?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSGrammarRange: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsgrammaruserdescription?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSGrammarUserDescription: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsgrammarcorrections?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSGrammarCorrections: &'static NSString;
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsspellserverdelegate?language=objc)
    pub unsafe trait NSSpellServerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[optional]
        #[method(spellServer:findMisspelledWordInString:language:wordCount:countOnly:)]
        unsafe fn spellServer_findMisspelledWordInString_language_wordCount_countOnly(
            &self,
            sender: &NSSpellServer,
            string_to_check: &NSString,
            language: &NSString,
            word_count: NonNull<NSInteger>,
            count_only: bool,
        ) -> NSRange;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other spellServer:suggestGuessesForWord:inLanguage:)]
        unsafe fn spellServer_suggestGuessesForWord_inLanguage(
            &self,
            sender: &NSSpellServer,
            word: &NSString,
            language: &NSString,
        ) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(feature = "NSString")]
        #[optional]
        #[method(spellServer:didLearnWord:inLanguage:)]
        unsafe fn spellServer_didLearnWord_inLanguage(
            &self,
            sender: &NSSpellServer,
            word: &NSString,
            language: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[optional]
        #[method(spellServer:didForgetWord:inLanguage:)]
        unsafe fn spellServer_didForgetWord_inLanguage(
            &self,
            sender: &NSSpellServer,
            word: &NSString,
            language: &NSString,
        );

        #[cfg(all(feature = "NSArray", feature = "NSRange", feature = "NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other spellServer:suggestCompletionsForPartialWordRange:inString:language:)]
        unsafe fn spellServer_suggestCompletionsForPartialWordRange_inString_language(
            &self,
            sender: &NSSpellServer,
            range: NSRange,
            string: &NSString,
            language: &NSString,
        ) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(all(
            feature = "NSArray",
            feature = "NSDictionary",
            feature = "NSRange",
            feature = "NSString"
        ))]
        #[optional]
        #[method(spellServer:checkGrammarInString:language:details:)]
        unsafe fn spellServer_checkGrammarInString_language_details(
            &self,
            sender: &NSSpellServer,
            string_to_check: &NSString,
            language: Option<&NSString>,
            details: Option<&mut Option<Retained<NSArray<NSDictionary<NSString, AnyObject>>>>>,
        ) -> NSRange;

        #[cfg(all(
            feature = "NSArray",
            feature = "NSDictionary",
            feature = "NSOrthography",
            feature = "NSString",
            feature = "NSTextCheckingResult"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other spellServer:checkString:offset:types:options:orthography:wordCount:)]
        unsafe fn spellServer_checkString_offset_types_options_orthography_wordCount(
            &self,
            sender: &NSSpellServer,
            string_to_check: &NSString,
            offset: NSUInteger,
            checking_types: NSTextCheckingTypes,
            options: Option<&NSDictionary<NSString, AnyObject>>,
            orthography: Option<&NSOrthography>,
            word_count: NonNull<NSInteger>,
        ) -> Option<Retained<NSArray<NSTextCheckingResult>>>;

        #[cfg(feature = "NSString")]
        #[optional]
        #[method(spellServer:recordResponse:toCorrection:forWord:language:)]
        unsafe fn spellServer_recordResponse_toCorrection_forWord_language(
            &self,
            sender: &NSSpellServer,
            response: NSUInteger,
            correction: &NSString,
            word: &NSString,
            language: &NSString,
        );
    }

    unsafe impl ProtocolType for dyn NSSpellServerDelegate {}
);
