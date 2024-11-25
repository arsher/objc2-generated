//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiblureffectstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIBlurEffectStyle(pub NSInteger);
impl UIBlurEffectStyle {
    #[doc(alias = "UIBlurEffectStyleExtraLight")]
    pub const ExtraLight: Self = Self(0);
    #[doc(alias = "UIBlurEffectStyleLight")]
    pub const Light: Self = Self(1);
    #[doc(alias = "UIBlurEffectStyleDark")]
    pub const Dark: Self = Self(2);
    #[doc(alias = "UIBlurEffectStyleExtraDark")]
    pub const ExtraDark: Self = Self(3);
    #[doc(alias = "UIBlurEffectStyleRegular")]
    pub const Regular: Self = Self(4);
    #[doc(alias = "UIBlurEffectStyleProminent")]
    pub const Prominent: Self = Self(5);
    #[doc(alias = "UIBlurEffectStyleSystemUltraThinMaterial")]
    pub const SystemUltraThinMaterial: Self = Self(6);
    #[doc(alias = "UIBlurEffectStyleSystemThinMaterial")]
    pub const SystemThinMaterial: Self = Self(7);
    #[doc(alias = "UIBlurEffectStyleSystemMaterial")]
    pub const SystemMaterial: Self = Self(8);
    #[doc(alias = "UIBlurEffectStyleSystemThickMaterial")]
    pub const SystemThickMaterial: Self = Self(9);
    #[doc(alias = "UIBlurEffectStyleSystemChromeMaterial")]
    pub const SystemChromeMaterial: Self = Self(10);
    #[doc(alias = "UIBlurEffectStyleSystemUltraThinMaterialLight")]
    pub const SystemUltraThinMaterialLight: Self = Self(11);
    #[doc(alias = "UIBlurEffectStyleSystemThinMaterialLight")]
    pub const SystemThinMaterialLight: Self = Self(12);
    #[doc(alias = "UIBlurEffectStyleSystemMaterialLight")]
    pub const SystemMaterialLight: Self = Self(13);
    #[doc(alias = "UIBlurEffectStyleSystemThickMaterialLight")]
    pub const SystemThickMaterialLight: Self = Self(14);
    #[doc(alias = "UIBlurEffectStyleSystemChromeMaterialLight")]
    pub const SystemChromeMaterialLight: Self = Self(15);
    #[doc(alias = "UIBlurEffectStyleSystemUltraThinMaterialDark")]
    pub const SystemUltraThinMaterialDark: Self = Self(16);
    #[doc(alias = "UIBlurEffectStyleSystemThinMaterialDark")]
    pub const SystemThinMaterialDark: Self = Self(17);
    #[doc(alias = "UIBlurEffectStyleSystemMaterialDark")]
    pub const SystemMaterialDark: Self = Self(18);
    #[doc(alias = "UIBlurEffectStyleSystemThickMaterialDark")]
    pub const SystemThickMaterialDark: Self = Self(19);
    #[doc(alias = "UIBlurEffectStyleSystemChromeMaterialDark")]
    pub const SystemChromeMaterialDark: Self = Self(20);
}

unsafe impl Encode for UIBlurEffectStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIBlurEffectStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiblureffect?language=objc)
    #[unsafe(super(UIVisualEffect, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIVisualEffect")]
    pub struct UIBlurEffect;
);

#[cfg(feature = "UIVisualEffect")]
unsafe impl NSCoding for UIBlurEffect {}

#[cfg(feature = "UIVisualEffect")]
unsafe impl NSCopying for UIBlurEffect {}

#[cfg(feature = "UIVisualEffect")]
unsafe impl CopyingHelper for UIBlurEffect {
    type Result = Self;
}

#[cfg(feature = "UIVisualEffect")]
unsafe impl NSObjectProtocol for UIBlurEffect {}

#[cfg(feature = "UIVisualEffect")]
unsafe impl NSSecureCoding for UIBlurEffect {}

extern_methods!(
    #[cfg(feature = "UIVisualEffect")]
    unsafe impl UIBlurEffect {
        #[method_id(@__retain_semantics Other effectWithStyle:)]
        pub unsafe fn effectWithStyle(
            style: UIBlurEffectStyle,
            mtm: MainThreadMarker,
        ) -> Retained<UIBlurEffect>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIVisualEffect")]
    unsafe impl UIBlurEffect {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
