//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsattributedstringkey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSAttributedStringKey = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsattributedstringformattingcontextkey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSAttributedStringFormattingContextKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinflectionconceptskey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSInflectionConceptsKey: &'static NSAttributedStringFormattingContextKey;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsattributedstring?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAttributedString;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSAttributedString {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSAttributedString {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSAttributedString {
    type Result = Self;
}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSAttributedString {}

#[cfg(feature = "NSObject")]
unsafe impl MutableCopyingHelper for NSAttributedString {
    type Result = NSMutableAttributedString;
}

unsafe impl NSObjectProtocol for NSAttributedString {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSAttributedString {}

extern_methods!(
    unsafe impl NSAttributedString {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other string)]
        pub fn string(&self) -> Retained<NSString>;

        #[cfg(all(feature = "NSDictionary", feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other attributesAtIndex:effectiveRange:)]
        pub unsafe fn attributesAtIndex_effectiveRange(
            &self,
            location: NSUInteger,
            range: NSRangePointer,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAttributedString {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for NSAttributedString {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsattributedstringenumerationoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAttributedStringEnumerationOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSAttributedStringEnumerationOptions: NSUInteger {
        const NSAttributedStringEnumerationReverse = 1<<1;
        const NSAttributedStringEnumerationLongestEffectiveRangeNotRequired = 1<<20;
    }
}

unsafe impl Encode for NSAttributedStringEnumerationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSAttributedStringEnumerationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSExtendedAttributedString
    unsafe impl NSAttributedString {
        #[method(length)]
        pub fn length(&self) -> NSUInteger;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other attribute:atIndex:effectiveRange:)]
        pub unsafe fn attribute_atIndex_effectiveRange(
            &self,
            attr_name: &NSAttributedStringKey,
            location: NSUInteger,
            range: NSRangePointer,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other attributedSubstringFromRange:)]
        pub unsafe fn attributedSubstringFromRange(
            &self,
            range: NSRange,
        ) -> Retained<NSAttributedString>;

        #[cfg(all(feature = "NSDictionary", feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other attributesAtIndex:longestEffectiveRange:inRange:)]
        pub unsafe fn attributesAtIndex_longestEffectiveRange_inRange(
            &self,
            location: NSUInteger,
            range: NSRangePointer,
            range_limit: NSRange,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other attribute:atIndex:longestEffectiveRange:inRange:)]
        pub unsafe fn attribute_atIndex_longestEffectiveRange_inRange(
            &self,
            attr_name: &NSAttributedStringKey,
            location: NSUInteger,
            range: NSRangePointer,
            range_limit: NSRange,
        ) -> Option<Retained<AnyObject>>;

        #[method(isEqualToAttributedString:)]
        pub unsafe fn isEqualToAttributedString(&self, other: &NSAttributedString) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub fn initWithString(this: Allocated<Self>, str: &NSString) -> Retained<Self>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithString:attributes:)]
        pub unsafe fn initWithString_attributes(
            this: Allocated<Self>,
            str: &NSString,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub fn initWithAttributedString(
            this: Allocated<Self>,
            attr_str: &NSAttributedString,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "NSDictionary",
            feature = "NSRange",
            feature = "NSString",
            feature = "block2"
        ))]
        #[method(enumerateAttributesInRange:options:usingBlock:)]
        pub unsafe fn enumerateAttributesInRange_options_usingBlock(
            &self,
            enumeration_range: NSRange,
            opts: NSAttributedStringEnumerationOptions,
            block: &block2::Block<
                dyn Fn(
                        NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
                        NSRange,
                        NonNull<Bool>,
                    ) + '_,
            >,
        );

        #[cfg(all(feature = "NSRange", feature = "NSString", feature = "block2"))]
        #[method(enumerateAttribute:inRange:options:usingBlock:)]
        pub unsafe fn enumerateAttribute_inRange_options_usingBlock(
            &self,
            attr_name: &NSAttributedStringKey,
            enumeration_range: NSRange,
            opts: NSAttributedStringEnumerationOptions,
            block: &block2::Block<dyn Fn(*mut AnyObject, NSRange, NonNull<Bool>) + '_>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSAttributedString`
    ///
    /// NSExtendedAttributedString
    unsafe impl NSMutableAttributedString {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub fn initWithString(this: Allocated<Self>, str: &NSString) -> Retained<Self>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithString:attributes:)]
        pub unsafe fn initWithString_attributes(
            this: Allocated<Self>,
            str: &NSString,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub fn initWithAttributedString(
            this: Allocated<Self>,
            attr_str: &NSAttributedString,
        ) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmutableattributedstring?language=objc)
    #[unsafe(super(NSAttributedString, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMutableAttributedString;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSMutableAttributedString {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSMutableAttributedString {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSMutableAttributedString {
    type Result = NSAttributedString;
}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSMutableAttributedString {}

