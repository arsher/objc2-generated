//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nslinebreakmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLineBreakMode(pub NSUInteger);
impl NSLineBreakMode {
    pub const NSLineBreakByWordWrapping: Self = Self(0);
    pub const NSLineBreakByCharWrapping: Self = Self(1);
    pub const NSLineBreakByClipping: Self = Self(2);
    pub const NSLineBreakByTruncatingHead: Self = Self(3);
    pub const NSLineBreakByTruncatingTail: Self = Self(4);
    pub const NSLineBreakByTruncatingMiddle: Self = Self(5);
}

unsafe impl Encode for NSLineBreakMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSLineBreakMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nslinebreakstrategy?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLineBreakStrategy(pub NSUInteger);
bitflags::bitflags! {
    impl NSLineBreakStrategy: NSUInteger {
        #[doc(alias = "NSLineBreakStrategyNone")]
        const None = 0;
        #[doc(alias = "NSLineBreakStrategyPushOut")]
        const PushOut = 1<<0;
        #[doc(alias = "NSLineBreakStrategyHangulWordPriority")]
        const HangulWordPriority = 1<<1;
        #[doc(alias = "NSLineBreakStrategyStandard")]
        const Standard = 0xFFFF;
    }
}

unsafe impl Encode for NSLineBreakStrategy {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSLineBreakStrategy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstexttaboptionkey?language=objc)
// NS_TYPED_ENUM
pub type NSTextTabOptionKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstabcolumnterminatorsattributename?language=objc)
    pub static NSTabColumnTerminatorsAttributeName: &'static NSTextTabOptionKey;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstexttab?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextTab;
);

unsafe impl NSCoding for NSTextTab {}

unsafe impl NSCopying for NSTextTab {}

unsafe impl CopyingHelper for NSTextTab {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSTextTab {}

unsafe impl NSSecureCoding for NSTextTab {}

extern_methods!(
    unsafe impl NSTextTab {
        #[method_id(@__retain_semantics Other columnTerminatorsForLocale:)]
        pub unsafe fn columnTerminatorsForLocale(
            a_locale: Option<&NSLocale>,
        ) -> Retained<NSCharacterSet>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(location)]
        pub unsafe fn location(&self) -> CGFloat;

        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Retained<NSDictionary<NSTextTabOptionKey, AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextTab {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsparagraphstyle?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSParagraphStyle;
);

unsafe impl NSCoding for NSParagraphStyle {}

unsafe impl NSCopying for NSParagraphStyle {}

unsafe impl CopyingHelper for NSParagraphStyle {
    type Result = Self;
}

unsafe impl NSMutableCopying for NSParagraphStyle {}

unsafe impl MutableCopyingHelper for NSParagraphStyle {
    type Result = NSMutableParagraphStyle;
}

unsafe impl NSObjectProtocol for NSParagraphStyle {}

unsafe impl NSSecureCoding for NSParagraphStyle {}

extern_methods!(
    unsafe impl NSParagraphStyle {
        #[method_id(@__retain_semantics Other defaultParagraphStyle)]
        pub unsafe fn defaultParagraphStyle() -> Retained<NSParagraphStyle>;

        #[cfg(feature = "NSText")]
        #[method(defaultWritingDirectionForLanguage:)]
        pub unsafe fn defaultWritingDirectionForLanguage(
            language_name: Option<&NSString>,
        ) -> NSWritingDirection;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(lineSpacing)]
        pub unsafe fn lineSpacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(paragraphSpacing)]
        pub unsafe fn paragraphSpacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(headIndent)]
        pub unsafe fn headIndent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(tailIndent)]
        pub unsafe fn tailIndent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(firstLineHeadIndent)]
        pub unsafe fn firstLineHeadIndent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumLineHeight)]
        pub unsafe fn minimumLineHeight(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(maximumLineHeight)]
        pub unsafe fn maximumLineHeight(&self) -> CGFloat;

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSText")]
        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(lineHeightMultiple)]
        pub unsafe fn lineHeightMultiple(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(paragraphSpacingBefore)]
        pub unsafe fn paragraphSpacingBefore(&self) -> CGFloat;

        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;

        #[method(usesDefaultHyphenation)]
        pub unsafe fn usesDefaultHyphenation(&self) -> bool;

        #[method_id(@__retain_semantics Other tabStops)]
        pub unsafe fn tabStops(&self) -> Retained<NSArray<NSTextTab>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(defaultTabInterval)]
        pub unsafe fn defaultTabInterval(&self) -> CGFloat;

        #[cfg(feature = "NSTextList")]
        #[method_id(@__retain_semantics Other textLists)]
        pub unsafe fn textLists(&self) -> Retained<NSArray<NSTextList>>;

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSParagraphStyle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsmutableparagraphstyle?language=objc)
    #[unsafe(super(NSParagraphStyle, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMutableParagraphStyle;
);

