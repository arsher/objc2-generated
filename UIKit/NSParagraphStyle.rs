//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nslinebreakmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLineBreakMode(pub NSInteger);
impl NSLineBreakMode {
    pub const NSLineBreakByWordWrapping: Self = Self(0);
    pub const NSLineBreakByCharWrapping: Self = Self(1);
    pub const NSLineBreakByClipping: Self = Self(2);
    pub const NSLineBreakByTruncatingHead: Self = Self(3);
    pub const NSLineBreakByTruncatingTail: Self = Self(4);
    pub const NSLineBreakByTruncatingMiddle: Self = Self(5);
}

unsafe impl Encode for NSLineBreakMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSLineBreakMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nslinebreakstrategy?language=objc)
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

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstexttaboptionkey?language=objc)
// NS_TYPED_ENUM
pub type NSTextTabOptionKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstabcolumnterminatorsattributename?language=objc)
    pub static NSTabColumnTerminatorsAttributeName: &'static NSTextTabOptionKey;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstexttab?language=objc)
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
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nsparagraphstyle?language=objc)
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

        #[method(lineSpacing)]
        pub unsafe fn lineSpacing(&self) -> CGFloat;

        #[method(paragraphSpacing)]
        pub unsafe fn paragraphSpacing(&self) -> CGFloat;

        #[method(headIndent)]
        pub unsafe fn headIndent(&self) -> CGFloat;

        #[method(tailIndent)]
        pub unsafe fn tailIndent(&self) -> CGFloat;

        #[method(firstLineHeadIndent)]
        pub unsafe fn firstLineHeadIndent(&self) -> CGFloat;

        #[method(minimumLineHeight)]
        pub unsafe fn minimumLineHeight(&self) -> CGFloat;

        #[method(maximumLineHeight)]
        pub unsafe fn maximumLineHeight(&self) -> CGFloat;

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSText")]
        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[method(lineHeightMultiple)]
        pub unsafe fn lineHeightMultiple(&self) -> CGFloat;

        #[method(paragraphSpacingBefore)]
        pub unsafe fn paragraphSpacingBefore(&self) -> CGFloat;

        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;

        #[method(usesDefaultHyphenation)]
        pub unsafe fn usesDefaultHyphenation(&self) -> bool;

        #[method_id(@__retain_semantics Other tabStops)]
        pub unsafe fn tabStops(&self) -> Retained<NSArray<NSTextTab>>;

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
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nsmutableparagraphstyle?language=objc)
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
        #[method(lineSpacing)]
        pub unsafe fn lineSpacing(&self) -> CGFloat;

        #[method(setLineSpacing:)]
        pub unsafe fn setLineSpacing(&self, line_spacing: CGFloat);

        #[method(paragraphSpacing)]
        pub unsafe fn paragraphSpacing(&self) -> CGFloat;

        #[method(setParagraphSpacing:)]
        pub unsafe fn setParagraphSpacing(&self, paragraph_spacing: CGFloat);

        #[method(firstLineHeadIndent)]
        pub unsafe fn firstLineHeadIndent(&self) -> CGFloat;

        #[method(setFirstLineHeadIndent:)]
        pub unsafe fn setFirstLineHeadIndent(&self, first_line_head_indent: CGFloat);

        #[method(headIndent)]
        pub unsafe fn headIndent(&self) -> CGFloat;

        #[method(setHeadIndent:)]
        pub unsafe fn setHeadIndent(&self, head_indent: CGFloat);

        #[method(tailIndent)]
        pub unsafe fn tailIndent(&self) -> CGFloat;

        #[method(setTailIndent:)]
        pub unsafe fn setTailIndent(&self, tail_indent: CGFloat);

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[method(minimumLineHeight)]
        pub unsafe fn minimumLineHeight(&self) -> CGFloat;

        #[method(setMinimumLineHeight:)]
        pub unsafe fn setMinimumLineHeight(&self, minimum_line_height: CGFloat);

        #[method(maximumLineHeight)]
        pub unsafe fn maximumLineHeight(&self) -> CGFloat;

        #[method(setMaximumLineHeight:)]
        pub unsafe fn setMaximumLineHeight(&self, maximum_line_height: CGFloat);

        #[cfg(feature = "NSText")]
        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[cfg(feature = "NSText")]
        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, base_writing_direction: NSWritingDirection);

        #[method(lineHeightMultiple)]
        pub unsafe fn lineHeightMultiple(&self) -> CGFloat;

        #[method(setLineHeightMultiple:)]
        pub unsafe fn setLineHeightMultiple(&self, line_height_multiple: CGFloat);

        #[method(paragraphSpacingBefore)]
        pub unsafe fn paragraphSpacingBefore(&self) -> CGFloat;

        #[method(setParagraphSpacingBefore:)]
        pub unsafe fn setParagraphSpacingBefore(&self, paragraph_spacing_before: CGFloat);

        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;

        #[method(setHyphenationFactor:)]
        pub unsafe fn setHyphenationFactor(&self, hyphenation_factor: c_float);

        #[method(usesDefaultHyphenation)]
        pub unsafe fn usesDefaultHyphenation(&self) -> bool;

        #[method(setUsesDefaultHyphenation:)]
        pub unsafe fn setUsesDefaultHyphenation(&self, uses_default_hyphenation: bool);

        #[method_id(@__retain_semantics Other tabStops)]
        pub unsafe fn tabStops(&self) -> Retained<NSArray<NSTextTab>>;

        #[method(setTabStops:)]
        pub unsafe fn setTabStops(&self, tab_stops: Option<&NSArray<NSTextTab>>);

        #[method(defaultTabInterval)]
        pub unsafe fn defaultTabInterval(&self) -> CGFloat;

        #[method(setDefaultTabInterval:)]
        pub unsafe fn setDefaultTabInterval(&self, default_tab_interval: CGFloat);

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        #[method(setAllowsDefaultTighteningForTruncation:)]
        pub unsafe fn setAllowsDefaultTighteningForTruncation(
            &self,
            allows_default_tightening_for_truncation: bool,
        );

        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;

        #[method(setLineBreakStrategy:)]
        pub unsafe fn setLineBreakStrategy(&self, line_break_strategy: NSLineBreakStrategy);

        #[cfg(feature = "NSTextList")]
        #[method_id(@__retain_semantics Other textLists)]
        pub unsafe fn textLists(&self) -> Retained<NSArray<NSTextList>>;

        #[cfg(feature = "NSTextList")]
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
        #[cfg(feature = "NSText")]
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
    }
);

extern_methods!(
    unsafe impl NSMutableParagraphStyle {
        #[cfg(feature = "NSText")]
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[cfg(feature = "NSText")]
        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);
    }
);
