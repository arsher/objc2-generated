//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowscenepresentationstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIWindowScenePresentationStyle(pub NSUInteger);
impl UIWindowScenePresentationStyle {
    #[doc(alias = "UIWindowScenePresentationStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIWindowScenePresentationStyleStandard")]
    pub const Standard: Self = Self(1);
    #[doc(alias = "UIWindowScenePresentationStyleProminent")]
    pub const Prominent: Self = Self(2);
}

unsafe impl Encode for UIWindowScenePresentationStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIWindowScenePresentationStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowsceneactivationrequestoptions?language=objc)
    #[unsafe(super(UISceneActivationRequestOptions, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UISceneOptions")]
    pub struct UIWindowSceneActivationRequestOptions;
);

#[cfg(feature = "UISceneOptions")]
unsafe impl NSObjectProtocol for UIWindowSceneActivationRequestOptions {}

extern_methods!(
    #[cfg(feature = "UISceneOptions")]
    unsafe impl UIWindowSceneActivationRequestOptions {
        #[deprecated = "Place use .placement with an appropriate UIWindowScenePlacement."]
        #[method(preferredPresentationStyle)]
        pub unsafe fn preferredPresentationStyle(&self) -> UIWindowScenePresentationStyle;

        #[deprecated = "Place use .placement with an appropriate UIWindowScenePlacement."]
        #[method(setPreferredPresentationStyle:)]
        pub unsafe fn setPreferredPresentationStyle(
            &self,
            preferred_presentation_style: UIWindowScenePresentationStyle,
        );

        #[cfg(feature = "UIWindowScenePlacement")]
        #[method_id(@__retain_semantics Other placement)]
        pub unsafe fn placement(&self) -> Option<Retained<UIWindowScenePlacement>>;

        #[cfg(feature = "UIWindowScenePlacement")]
        #[method(setPlacement:)]
        pub unsafe fn setPlacement(&self, placement: Option<&UIWindowScenePlacement>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UISceneOptions")]
    unsafe impl UIWindowSceneActivationRequestOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
