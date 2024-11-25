//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uikeyboardlayoutguide?language=objc)
    #[unsafe(super(UITrackingLayoutGuide, UILayoutGuide, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UILayoutGuide", feature = "UITrackingLayoutGuide"))]
    pub struct UIKeyboardLayoutGuide;
);

#[cfg(all(feature = "UILayoutGuide", feature = "UITrackingLayoutGuide"))]
unsafe impl NSCoding for UIKeyboardLayoutGuide {}

#[cfg(all(feature = "UILayoutGuide", feature = "UITrackingLayoutGuide"))]
unsafe impl NSObjectProtocol for UIKeyboardLayoutGuide {}

extern_methods!(
    #[cfg(all(feature = "UILayoutGuide", feature = "UITrackingLayoutGuide"))]
    unsafe impl UIKeyboardLayoutGuide {
        #[method(followsUndockedKeyboard)]
        pub unsafe fn followsUndockedKeyboard(&self) -> bool;

        #[method(setFollowsUndockedKeyboard:)]
        pub unsafe fn setFollowsUndockedKeyboard(&self, follows_undocked_keyboard: bool);

        #[method(usesBottomSafeArea)]
        pub unsafe fn usesBottomSafeArea(&self) -> bool;

        #[method(setUsesBottomSafeArea:)]
        pub unsafe fn setUsesBottomSafeArea(&self, uses_bottom_safe_area: bool);

        #[method(keyboardDismissPadding)]
        pub unsafe fn keyboardDismissPadding(&self) -> CGFloat;

        #[method(setKeyboardDismissPadding:)]
        pub unsafe fn setKeyboardDismissPadding(&self, keyboard_dismiss_padding: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UILayoutGuide", feature = "UITrackingLayoutGuide"))]
    unsafe impl UIKeyboardLayoutGuide {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