#[cfg(feature = "NSObject")]
unsafe impl MutableCopyingHelper for NSMutableAttributedString {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSMutableAttributedString {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSMutableAttributedString {}

extern_methods!(
    unsafe impl NSMutableAttributedString {
        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method(replaceCharactersInRange:withString:)]
        pub unsafe fn replaceCharactersInRange_withString(&self, range: NSRange, str: &NSString);

        #[cfg(all(feature = "NSDictionary", feature = "NSRange", feature = "NSString"))]
        #[method(setAttributes:range:)]
        pub unsafe fn setAttributes_range(
            &self,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            range: NSRange,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMutableAttributedString {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for NSMutableAttributedString {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedMutableAttributedString
    unsafe impl NSMutableAttributedString {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other mutableString)]
        pub unsafe fn mutableString(&self) -> Retained<NSMutableString>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method(addAttribute:value:range:)]
        pub unsafe fn addAttribute_value_range(
            &self,
            name: &NSAttributedStringKey,
            value: &AnyObject,
            range: NSRange,
        );

        #[cfg(all(feature = "NSDictionary", feature = "NSRange", feature = "NSString"))]
        #[method(addAttributes:range:)]
        pub unsafe fn addAttributes_range(
            &self,
            attrs: &NSDictionary<NSAttributedStringKey, AnyObject>,
            range: NSRange,
        );

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method(removeAttribute:range:)]
        pub unsafe fn removeAttribute_range(&self, name: &NSAttributedStringKey, range: NSRange);

        #[cfg(feature = "NSRange")]
        #[method(replaceCharactersInRange:withAttributedString:)]
        pub unsafe fn replaceCharactersInRange_withAttributedString(
            &self,
            range: NSRange,
            attr_string: &NSAttributedString,
        );

        #[method(insertAttributedString:atIndex:)]
        pub unsafe fn insertAttributedString_atIndex(
            &self,
            attr_string: &NSAttributedString,
            loc: NSUInteger,
        );

        #[method(appendAttributedString:)]
        pub unsafe fn appendAttributedString(&self, attr_string: &NSAttributedString);

        #[cfg(feature = "NSRange")]
        #[method(deleteCharactersInRange:)]
        pub unsafe fn deleteCharactersInRange(&self, range: NSRange);

        #[method(setAttributedString:)]
        pub fn setAttributedString(&self, attr_string: &NSAttributedString);

        #[method(beginEditing)]
        pub unsafe fn beginEditing(&self);

        #[method(endEditing)]
        pub unsafe fn endEditing(&self);
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinlinepresentationintent?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSInlinePresentationIntent(pub NSUInteger);
bitflags::bitflags! {
    impl NSInlinePresentationIntent: NSUInteger {
        #[doc(alias = "NSInlinePresentationIntentEmphasized")]
        const Emphasized = 1<<0;
        #[doc(alias = "NSInlinePresentationIntentStronglyEmphasized")]
        const StronglyEmphasized = 1<<1;
        #[doc(alias = "NSInlinePresentationIntentCode")]
        const Code = 1<<2;
        #[doc(alias = "NSInlinePresentationIntentStrikethrough")]
        const Strikethrough = 1<<5;
        #[doc(alias = "NSInlinePresentationIntentSoftBreak")]
        const SoftBreak = 1<<6;
        #[doc(alias = "NSInlinePresentationIntentLineBreak")]
        const LineBreak = 1<<7;
        #[doc(alias = "NSInlinePresentationIntentInlineHTML")]
        const InlineHTML = 1<<8;
        #[doc(alias = "NSInlinePresentationIntentBlockHTML")]
        const BlockHTML = 1<<9;
    }
}

unsafe impl Encode for NSInlinePresentationIntent {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSInlinePresentationIntent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinlinepresentationintentattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSInlinePresentationIntentAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsalternatedescriptionattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSAlternateDescriptionAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsimageurlattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSImageURLAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslanguageidentifierattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLanguageIdentifierAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmarkdownsourcepositionattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSMarkdownSourcePositionAttributeName: &'static NSAttributedStringKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsattributedstringmarkdownparsingfailurepolicy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAttributedStringMarkdownParsingFailurePolicy(pub NSInteger);
impl NSAttributedStringMarkdownParsingFailurePolicy {
    pub const NSAttributedStringMarkdownParsingFailureReturnError: Self = Self(0);
    pub const NSAttributedStringMarkdownParsingFailureReturnPartiallyParsedIfPossible: Self =
        Self(1);
}

unsafe impl Encode for NSAttributedStringMarkdownParsingFailurePolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSAttributedStringMarkdownParsingFailurePolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsattributedstringmarkdowninterpretedsyntax?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAttributedStringMarkdownInterpretedSyntax(pub NSInteger);
impl NSAttributedStringMarkdownInterpretedSyntax {
    #[doc(alias = "NSAttributedStringMarkdownInterpretedSyntaxFull")]
    pub const Full: Self = Self(0);
    #[doc(alias = "NSAttributedStringMarkdownInterpretedSyntaxInlineOnly")]
    pub const InlineOnly: Self = Self(1);
    #[doc(alias = "NSAttributedStringMarkdownInterpretedSyntaxInlineOnlyPreservingWhitespace")]
    pub const InlineOnlyPreservingWhitespace: Self = Self(2);
}

unsafe impl Encode for NSAttributedStringMarkdownInterpretedSyntax {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSAttributedStringMarkdownInterpretedSyntax {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsattributedstringmarkdownsourceposition?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAttributedStringMarkdownSourcePosition;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSAttributedStringMarkdownSourcePosition {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSAttributedStringMarkdownSourcePosition {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSAttributedStringMarkdownSourcePosition {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSAttributedStringMarkdownSourcePosition {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSAttributedStringMarkdownSourcePosition {}

extern_methods!(
    unsafe impl NSAttributedStringMarkdownSourcePosition {
        #[method(startLine)]
        pub unsafe fn startLine(&self) -> NSInteger;

        #[method(startColumn)]
        pub unsafe fn startColumn(&self) -> NSInteger;

        #[method(endLine)]
        pub unsafe fn endLine(&self) -> NSInteger;

        #[method(endColumn)]
        pub unsafe fn endColumn(&self) -> NSInteger;

        #[method_id(@__retain_semantics Init initWithStartLine:startColumn:endLine:endColumn:)]
        pub unsafe fn initWithStartLine_startColumn_endLine_endColumn(
            this: Allocated<Self>,
            start_line: NSInteger,
            start_column: NSInteger,
            end_line: NSInteger,
            end_column: NSInteger,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method(rangeInString:)]
        pub unsafe fn rangeInString(&self, string: &NSString) -> NSRange;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAttributedStringMarkdownSourcePosition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsattributedstringmarkdownparsingoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAttributedStringMarkdownParsingOptions;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSAttributedStringMarkdownParsingOptions {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSAttributedStringMarkdownParsingOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSAttributedStringMarkdownParsingOptions {}

extern_methods!(
    unsafe impl NSAttributedStringMarkdownParsingOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(allowsExtendedAttributes)]
        pub unsafe fn allowsExtendedAttributes(&self) -> bool;

        /// Setter for [`allowsExtendedAttributes`][Self::allowsExtendedAttributes].
        #[method(setAllowsExtendedAttributes:)]
        pub unsafe fn setAllowsExtendedAttributes(&self, allows_extended_attributes: bool);

        #[method(interpretedSyntax)]
        pub unsafe fn interpretedSyntax(&self) -> NSAttributedStringMarkdownInterpretedSyntax;

        /// Setter for [`interpretedSyntax`][Self::interpretedSyntax].
        #[method(setInterpretedSyntax:)]
        pub unsafe fn setInterpretedSyntax(
            &self,
            interpreted_syntax: NSAttributedStringMarkdownInterpretedSyntax,
        );

        #[method(failurePolicy)]
        pub unsafe fn failurePolicy(&self) -> NSAttributedStringMarkdownParsingFailurePolicy;

        /// Setter for [`failurePolicy`][Self::failurePolicy].
        #[method(setFailurePolicy:)]
        pub unsafe fn setFailurePolicy(
            &self,
            failure_policy: NSAttributedStringMarkdownParsingFailurePolicy,
        );

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other languageCode)]
        pub unsafe fn languageCode(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        /// Setter for [`languageCode`][Self::languageCode].
        #[method(setLanguageCode:)]
        pub unsafe fn setLanguageCode(&self, language_code: Option<&NSString>);

        #[method(appliesSourcePositionAttributes)]
        pub unsafe fn appliesSourcePositionAttributes(&self) -> bool;

        /// Setter for [`appliesSourcePositionAttributes`][Self::appliesSourcePositionAttributes].
        #[method(setAppliesSourcePositionAttributes:)]
        pub unsafe fn setAppliesSourcePositionAttributes(
            &self,
            applies_source_position_attributes: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAttributedStringMarkdownParsingOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSAttributedStringCreateFromMarkdown
    unsafe impl NSAttributedString {
        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfMarkdownFileAtURL:options:baseURL:error:_)]
        pub unsafe fn initWithContentsOfMarkdownFileAtURL_options_baseURL_error(
            this: Allocated<Self>,
            markdown_file: &NSURL,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSData", feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithMarkdown:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdown_options_baseURL_error(
            this: Allocated<Self>,
            markdown: &NSData,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithMarkdownString:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdownString_options_baseURL_error(
            this: Allocated<Self>,
            markdown_string: &NSString,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSAttributedString`
    ///
    /// NSAttributedStringCreateFromMarkdown
    unsafe impl NSMutableAttributedString {
        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfMarkdownFileAtURL:options:baseURL:error:_)]
        pub unsafe fn initWithContentsOfMarkdownFileAtURL_options_baseURL_error(
            this: Allocated<Self>,
            markdown_file: &NSURL,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSData", feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithMarkdown:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdown_options_baseURL_error(
            this: Allocated<Self>,
            markdown: &NSData,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithMarkdownString:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdownString_options_baseURL_error(
            this: Allocated<Self>,
            markdown_string: &NSString,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsattributedstringformattingoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAttributedStringFormattingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSAttributedStringFormattingOptions: NSUInteger {
        const NSAttributedStringFormattingInsertArgumentAttributesWithoutMerging = 1<<0;
        const NSAttributedStringFormattingApplyReplacementIndexAttribute = 1<<1;
    }
}

unsafe impl Encode for NSAttributedStringFormattingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSAttributedStringFormattingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSAttributedStringFormatting
    unsafe impl NSAttributedString {}
);

extern_methods!(
    /// NSMutableAttributedStringFormatting
    unsafe impl NSMutableAttributedString {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsreplacementindexattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSReplacementIndexAttributeName: &'static NSAttributedStringKey;
}

extern_methods!(
    /// NSMorphology
    unsafe impl NSAttributedString {
        /// If the string has portions tagged with NSInflectionRuleAttributeName
        /// that have no format specifiers, create a new string with those portions inflected
        /// by following the rule in the attribute.
        #[method_id(@__retain_semantics Other attributedStringByInflectingString)]
        pub unsafe fn attributedStringByInflectingString(&self) -> Retained<NSAttributedString>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmorphologyattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSMorphologyAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinflectionruleattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSInflectionRuleAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinflectionagreementargumentattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSInflectionAgreementArgumentAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinflectionagreementconceptattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSInflectionAgreementConceptAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinflectionreferentconceptattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSInflectionReferentConceptAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinflectionalternativeattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSInflectionAlternativeAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalizednumberformatattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSLocalizedNumberFormatAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspresentationintentattributename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSPresentationIntentAttributeName: &'static NSAttributedStringKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspresentationintentkind?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPresentationIntentKind(pub NSInteger);
impl NSPresentationIntentKind {
    #[doc(alias = "NSPresentationIntentKindParagraph")]
    pub const Paragraph: Self = Self(0);
    #[doc(alias = "NSPresentationIntentKindHeader")]
    pub const Header: Self = Self(1);
    #[doc(alias = "NSPresentationIntentKindOrderedList")]
    pub const OrderedList: Self = Self(2);
    #[doc(alias = "NSPresentationIntentKindUnorderedList")]
    pub const UnorderedList: Self = Self(3);
    #[doc(alias = "NSPresentationIntentKindListItem")]
    pub const ListItem: Self = Self(4);
    #[doc(alias = "NSPresentationIntentKindCodeBlock")]
    pub const CodeBlock: Self = Self(5);
    #[doc(alias = "NSPresentationIntentKindBlockQuote")]
    pub const BlockQuote: Self = Self(6);
    #[doc(alias = "NSPresentationIntentKindThematicBreak")]
    pub const ThematicBreak: Self = Self(7);
    #[doc(alias = "NSPresentationIntentKindTable")]
    pub const Table: Self = Self(8);
    #[doc(alias = "NSPresentationIntentKindTableHeaderRow")]
    pub const TableHeaderRow: Self = Self(9);
    #[doc(alias = "NSPresentationIntentKindTableRow")]
    pub const TableRow: Self = Self(10);
    #[doc(alias = "NSPresentationIntentKindTableCell")]
    pub const TableCell: Self = Self(11);
}

unsafe impl Encode for NSPresentationIntentKind {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPresentationIntentKind {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspresentationintenttablecolumnalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPresentationIntentTableColumnAlignment(pub NSInteger);
impl NSPresentationIntentTableColumnAlignment {
    #[doc(alias = "NSPresentationIntentTableColumnAlignmentLeft")]
    pub const Left: Self = Self(0);
    #[doc(alias = "NSPresentationIntentTableColumnAlignmentCenter")]
    pub const Center: Self = Self(1);
    #[doc(alias = "NSPresentationIntentTableColumnAlignmentRight")]
    pub const Right: Self = Self(2);
}

unsafe impl Encode for NSPresentationIntentTableColumnAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPresentationIntentTableColumnAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspresentationintent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPresentationIntent;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSPresentationIntent {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSPresentationIntent {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSPresentationIntent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSPresentationIntent {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSPresentationIntent {}

extern_methods!(
    unsafe impl NSPresentationIntent {
        #[method(intentKind)]
        pub unsafe fn intentKind(&self) -> NSPresentationIntentKind;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other parentIntent)]
        pub unsafe fn parentIntent(&self) -> Option<Retained<NSPresentationIntent>>;

        #[method_id(@__retain_semantics Other paragraphIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn paragraphIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other headerIntentWithIdentity:level:nestedInsideIntent:)]
        pub unsafe fn headerIntentWithIdentity_level_nestedInsideIntent(
            identity: NSInteger,
            level: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other codeBlockIntentWithIdentity:languageHint:nestedInsideIntent:)]
        pub unsafe fn codeBlockIntentWithIdentity_languageHint_nestedInsideIntent(
            identity: NSInteger,
            language_hint: Option<&NSString>,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other thematicBreakIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn thematicBreakIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other orderedListIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn orderedListIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other unorderedListIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn unorderedListIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other listItemIntentWithIdentity:ordinal:nestedInsideIntent:)]
        pub unsafe fn listItemIntentWithIdentity_ordinal_nestedInsideIntent(
            identity: NSInteger,
            ordinal: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other blockQuoteIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn blockQuoteIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[cfg(all(feature = "NSArray", feature = "NSValue"))]
        #[method_id(@__retain_semantics Other tableIntentWithIdentity:columnCount:alignments:nestedInsideIntent:)]
        pub unsafe fn tableIntentWithIdentity_columnCount_alignments_nestedInsideIntent(
            identity: NSInteger,
            column_count: NSInteger,
            alignments: &NSArray<NSNumber>,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other tableHeaderRowIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn tableHeaderRowIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other tableRowIntentWithIdentity:row:nestedInsideIntent:)]
        pub unsafe fn tableRowIntentWithIdentity_row_nestedInsideIntent(
            identity: NSInteger,
            row: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other tableCellIntentWithIdentity:column:nestedInsideIntent:)]
        pub unsafe fn tableCellIntentWithIdentity_column_nestedInsideIntent(
            identity: NSInteger,
            column: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Retained<NSPresentationIntent>;

        /// An integer value which uniquely identifies this intent in the document. Identity disambiguates attributes which apply to contiguous text -- for example, two headers in a row with the same level. It can also be used to track the location in an attributed string of a particular part of a document, even after mutation.
        #[method(identity)]
        pub unsafe fn identity(&self) -> NSInteger;

        /// If the intent is not a list, this value is 0.
        #[method(ordinal)]
        pub unsafe fn ordinal(&self) -> NSInteger;

        #[cfg(all(feature = "NSArray", feature = "NSValue"))]
        /// If the intent is not a table, this value is `nil`.
        #[method_id(@__retain_semantics Other columnAlignments)]
        pub unsafe fn columnAlignments(&self) -> Option<Retained<NSArray<NSNumber>>>;

        /// If the intent is not a table, this value is 0.
        #[method(columnCount)]
        pub unsafe fn columnCount(&self) -> NSInteger;

        /// If the intent is not a header, this value is 0.
        #[method(headerLevel)]
        pub unsafe fn headerLevel(&self) -> NSInteger;

        #[cfg(feature = "NSString")]
        /// If the intent is not a code block, this value is `nil`.
        #[method_id(@__retain_semantics Other languageHint)]
        pub unsafe fn languageHint(&self) -> Option<Retained<NSString>>;

        /// The column to which this cell belongs (0-based). If the intent is not a cell, this value is 0.
        #[method(column)]
        pub unsafe fn column(&self) -> NSInteger;

        /// The row to which this cell belongs (0-based). If the intent is not a row, this value is 0. Header rows are always row 0. If the table has more rows, those start at row 1.
        #[method(row)]
        pub unsafe fn row(&self) -> NSInteger;

        /// The indentation level of this intent. Each nested list increases the indentation level by one; all elements within the same list (and not then nested into a child list intent) have the same indentation level.
        /// Text outside list intents has an indentation level of 0.
        #[method(indentationLevel)]
        pub unsafe fn indentationLevel(&self) -> NSInteger;

        /// Returns `YES` if this intent is equivalent to the other presentation intent. Equivalence is the same as equality except that identity is not taken into account.
        #[method(isEquivalentToPresentationIntent:)]
        pub unsafe fn isEquivalentToPresentationIntent(&self, other: &NSPresentationIntent)
            -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPresentationIntent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
