//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontentunavailableimageproperties?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIContentUnavailableImageProperties;
);

unsafe impl NSCoding for UIContentUnavailableImageProperties {}

unsafe impl NSCopying for UIContentUnavailableImageProperties {}

unsafe impl CopyingHelper for UIContentUnavailableImageProperties {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIContentUnavailableImageProperties {}

unsafe impl NSSecureCoding for UIContentUnavailableImageProperties {}

extern_methods!(
    unsafe impl UIContentUnavailableImageProperties {
        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method_id(@__retain_semantics Other preferredSymbolConfiguration)]
        pub unsafe fn preferredSymbolConfiguration(
            &self,
        ) -> Option<Retained<UIImageSymbolConfiguration>>;

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method(setPreferredSymbolConfiguration:)]
        pub unsafe fn setPreferredSymbolConfiguration(
            &self,
            preferred_symbol_configuration: Option<&UIImageSymbolConfiguration>,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other tintColor)]
        pub unsafe fn tintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);

        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);

        #[method(maximumSize)]
        pub unsafe fn maximumSize(&self) -> CGSize;

        #[method(setMaximumSize:)]
        pub unsafe fn setMaximumSize(&self, maximum_size: CGSize);

        #[method(accessibilityIgnoresInvertColors)]
        pub unsafe fn accessibilityIgnoresInvertColors(&self) -> bool;

        #[method(setAccessibilityIgnoresInvertColors:)]
        pub unsafe fn setAccessibilityIgnoresInvertColors(
            &self,
            accessibility_ignores_invert_colors: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIContentUnavailableImageProperties {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
