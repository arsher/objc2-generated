//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilabelvibrancy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UILabelVibrancy(pub NSInteger);
impl UILabelVibrancy {
    #[doc(alias = "UILabelVibrancyNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UILabelVibrancyAutomatic")]
    pub const Automatic: Self = Self(1);
}

unsafe impl Encode for UILabelVibrancy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UILabelVibrancy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilabel?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UILabel;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UILabel {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UILabel {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UILabel {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UILabel {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UILabel {}

#[cfg(all(
    feature = "UIContentSizeCategoryAdjusting",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIContentSizeCategoryAdjusting for UILabel {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UILabel {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UILabel {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UILabel {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UILabel {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UILabel {}

#[cfg(all(
    feature = "UILetterformAwareAdjusting",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UILetterformAwareAdjusting for UILabel {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UILabel {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UILabel {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UILabel {
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Option<Retained<NSString>>;

        /// Setter for [`text`][Self::text].
        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Retained<UIFont>>;

        #[cfg(feature = "UIFont")]
        /// Setter for [`font`][Self::font].
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&UIFont>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`textColor`][Self::textColor].
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, text_color: Option<&UIColor>);

        #[method(preferredVibrancy)]
        pub unsafe fn preferredVibrancy(&self) -> UILabelVibrancy;

        /// Setter for [`preferredVibrancy`][Self::preferredVibrancy].
        #[method(setPreferredVibrancy:)]
        pub unsafe fn setPreferredVibrancy(&self, preferred_vibrancy: UILabelVibrancy);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other shadowColor)]
        pub unsafe fn shadowColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`shadowColor`][Self::shadowColor].
        #[method(setShadowColor:)]
        pub unsafe fn setShadowColor(&self, shadow_color: Option<&UIColor>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(shadowOffset)]
        pub unsafe fn shadowOffset(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`shadowOffset`][Self::shadowOffset].
        #[method(setShadowOffset:)]
        pub unsafe fn setShadowOffset(&self, shadow_offset: CGSize);

        #[cfg(feature = "NSText")]
        #[method(textAlignment)]
        pub unsafe fn textAlignment(&self) -> NSTextAlignment;

        #[cfg(feature = "NSText")]
        /// Setter for [`textAlignment`][Self::textAlignment].
        #[method(setTextAlignment:)]
        pub unsafe fn setTextAlignment(&self, text_alignment: NSTextAlignment);

        #[cfg(feature = "NSParagraphStyle")]
        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        /// Setter for [`lineBreakMode`][Self::lineBreakMode].
        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[method_id(@__retain_semantics Other attributedText)]
        pub unsafe fn attributedText(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`attributedText`][Self::attributedText].
        #[method(setAttributedText:)]
        pub unsafe fn setAttributedText(&self, attributed_text: Option<&NSAttributedString>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other highlightedTextColor)]
        pub unsafe fn highlightedTextColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`highlightedTextColor`][Self::highlightedTextColor].
        #[method(setHighlightedTextColor:)]
        pub unsafe fn setHighlightedTextColor(&self, highlighted_text_color: Option<&UIColor>);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        /// Setter for [`isHighlighted`][Self::isHighlighted].
        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[method(isUserInteractionEnabled)]
        pub unsafe fn isUserInteractionEnabled(&self) -> bool;

        /// Setter for [`isUserInteractionEnabled`][Self::isUserInteractionEnabled].
        #[method(setUserInteractionEnabled:)]
        pub unsafe fn setUserInteractionEnabled(&self, user_interaction_enabled: bool);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(numberOfLines)]
        pub unsafe fn numberOfLines(&self) -> NSInteger;

        /// Setter for [`numberOfLines`][Self::numberOfLines].
        #[method(setNumberOfLines:)]
        pub unsafe fn setNumberOfLines(&self, number_of_lines: NSInteger);

        #[method(adjustsFontSizeToFitWidth)]
        pub unsafe fn adjustsFontSizeToFitWidth(&self) -> bool;

        /// Setter for [`adjustsFontSizeToFitWidth`][Self::adjustsFontSizeToFitWidth].
        #[method(setAdjustsFontSizeToFitWidth:)]
        pub unsafe fn setAdjustsFontSizeToFitWidth(&self, adjusts_font_size_to_fit_width: bool);

        #[cfg(feature = "UIStringDrawing")]
        #[method(baselineAdjustment)]
        pub unsafe fn baselineAdjustment(&self) -> UIBaselineAdjustment;

        #[cfg(feature = "UIStringDrawing")]
        /// Setter for [`baselineAdjustment`][Self::baselineAdjustment].
        #[method(setBaselineAdjustment:)]
        pub unsafe fn setBaselineAdjustment(&self, baseline_adjustment: UIBaselineAdjustment);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumScaleFactor)]
        pub unsafe fn minimumScaleFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumScaleFactor`][Self::minimumScaleFactor].
        #[method(setMinimumScaleFactor:)]
        pub unsafe fn setMinimumScaleFactor(&self, minimum_scale_factor: CGFloat);

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        /// Setter for [`allowsDefaultTighteningForTruncation`][Self::allowsDefaultTighteningForTruncation].
        #[method(setAllowsDefaultTighteningForTruncation:)]
        pub unsafe fn setAllowsDefaultTighteningForTruncation(
            &self,
            allows_default_tightening_for_truncation: bool,
        );

        #[cfg(feature = "NSParagraphStyle")]
        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;

        #[cfg(feature = "NSParagraphStyle")]
        /// Setter for [`lineBreakStrategy`][Self::lineBreakStrategy].
        #[method(setLineBreakStrategy:)]
        pub unsafe fn setLineBreakStrategy(&self, line_break_strategy: NSLineBreakStrategy);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(textRectForBounds:limitedToNumberOfLines:)]
        pub unsafe fn textRectForBounds_limitedToNumberOfLines(
            &self,
            bounds: CGRect,
            number_of_lines: NSInteger,
        ) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawTextInRect:)]
        pub unsafe fn drawTextInRect(&self, rect: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(preferredMaxLayoutWidth)]
        pub unsafe fn preferredMaxLayoutWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`preferredMaxLayoutWidth`][Self::preferredMaxLayoutWidth].
        #[method(setPreferredMaxLayoutWidth:)]
        pub unsafe fn setPreferredMaxLayoutWidth(&self, preferred_max_layout_width: CGFloat);

        #[method(enablesMarqueeWhenAncestorFocused)]
        pub unsafe fn enablesMarqueeWhenAncestorFocused(&self) -> bool;

        /// Setter for [`enablesMarqueeWhenAncestorFocused`][Self::enablesMarqueeWhenAncestorFocused].
        #[method(setEnablesMarqueeWhenAncestorFocused:)]
        pub unsafe fn setEnablesMarqueeWhenAncestorFocused(
            &self,
            enables_marquee_when_ancestor_focused: bool,
        );

        /// Indicates whether expansion text will be shown when the view is too small to show all the contents. Defaults to NO.
        #[method(showsExpansionTextWhenTruncated)]
        pub unsafe fn showsExpansionTextWhenTruncated(&self) -> bool;

        /// Setter for [`showsExpansionTextWhenTruncated`][Self::showsExpansionTextWhenTruncated].
        #[method(setShowsExpansionTextWhenTruncated:)]
        pub unsafe fn setShowsExpansionTextWhenTruncated(
            &self,
            shows_expansion_text_when_truncated: bool,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated]
        #[method(minimumFontSize)]
        pub unsafe fn minimumFontSize(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumFontSize`][Self::minimumFontSize].
        #[deprecated]
        #[method(setMinimumFontSize:)]
        pub unsafe fn setMinimumFontSize(&self, minimum_font_size: CGFloat);

        #[deprecated]
        #[method(adjustsLetterSpacingToFitWidth)]
        pub unsafe fn adjustsLetterSpacingToFitWidth(&self) -> bool;

        /// Setter for [`adjustsLetterSpacingToFitWidth`][Self::adjustsLetterSpacingToFitWidth].
        #[deprecated]
        #[method(setAdjustsLetterSpacingToFitWidth:)]
        pub unsafe fn setAdjustsLetterSpacingToFitWidth(
            &self,
            adjusts_letter_spacing_to_fit_width: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UILabel {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UILabel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
