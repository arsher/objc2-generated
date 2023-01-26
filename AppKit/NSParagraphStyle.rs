//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLineBreakMode {
        NSLineBreakByWordWrapping = 0,
        NSLineBreakByCharWrapping = 1,
        NSLineBreakByClipping = 2,
        NSLineBreakByTruncatingHead = 3,
        NSLineBreakByTruncatingTail = 4,
        NSLineBreakByTruncatingMiddle = 5,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSLineBreakStrategy {
        NSLineBreakStrategyNone = 0,
        NSLineBreakStrategyPushOut = 1 << 0,
        NSLineBreakStrategyHangulWordPriority = 1 << 1,
        NSLineBreakStrategyStandard = 0xFFFF,
    }
);

typed_enum!(
    pub type NSTextTabOptionKey = NSString;
);

extern_static!(NSTabColumnTerminatorsAttributeName: &'static NSTextTabOptionKey);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextTab")]
    pub struct NSTextTab;

    #[cfg(feature = "AppKit_NSTextTab")]
    unsafe impl ClassType for NSTextTab {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTextTab")]
unsafe impl NSCoding for NSTextTab {}

#[cfg(feature = "AppKit_NSTextTab")]
unsafe impl NSObjectProtocol for NSTextTab {}

#[cfg(feature = "AppKit_NSTextTab")]
unsafe impl NSSecureCoding for NSTextTab {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextTab")]
    unsafe impl NSTextTab {
        #[cfg(all(feature = "Foundation_NSCharacterSet", feature = "Foundation_NSLocale"))]
        #[method_id(@__retain_semantics Other columnTerminatorsForLocale:)]
        pub unsafe fn columnTerminatorsForLocale(
            a_locale: Option<&NSLocale>,
        ) -> Id<NSCharacterSet, Shared>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Init initWithTextAlignment:location:options:)]
        pub unsafe fn initWithTextAlignment_location_options(
            this: Option<Allocated<Self>>,
            alignment: NSTextAlignment,
            loc: CGFloat,
            options: &NSDictionary<NSTextTabOptionKey, Object>,
        ) -> Id<Self, Shared>;

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(location)]
        pub unsafe fn location(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Id<NSDictionary<NSTextTabOptionKey, Object>, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSParagraphStyle")]
    pub struct NSParagraphStyle;

    #[cfg(feature = "AppKit_NSParagraphStyle")]
    unsafe impl ClassType for NSParagraphStyle {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSParagraphStyle")]
unsafe impl NSCoding for NSParagraphStyle {}

#[cfg(feature = "AppKit_NSParagraphStyle")]
unsafe impl NSObjectProtocol for NSParagraphStyle {}

#[cfg(feature = "AppKit_NSParagraphStyle")]
unsafe impl NSSecureCoding for NSParagraphStyle {}

extern_methods!(
    #[cfg(feature = "AppKit_NSParagraphStyle")]
    unsafe impl NSParagraphStyle {
        #[method_id(@__retain_semantics Other defaultParagraphStyle)]
        pub unsafe fn defaultParagraphStyle() -> Id<NSParagraphStyle, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(defaultWritingDirectionForLanguage:)]
        pub unsafe fn defaultWritingDirectionForLanguage(
            language_name: Option<&NSString>,
        ) -> NSWritingDirection;

        #[method(lineSpacing)]
        pub unsafe fn lineSpacing(&self) -> CGFloat;

        #[method(paragraphSpacing)]
        pub unsafe fn paragraphSpacing(&self) -> CGFloat;

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

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

        #[cfg(all(feature = "AppKit_NSTextTab", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other tabStops)]
        pub unsafe fn tabStops(&self) -> Id<NSArray<NSTextTab>, Shared>;

        #[method(defaultTabInterval)]
        pub unsafe fn defaultTabInterval(&self) -> CGFloat;

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        #[method(tighteningFactorForTruncation)]
        pub unsafe fn tighteningFactorForTruncation(&self) -> c_float;

        #[cfg(all(feature = "AppKit_NSTextBlock", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textBlocks)]
        pub unsafe fn textBlocks(&self) -> Id<NSArray<NSTextBlock>, Shared>;

        #[cfg(all(feature = "AppKit_NSTextList", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textLists)]
        pub unsafe fn textLists(&self) -> Id<NSArray<NSTextList>, Shared>;

        #[method(headerLevel)]
        pub unsafe fn headerLevel(&self) -> NSInteger;

        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSMutableParagraphStyle")]
    pub struct NSMutableParagraphStyle;

    #[cfg(feature = "AppKit_NSMutableParagraphStyle")]
    unsafe impl ClassType for NSMutableParagraphStyle {
        #[inherits(NSObject)]
        type Super = NSParagraphStyle;
    }
);

#[cfg(feature = "AppKit_NSMutableParagraphStyle")]
unsafe impl NSCoding for NSMutableParagraphStyle {}

#[cfg(feature = "AppKit_NSMutableParagraphStyle")]
unsafe impl NSObjectProtocol for NSMutableParagraphStyle {}

#[cfg(feature = "AppKit_NSMutableParagraphStyle")]
unsafe impl NSSecureCoding for NSMutableParagraphStyle {}

extern_methods!(
    #[cfg(feature = "AppKit_NSMutableParagraphStyle")]
    unsafe impl NSMutableParagraphStyle {
        #[method(lineSpacing)]
        pub unsafe fn lineSpacing(&self) -> CGFloat;

        #[method(setLineSpacing:)]
        pub unsafe fn setLineSpacing(&self, line_spacing: CGFloat);

        #[method(paragraphSpacing)]
        pub unsafe fn paragraphSpacing(&self) -> CGFloat;

        #[method(setParagraphSpacing:)]
        pub unsafe fn setParagraphSpacing(&self, paragraph_spacing: CGFloat);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

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

        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

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

        #[cfg(all(feature = "AppKit_NSTextTab", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other tabStops)]
        pub unsafe fn tabStops(&self) -> Id<NSArray<NSTextTab>, Shared>;

        #[cfg(all(feature = "AppKit_NSTextTab", feature = "Foundation_NSArray"))]
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

        #[cfg(feature = "AppKit_NSTextTab")]
        #[method(addTabStop:)]
        pub unsafe fn addTabStop(&self, an_object: &NSTextTab);

        #[cfg(feature = "AppKit_NSTextTab")]
        #[method(removeTabStop:)]
        pub unsafe fn removeTabStop(&self, an_object: &NSTextTab);

        #[method(setParagraphStyle:)]
        pub unsafe fn setParagraphStyle(&self, obj: &NSParagraphStyle);

        #[method(tighteningFactorForTruncation)]
        pub unsafe fn tighteningFactorForTruncation(&self) -> c_float;

        #[method(setTighteningFactorForTruncation:)]
        pub unsafe fn setTighteningFactorForTruncation(
            &self,
            tightening_factor_for_truncation: c_float,
        );

        #[cfg(all(feature = "AppKit_NSTextBlock", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textBlocks)]
        pub unsafe fn textBlocks(&self) -> Id<NSArray<NSTextBlock>, Shared>;

        #[cfg(all(feature = "AppKit_NSTextBlock", feature = "Foundation_NSArray"))]
        #[method(setTextBlocks:)]
        pub unsafe fn setTextBlocks(&self, text_blocks: &NSArray<NSTextBlock>);

        #[cfg(all(feature = "AppKit_NSTextList", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textLists)]
        pub unsafe fn textLists(&self) -> Id<NSArray<NSTextList>, Shared>;

        #[cfg(all(feature = "AppKit_NSTextList", feature = "Foundation_NSArray"))]
        #[method(setTextLists:)]
        pub unsafe fn setTextLists(&self, text_lists: &NSArray<NSTextList>);

        #[method(headerLevel)]
        pub unsafe fn headerLevel(&self) -> NSInteger;

        #[method(setHeaderLevel:)]
        pub unsafe fn setHeaderLevel(&self, header_level: NSInteger);

        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;

        #[method(setLineBreakStrategy:)]
        pub unsafe fn setLineBreakStrategy(&self, line_break_strategy: NSLineBreakStrategy);
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTextTabType {
        NSLeftTabStopType = 0,
        NSRightTabStopType = 1,
        NSCenterTabStopType = 2,
        NSDecimalTabStopType = 3,
    }
);

extern_methods!(
    /// NSTextTabDeprecated
    #[cfg(feature = "AppKit_NSTextTab")]
    unsafe impl NSTextTab {
        #[method_id(@__retain_semantics Init initWithType:location:)]
        pub unsafe fn initWithType_location(
            this: Option<Allocated<Self>>,
            r#type: NSTextTabType,
            loc: CGFloat,
        ) -> Id<Self, Shared>;

        #[method(tabStopType)]
        pub unsafe fn tabStopType(&self) -> NSTextTabType;
    }
);
