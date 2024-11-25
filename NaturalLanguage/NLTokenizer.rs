//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/naturallanguage/nltokenunit?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NLTokenUnit(pub NSInteger);
impl NLTokenUnit {
    #[doc(alias = "NLTokenUnitWord")]
    pub const Word: Self = Self(0);
    #[doc(alias = "NLTokenUnitSentence")]
    pub const Sentence: Self = Self(1);
    #[doc(alias = "NLTokenUnitParagraph")]
    pub const Paragraph: Self = Self(2);
    #[doc(alias = "NLTokenUnitDocument")]
    pub const Document: Self = Self(3);
}

unsafe impl Encode for NLTokenUnit {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NLTokenUnit {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/naturallanguage/nltokenizerattributes?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NLTokenizerAttributes(pub NSUInteger);
bitflags::bitflags! {
    impl NLTokenizerAttributes: NSUInteger {
        const NLTokenizerAttributeNumeric = 1<<0;
        const NLTokenizerAttributeSymbolic = 1<<1;
        const NLTokenizerAttributeEmoji = 1<<2;
    }
}

unsafe impl Encode for NLTokenizerAttributes {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NLTokenizerAttributes {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/naturallanguage/nltokenizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NLTokenizer;
);

unsafe impl NSObjectProtocol for NLTokenizer {}

extern_methods!(
    unsafe impl NLTokenizer {
        #[method_id(@__retain_semantics Init initWithUnit:)]
        pub unsafe fn initWithUnit(this: Allocated<Self>, unit: NLTokenUnit) -> Retained<Self>;

        #[method(unit)]
        pub unsafe fn unit(&self) -> NLTokenUnit;

        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Option<Retained<NSString>>;

        #[method(setString:)]
        pub unsafe fn setString(&self, string: Option<&NSString>);

        #[cfg(feature = "NLLanguage")]
        #[method(setLanguage:)]
        pub unsafe fn setLanguage(&self, language: &NLLanguage);

        #[method(tokenRangeAtIndex:)]
        pub unsafe fn tokenRangeAtIndex(&self, character_index: NSUInteger) -> NSRange;

        #[method(tokenRangeForRange:)]
        pub unsafe fn tokenRangeForRange(&self, range: NSRange) -> NSRange;

        #[method_id(@__retain_semantics Other tokensForRange:)]
        pub unsafe fn tokensForRange(&self, range: NSRange) -> Retained<NSArray<NSValue>>;

        #[cfg(feature = "block2")]
        #[method(enumerateTokensInRange:usingBlock:)]
        pub unsafe fn enumerateTokensInRange_usingBlock(
            &self,
            range: NSRange,
            block: &block2::Block<dyn Fn(NSRange, NLTokenizerAttributes, NonNull<Bool>) + '_>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NLTokenizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
