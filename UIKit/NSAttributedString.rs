//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static NSFontAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSParagraphStyleAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSForegroundColorAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSBackgroundColorAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSLigatureAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSKernAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSTrackingAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSStrikethroughStyleAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSUnderlineStyleAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSStrokeColorAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSStrokeWidthAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSShadowAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSTextEffectAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSAttachmentAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSLinkAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSBaselineOffsetAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSUnderlineColorAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSStrikethroughColorAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSWritingDirectionAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSTextHighlightStyleAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSTextHighlightColorSchemeAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSAdaptiveImageGlyphAttributeName: &'static NSAttributedStringKey;
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSUnderlineStyle(pub NSInteger);
bitflags::bitflags! {
    impl NSUnderlineStyle: NSInteger {
        #[doc(alias = "NSUnderlineStyleNone")]
        const None = 0x00;
        #[doc(alias = "NSUnderlineStyleSingle")]
        const Single = 0x01;
        #[doc(alias = "NSUnderlineStyleThick")]
        const Thick = 0x02;
        #[doc(alias = "NSUnderlineStyleDouble")]
        const Double = 0x09;
        #[doc(alias = "NSUnderlineStylePatternSolid")]
        const PatternSolid = 0x0000;
        #[doc(alias = "NSUnderlineStylePatternDot")]
        const PatternDot = 0x0100;
        #[doc(alias = "NSUnderlineStylePatternDash")]
        const PatternDash = 0x0200;
        #[doc(alias = "NSUnderlineStylePatternDashDot")]
        const PatternDashDot = 0x0300;
        #[doc(alias = "NSUnderlineStylePatternDashDotDot")]
        const PatternDashDotDot = 0x0400;
        #[doc(alias = "NSUnderlineStyleByWord")]
        const ByWord = 0x8000;
    }
}

unsafe impl Encode for NSUnderlineStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSUnderlineStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWritingDirectionFormatType(pub NSInteger);
impl NSWritingDirectionFormatType {
    pub const NSWritingDirectionEmbedding: Self = Self(0 << 1);
    pub const NSWritingDirectionOverride: Self = Self(1 << 1);
}

unsafe impl Encode for NSWritingDirectionFormatType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSWritingDirectionFormatType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_ENUM
pub type NSTextEffectStyle = NSString;