unsafe impl NSCoding for NSMutableParagraphStyle {}

unsafe impl NSCopying for NSMutableParagraphStyle {}

unsafe impl CopyingHelper for NSMutableParagraphStyle {
    type Result = NSParagraphStyle;
}

unsafe impl NSMutableCopying for NSMutableParagraphStyle {}

unsafe impl MutableCopyingHelper for NSMutableParagraphStyle {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSMutableParagraphStyle {}

unsafe impl NSSecureCoding for NSMutableParagraphStyle {}

extern_methods!(
    unsafe impl NSMutableParagraphStyle {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(lineSpacing)]
        pub unsafe fn lineSpacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`lineSpacing`][Self::lineSpacing].
        #[method(setLineSpacing:)]
        pub unsafe fn setLineSpacing(&self, line_spacing: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(paragraphSpacing)]
        pub unsafe fn paragraphSpacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`paragraphSpacing`][Self::paragraphSpacing].
        #[method(setParagraphSpacing:)]
        pub unsafe fn setParagraphSpacing(&self, paragraph_spacing: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(firstLineHeadIndent)]
        pub unsafe fn firstLineHeadIndent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`firstLineHeadIndent`][Self::firstLineHeadIndent].
        #[method(setFirstLineHeadIndent:)]
        pub unsafe fn setFirstLineHeadIndent(&self, first_line_head_indent: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(headIndent)]
        pub unsafe fn headIndent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`headIndent`][Self::headIndent].
        #[method(setHeadIndent:)]
        pub unsafe fn setHeadIndent(&self, head_indent: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(tailIndent)]
        pub unsafe fn tailIndent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`tailIndent`][Self::tailIndent].
        #[method(setTailIndent:)]
        pub unsafe fn setTailIndent(&self, tail_indent: CGFloat);

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        /// Setter for [`lineBreakMode`][Self::lineBreakMode].
        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumLineHeight)]
        pub unsafe fn minimumLineHeight(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumLineHeight`][Self::minimumLineHeight].
        #[method(setMinimumLineHeight:)]
        pub unsafe fn setMinimumLineHeight(&self, minimum_line_height: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(maximumLineHeight)]
        pub unsafe fn maximumLineHeight(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`maximumLineHeight`][Self::maximumLineHeight].
        #[method(setMaximumLineHeight:)]
        pub unsafe fn setMaximumLineHeight(&self, maximum_line_height: CGFloat);

        #[cfg(feature = "NSText")]
        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[cfg(feature = "NSText")]
        /// Setter for [`baseWritingDirection`][Self::baseWritingDirection].
        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, base_writing_direction: NSWritingDirection);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(lineHeightMultiple)]
        pub unsafe fn lineHeightMultiple(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`lineHeightMultiple`][Self::lineHeightMultiple].
        #[method(setLineHeightMultiple:)]
        pub unsafe fn setLineHeightMultiple(&self, line_height_multiple: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(paragraphSpacingBefore)]
        pub unsafe fn paragraphSpacingBefore(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`paragraphSpacingBefore`][Self::paragraphSpacingBefore].
        #[method(setParagraphSpacingBefore:)]
        pub unsafe fn setParagraphSpacingBefore(&self, paragraph_spacing_before: CGFloat);

        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;

        /// Setter for [`hyphenationFactor`][Self::hyphenationFactor].
        #[method(setHyphenationFactor:)]
        pub unsafe fn setHyphenationFactor(&self, hyphenation_factor: c_float);

        #[method(usesDefaultHyphenation)]
        pub unsafe fn usesDefaultHyphenation(&self) -> bool;

        /// Setter for [`usesDefaultHyphenation`][Self::usesDefaultHyphenation].
        #[method(setUsesDefaultHyphenation:)]
        pub unsafe fn setUsesDefaultHyphenation(&self, uses_default_hyphenation: bool);

        #[method_id(@__retain_semantics Other tabStops)]
        pub unsafe fn tabStops(&self) -> Retained<NSArray<NSTextTab>>;

        /// Setter for [`tabStops`][Self::tabStops].
        #[method(setTabStops:)]
        pub unsafe fn setTabStops(&self, tab_stops: Option<&NSArray<NSTextTab>>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(defaultTabInterval)]
        pub unsafe fn defaultTabInterval(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`defaultTabInterval`][Self::defaultTabInterval].
        #[method(setDefaultTabInterval:)]
        pub unsafe fn setDefaultTabInterval(&self, default_tab_interval: CGFloat);

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        /// Setter for [`allowsDefaultTighteningForTruncation`][Self::allowsDefaultTighteningForTruncation].
        #[method(setAllowsDefaultTighteningForTruncation:)]
        pub unsafe fn setAllowsDefaultTighteningForTruncation(
            &self,
            allows_default_tightening_for_truncation: bool,
        );

        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;

        /// Setter for [`lineBreakStrategy`][Self::lineBreakStrategy].
        #[method(setLineBreakStrategy:)]
        pub unsafe fn setLineBreakStrategy(&self, line_break_strategy: NSLineBreakStrategy);

        #[cfg(feature = "NSTextList")]
        #[method_id(@__retain_semantics Other textLists)]
        pub unsafe fn textLists(&self) -> Retained<NSArray<NSTextList>>;

        #[cfg(feature = "NSTextList")]
        /// Setter for [`textLists`][Self::textLists].
        #[method(setTextLists:)]
        pub unsafe fn setTextLists(&self, text_lists: &NSArray<NSTextList>);

        #[method(addTabStop:)]
        pub unsafe fn addTabStop(&self, an_object: &NSTextTab);

        #[method(removeTabStop:)]
        pub unsafe fn removeTabStop(&self, an_object: &NSTextTab);

        #[method(setParagraphStyle:)]
        pub unsafe fn setParagraphStyle(&self, obj: &NSParagraphStyle);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMutableParagraphStyle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl NSTextTab {
        #[cfg(all(feature = "NSText", feature = "objc2-core-foundation"))]
        #[method_id(@__retain_semantics Init initWithTextAlignment:location:options:)]
        pub unsafe fn initWithTextAlignment_location_options(
            this: Allocated<Self>,
            alignment: NSTextAlignment,
            loc: CGFloat,
            options: &NSDictionary<NSTextTabOptionKey, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSText")]
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;
    }
);

extern_methods!(
    unsafe impl NSParagraphStyle {
        #[cfg(feature = "NSText")]
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(tighteningFactorForTruncation)]
        pub unsafe fn tighteningFactorForTruncation(&self) -> c_float;

        #[cfg(feature = "NSTextTable")]
        #[method_id(@__retain_semantics Other textBlocks)]
        pub unsafe fn textBlocks(&self) -> Retained<NSArray<NSTextBlock>>;

        #[method(headerLevel)]
        pub unsafe fn headerLevel(&self) -> NSInteger;
    }
);

extern_methods!(
    unsafe impl NSMutableParagraphStyle {
        #[cfg(feature = "NSText")]
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[cfg(feature = "NSText")]
        /// Setter for [`alignment`][Self::alignment].
        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

        #[method(tighteningFactorForTruncation)]
        pub unsafe fn tighteningFactorForTruncation(&self) -> c_float;

        /// Setter for [`tighteningFactorForTruncation`][Self::tighteningFactorForTruncation].
        #[method(setTighteningFactorForTruncation:)]
        pub unsafe fn setTighteningFactorForTruncation(
            &self,
            tightening_factor_for_truncation: c_float,
        );

        #[cfg(feature = "NSTextTable")]
        #[method_id(@__retain_semantics Other textBlocks)]
        pub unsafe fn textBlocks(&self) -> Retained<NSArray<NSTextBlock>>;

        #[cfg(feature = "NSTextTable")]
        /// Setter for [`textBlocks`][Self::textBlocks].
        #[method(setTextBlocks:)]
        pub unsafe fn setTextBlocks(&self, text_blocks: &NSArray<NSTextBlock>);

        #[method(headerLevel)]
        pub unsafe fn headerLevel(&self) -> NSInteger;

        /// Setter for [`headerLevel`][Self::headerLevel].
        #[method(setHeaderLevel:)]
        pub unsafe fn setHeaderLevel(&self, header_level: NSInteger);
    }
);

/// ********************** Deprecated ***********************
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nstexttabtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextTabType(pub NSUInteger);
impl NSTextTabType {
    pub const NSLeftTabStopType: Self = Self(0);
    pub const NSRightTabStopType: Self = Self(1);
    pub const NSCenterTabStopType: Self = Self(2);
    pub const NSDecimalTabStopType: Self = Self(3);
}

unsafe impl Encode for NSTextTabType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextTabType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSTextTabDeprecated
    unsafe impl NSTextTab {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithType:location:)]
        pub unsafe fn initWithType_location(
            this: Allocated<Self>,
            r#type: NSTextTabType,
            loc: CGFloat,
        ) -> Retained<Self>;

        #[method(tabStopType)]
        pub unsafe fn tabStopType(&self) -> NSTextTabType;
    }
);
