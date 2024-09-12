//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImageSymbolScale(pub NSInteger);
impl UIImageSymbolScale {
    #[doc(alias = "UIImageSymbolScaleDefault")]
    pub const Default: Self = Self(-1);
    #[doc(alias = "UIImageSymbolScaleUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "UIImageSymbolScaleSmall")]
    pub const Small: Self = Self(1);
    #[doc(alias = "UIImageSymbolScaleMedium")]
    pub const Medium: Self = Self(2);
    #[doc(alias = "UIImageSymbolScaleLarge")]
    pub const Large: Self = Self(3);
}

unsafe impl Encode for UIImageSymbolScale {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImageSymbolScale {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImageSymbolWeight(pub NSInteger);
impl UIImageSymbolWeight {
    #[doc(alias = "UIImageSymbolWeightUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "UIImageSymbolWeightUltraLight")]
    pub const UltraLight: Self = Self(1);
    #[doc(alias = "UIImageSymbolWeightThin")]
    pub const Thin: Self = Self(2);
    #[doc(alias = "UIImageSymbolWeightLight")]
    pub const Light: Self = Self(3);
    #[doc(alias = "UIImageSymbolWeightRegular")]
    pub const Regular: Self = Self(4);
    #[doc(alias = "UIImageSymbolWeightMedium")]
    pub const Medium: Self = Self(5);
    #[doc(alias = "UIImageSymbolWeightSemibold")]
    pub const Semibold: Self = Self(6);
    #[doc(alias = "UIImageSymbolWeightBold")]
    pub const Bold: Self = Self(7);
    #[doc(alias = "UIImageSymbolWeightHeavy")]
    pub const Heavy: Self = Self(8);
    #[doc(alias = "UIImageSymbolWeightBlack")]
    pub const Black: Self = Self(9);
}

unsafe impl Encode for UIImageSymbolWeight {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImageSymbolWeight {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "UIFontDescriptor")]
    pub fn UIFontWeightForImageSymbolWeight(symbol_weight: UIImageSymbolWeight) -> UIFontWeight;
}

extern "C-unwind" {
    #[cfg(feature = "UIFontDescriptor")]
    pub fn UIImageSymbolWeightForFontWeight(font_weight: UIFontWeight) -> UIImageSymbolWeight;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIImageConfiguration")]
    pub struct UIImageSymbolConfiguration;

    #[cfg(feature = "UIImageConfiguration")]
    unsafe impl ClassType for UIImageSymbolConfiguration {
        #[inherits(NSObject)]
        type Super = UIImageConfiguration;
    }
);

#[cfg(feature = "UIImageConfiguration")]
unsafe impl Send for UIImageSymbolConfiguration {}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl Sync for UIImageSymbolConfiguration {}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl NSCoding for UIImageSymbolConfiguration {}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl NSCopying for UIImageSymbolConfiguration {}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl CopyingHelper for UIImageSymbolConfiguration {
    type Result = Self;
}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl NSObjectProtocol for UIImageSymbolConfiguration {}

#[cfg(feature = "UIImageConfiguration")]
unsafe impl NSSecureCoding for UIImageSymbolConfiguration {}

extern_methods!(
    #[cfg(feature = "UIImageConfiguration")]
    unsafe impl UIImageSymbolConfiguration {
        #[method_id(@__retain_semantics Other unspecifiedConfiguration)]
        pub unsafe fn unspecifiedConfiguration() -> Retained<UIImageSymbolConfiguration>;

        #[method_id(@__retain_semantics Other configurationWithScale:)]
        pub unsafe fn configurationWithScale(scale: UIImageSymbolScale) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithPointSize:)]
        pub unsafe fn configurationWithPointSize(point_size: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithWeight:)]
        pub unsafe fn configurationWithWeight(weight: UIImageSymbolWeight) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithPointSize:weight:)]
        pub unsafe fn configurationWithPointSize_weight(
            point_size: CGFloat,
            weight: UIImageSymbolWeight,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithPointSize:weight:scale:)]
        pub unsafe fn configurationWithPointSize_weight_scale(
            point_size: CGFloat,
            weight: UIImageSymbolWeight,
            scale: UIImageSymbolScale,
        ) -> Retained<Self>;

        #[cfg(feature = "UIFontDescriptor")]
        #[method_id(@__retain_semantics Other configurationWithTextStyle:)]
        pub unsafe fn configurationWithTextStyle(text_style: &UIFontTextStyle) -> Retained<Self>;

        #[cfg(feature = "UIFontDescriptor")]
        #[method_id(@__retain_semantics Other configurationWithTextStyle:scale:)]
        pub unsafe fn configurationWithTextStyle_scale(
            text_style: &UIFontTextStyle,
            scale: UIImageSymbolScale,
        ) -> Retained<Self>;

        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other configurationWithFont:)]
        pub unsafe fn configurationWithFont(font: &UIFont) -> Retained<Self>;

        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other configurationWithFont:scale:)]
        pub unsafe fn configurationWithFont_scale(
            font: &UIFont,
            scale: UIImageSymbolScale,
        ) -> Retained<Self>;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other configurationWithHierarchicalColor:)]
        pub unsafe fn configurationWithHierarchicalColor(
            hierarchical_color: &UIColor,
        ) -> Retained<Self>;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other configurationWithPaletteColors:)]
        pub unsafe fn configurationWithPaletteColors(
            palette_colors: &NSArray<UIColor>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationPreferringMulticolor)]
        pub unsafe fn configurationPreferringMulticolor() -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationPreferringMonochrome)]
        pub unsafe fn configurationPreferringMonochrome() -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithoutTextStyle)]
        pub unsafe fn configurationWithoutTextStyle(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithoutScale)]
        pub unsafe fn configurationWithoutScale(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithoutWeight)]
        pub unsafe fn configurationWithoutWeight(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithoutPointSizeAndWeight)]
        pub unsafe fn configurationWithoutPointSizeAndWeight(&self) -> Retained<Self>;

        #[method(isEqualToConfiguration:)]
        pub unsafe fn isEqualToConfiguration(
            &self,
            other_configuration: Option<&UIImageSymbolConfiguration>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIImageConfiguration`
    #[cfg(feature = "UIImageConfiguration")]
    unsafe impl UIImageSymbolConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other configurationWithTraitCollection:)]
        pub unsafe fn configurationWithTraitCollection(
            trait_collection: Option<&UITraitCollection>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithLocale:)]
        pub unsafe fn configurationWithLocale(locale: Option<&NSLocale>) -> Retained<Self>;
    }
);