extern "C" {
    pub static NSTextEffectLetterpressStyle: &'static NSTextEffectStyle;
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSTextHighlightStyle = NSString;

extern "C" {
    pub static NSTextHighlightStyleDefault: &'static NSTextHighlightStyle;
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSTextHighlightColorScheme = NSString;

extern "C" {
    pub static NSTextHighlightColorSchemeDefault: &'static NSTextHighlightColorScheme;
}

extern "C" {
    pub static NSTextHighlightColorSchemePurple: &'static NSTextHighlightColorScheme;
}

extern "C" {
    pub static NSTextHighlightColorSchemePink: &'static NSTextHighlightColorScheme;
}

extern "C" {
    pub static NSTextHighlightColorSchemeOrange: &'static NSTextHighlightColorScheme;
}

extern "C" {
    pub static NSTextHighlightColorSchemeMint: &'static NSTextHighlightColorScheme;
}

extern "C" {
    pub static NSTextHighlightColorSchemeBlue: &'static NSTextHighlightColorScheme;
}

extern_category!(
    /// Category on [`NSMutableAttributedString`].
    pub unsafe trait NSAttributedStringAttributeFixing {
        #[method(fixAttributesInRange:)]
        unsafe fn fixAttributesInRange(&self, range: NSRange);
    }

    unsafe impl NSAttributedStringAttributeFixing for NSMutableAttributedString {}
);

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSAttributedStringDocumentType = NSString;

extern "C" {
    pub static NSPlainTextDocumentType: &'static NSAttributedStringDocumentType;
}

extern "C" {
    pub static NSRTFTextDocumentType: &'static NSAttributedStringDocumentType;
}

extern "C" {
    pub static NSRTFDTextDocumentType: &'static NSAttributedStringDocumentType;
}

extern "C" {
    pub static NSHTMLTextDocumentType: &'static NSAttributedStringDocumentType;
}

// NS_TYPED_ENUM
pub type NSTextLayoutSectionKey = NSString;

extern "C" {
    pub static NSTextLayoutSectionOrientation: &'static NSTextLayoutSectionKey;
}

extern "C" {
    pub static NSTextLayoutSectionRange: &'static NSTextLayoutSectionKey;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextScalingType(pub NSInteger);
impl NSTextScalingType {
    pub const NSTextScalingStandard: Self = Self(0);
    pub const NSTextScalingiOS: Self = Self(1);
}

unsafe impl Encode for NSTextScalingType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextScalingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSAttributedStringDocumentAttributeKey = NSString;

extern "C" {
    pub static NSDocumentTypeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSCharacterEncodingDocumentAttribute:
        &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSDefaultAttributesDocumentAttribute:
        &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSPaperSizeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSViewSizeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSViewZoomDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSViewModeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSDefaultFontExcludedDocumentAttribute:
        &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSReadOnlyDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSBackgroundColorDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSHyphenationFactorDocumentAttribute:
        &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSDefaultTabIntervalDocumentAttribute:
        &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSTextLayoutSectionsAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSTextScalingDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSSourceTextScalingDocumentAttribute:
        &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSCocoaVersionDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSAttributedStringDocumentReadingOptionKey = NSString;

extern "C" {
    pub static NSDocumentTypeDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey;
}

extern "C" {
    pub static NSDefaultAttributesDocumentOption:
        &'static NSAttributedStringDocumentReadingOptionKey;
}

extern "C" {
    pub static NSCharacterEncodingDocumentOption:
        &'static NSAttributedStringDocumentReadingOptionKey;
}

extern "C" {
    pub static NSTargetTextScalingDocumentOption:
        &'static NSAttributedStringDocumentReadingOptionKey;
}

extern "C" {
    pub static NSSourceTextScalingDocumentOption:
        &'static NSAttributedStringDocumentReadingOptionKey;
}

extern "C" {
    pub static NSTextKit1ListMarkerFormatDocumentOption:
        &'static NSAttributedStringDocumentReadingOptionKey;
}

extern_category!(
    /// Category on [`NSAttributedString`].
    pub unsafe trait NSAttributedStringDocumentFormats {
        #[method_id(@__retain_semantics Init initWithURL:options:documentAttributes:error:_)]
        unsafe fn initWithURL_options_documentAttributes_error(
            this: Allocated<Self>,
            url: &NSURL,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, AnyObject>,
            dict: Option<
                &mut Option<
                    Retained<NSDictionary<NSAttributedStringDocumentAttributeKey, AnyObject>>,
                >,
            >,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init initWithData:options:documentAttributes:error:_)]
        unsafe fn initWithData_options_documentAttributes_error(
            this: Allocated<Self>,
            data: &NSData,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, AnyObject>,
            dict: Option<
                &mut Option<
                    Retained<NSDictionary<NSAttributedStringDocumentAttributeKey, AnyObject>>,
                >,
            >,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other dataFromRange:documentAttributes:error:_)]
        unsafe fn dataFromRange_documentAttributes_error(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, AnyObject>,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other fileWrapperFromRange:documentAttributes:error:_)]
        unsafe fn fileWrapperFromRange_documentAttributes_error(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, AnyObject>,
        ) -> Result<Retained<NSFileWrapper>, Retained<NSError>>;
    }

    unsafe impl NSAttributedStringDocumentFormats for NSAttributedString {}
);

extern_category!(
    /// Category on [`NSMutableAttributedString`].
    pub unsafe trait NSMutableAttributedStringDocumentFormats {
        #[method(readFromURL:options:documentAttributes:error:_)]
        unsafe fn readFromURL_options_documentAttributes_error(
            &self,
            url: &NSURL,
            opts: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, AnyObject>,
            dict: Option<
                &mut Option<
                    Retained<NSDictionary<NSAttributedStringDocumentAttributeKey, AnyObject>>,
                >,
            >,
        ) -> Result<(), Retained<NSError>>;

        #[method(readFromData:options:documentAttributes:error:_)]
        unsafe fn readFromData_options_documentAttributes_error(
            &self,
            data: &NSData,
            opts: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, AnyObject>,
            dict: Option<
                &mut Option<
                    Retained<NSDictionary<NSAttributedStringDocumentAttributeKey, AnyObject>>,
                >,
            >,
        ) -> Result<(), Retained<NSError>>;
    }

    unsafe impl NSMutableAttributedStringDocumentFormats for NSMutableAttributedString {}
);

extern_category!(
    /// Category on [`NSAttributedString`].
    pub unsafe trait NSAttributedStringKitAdditions {
        #[method(containsAttachmentsInRange:)]
        unsafe fn containsAttachmentsInRange(&self, range: NSRange) -> bool;

        #[method(prefersRTFDInRange:)]
        unsafe fn prefersRTFDInRange(&self, range: NSRange) -> bool;
    }

    unsafe impl NSAttributedStringKitAdditions for NSAttributedString {}
);

extern "C" {
    pub static NSPaperMarginDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey;
}

extern "C" {
    pub static NSObliquenessAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSExpansionAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    pub static NSVerticalGlyphFormAttributeName: &'static NSAttributedStringKey;
}

pub static NSUnderlinePatternSolid: NSUnderlineStyle =
    NSUnderlineStyle(NSUnderlineStyle::PatternSolid.0);

pub static NSUnderlinePatternDot: NSUnderlineStyle =
    NSUnderlineStyle(NSUnderlineStyle::PatternDot.0);

pub static NSUnderlinePatternDash: NSUnderlineStyle =
    NSUnderlineStyle(NSUnderlineStyle::PatternDash.0);

pub static NSUnderlinePatternDashDot: NSUnderlineStyle =
    NSUnderlineStyle(NSUnderlineStyle::PatternDashDot.0);

pub static NSUnderlinePatternDashDotDot: NSUnderlineStyle =
    NSUnderlineStyle(NSUnderlineStyle::PatternDashDotDot.0);

pub static NSUnderlineByWord: NSUnderlineStyle = NSUnderlineStyle(NSUnderlineStyle::ByWord.0);

// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextWritingDirection(pub NSInteger);
impl NSTextWritingDirection {
    #[deprecated]
    #[doc(alias = "NSTextWritingDirectionEmbedding")]
    pub const Embedding: Self = Self(0 << 1);
    #[deprecated]
    #[doc(alias = "NSTextWritingDirectionOverride")]
    pub const Override: Self = Self(1 << 1);
}

unsafe impl Encode for NSTextWritingDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextWritingDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
