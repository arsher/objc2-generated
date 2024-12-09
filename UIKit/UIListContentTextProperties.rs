//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistcontenttextalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIListContentTextAlignment(pub NSInteger);
impl UIListContentTextAlignment {
    #[doc(alias = "UIListContentTextAlignmentNatural")]
    pub const Natural: Self = Self(0);
    #[doc(alias = "UIListContentTextAlignmentCenter")]
    pub const Center: Self = Self(1);
    #[doc(alias = "UIListContentTextAlignmentJustified")]
    pub const Justified: Self = Self(2);
}

unsafe impl Encode for UIListContentTextAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIListContentTextAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistcontenttexttransform?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIListContentTextTransform(pub NSInteger);
impl UIListContentTextTransform {
    #[doc(alias = "UIListContentTextTransformNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UIListContentTextTransformUppercase")]
    pub const Uppercase: Self = Self(1);
    #[doc(alias = "UIListContentTextTransformLowercase")]
    pub const Lowercase: Self = Self(2);
    #[doc(alias = "UIListContentTextTransformCapitalized")]
    pub const Capitalized: Self = Self(3);
}

unsafe impl Encode for UIListContentTextTransform {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIListContentTextTransform {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilistcontenttextproperties?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIListContentTextProperties;
);

unsafe impl NSCoding for UIListContentTextProperties {}

unsafe impl NSCopying for UIListContentTextProperties {}

unsafe impl CopyingHelper for UIListContentTextProperties {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIListContentTextProperties {}

unsafe impl NSSecureCoding for UIListContentTextProperties {}

extern_methods!(
    unsafe impl UIListContentTextProperties {
        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Retained<UIFont>;

        #[cfg(feature = "UIFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: &UIFont);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Retained<UIColor>;

        #[cfg(feature = "UIColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &UIColor);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(colorTransformer)]
        pub unsafe fn colorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(setColorTransformer:)]
        pub unsafe fn setColorTransformer(
            &self,
            color_transformer: UIConfigurationColorTransformer,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other resolvedColor)]
        pub unsafe fn resolvedColor(&self) -> Retained<UIColor>;

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> UIListContentTextAlignment;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: UIListContentTextAlignment);

        #[cfg(feature = "NSParagraphStyle")]
        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[method(numberOfLines)]
        pub unsafe fn numberOfLines(&self) -> NSInteger;

        #[method(setNumberOfLines:)]
        pub unsafe fn setNumberOfLines(&self, number_of_lines: NSInteger);

        #[method(adjustsFontSizeToFitWidth)]
        pub unsafe fn adjustsFontSizeToFitWidth(&self) -> bool;

        #[method(setAdjustsFontSizeToFitWidth:)]
        pub unsafe fn setAdjustsFontSizeToFitWidth(&self, adjusts_font_size_to_fit_width: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumScaleFactor)]
        pub unsafe fn minimumScaleFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMinimumScaleFactor:)]
        pub unsafe fn setMinimumScaleFactor(&self, minimum_scale_factor: CGFloat);

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        #[method(setAllowsDefaultTighteningForTruncation:)]
        pub unsafe fn setAllowsDefaultTighteningForTruncation(
            &self,
            allows_default_tightening_for_truncation: bool,
        );

        #[method(adjustsFontForContentSizeCategory)]
        pub unsafe fn adjustsFontForContentSizeCategory(&self) -> bool;

        #[method(setAdjustsFontForContentSizeCategory:)]
        pub unsafe fn setAdjustsFontForContentSizeCategory(
            &self,
            adjusts_font_for_content_size_category: bool,
        );

        #[method(showsExpansionTextWhenTruncated)]
        pub unsafe fn showsExpansionTextWhenTruncated(&self) -> bool;

        #[method(setShowsExpansionTextWhenTruncated:)]
        pub unsafe fn setShowsExpansionTextWhenTruncated(
            &self,
            shows_expansion_text_when_truncated: bool,
        );

        #[method(transform)]
        pub unsafe fn transform(&self) -> UIListContentTextTransform;

        #[method(setTransform:)]
        pub unsafe fn setTransform(&self, transform: UIListContentTextTransform);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIListContentTextProperties {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
