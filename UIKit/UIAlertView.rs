//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uialertviewstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAlertViewStyle(pub NSInteger);
impl UIAlertViewStyle {
    #[doc(alias = "UIAlertViewStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "UIAlertViewStyleSecureTextInput")]
    pub const SecureTextInput: Self = Self(1);
    #[doc(alias = "UIAlertViewStylePlainTextInput")]
    pub const PlainTextInput: Self = Self(2);
    #[doc(alias = "UIAlertViewStyleLoginAndPasswordInput")]
    pub const LoginAndPasswordInput: Self = Self(3);
}

unsafe impl Encode for UIAlertViewStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAlertViewStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uialertview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
    pub struct UIAlertView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIAlertView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIAlertView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIAlertView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIAlertView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIAlertView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIAlertView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIAlertView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIAlertView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIAlertView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIAlertView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIAlertView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIAlertView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIAlertView {
        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<AnyObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AnyObject>);

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method_id(@__retain_semantics Other message)]
        pub unsafe fn message(&self) -> Option<Retained<NSString>>;

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(setMessage:)]
        pub unsafe fn setMessage(&self, message: Option<&NSString>);

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(addButtonWithTitle:)]
        pub unsafe fn addButtonWithTitle(&self, title: Option<&NSString>) -> NSInteger;

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method_id(@__retain_semantics Other buttonTitleAtIndex:)]
        pub unsafe fn buttonTitleAtIndex(
            &self,
            button_index: NSInteger,
        ) -> Option<Retained<NSString>>;

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(numberOfButtons)]
        pub unsafe fn numberOfButtons(&self) -> NSInteger;

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(cancelButtonIndex)]
        pub unsafe fn cancelButtonIndex(&self) -> NSInteger;

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(setCancelButtonIndex:)]
        pub unsafe fn setCancelButtonIndex(&self, cancel_button_index: NSInteger);

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(firstOtherButtonIndex)]
        pub unsafe fn firstOtherButtonIndex(&self) -> NSInteger;

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(show)]
        pub unsafe fn show(&self);

        #[deprecated = "UIAlertView is deprecated. Use UIAlertController with a preferredStyle of UIAlertControllerStyleAlert instead"]
        #[method(dismissWithClickedButtonIndex:animated:)]
        pub unsafe fn dismissWithClickedButtonIndex_animated(
            &self,
            button_index: NSInteger,
            animated: bool,
        );

        #[method(alertViewStyle)]
        pub unsafe fn alertViewStyle(&self) -> UIAlertViewStyle;

        #[method(setAlertViewStyle:)]
        pub unsafe fn setAlertViewStyle(&self, alert_view_style: UIAlertViewStyle);

        #[cfg(all(feature = "UIControl", feature = "UITextField"))]
        #[method_id(@__retain_semantics Other textFieldAtIndex:)]
        pub unsafe fn textFieldAtIndex(
            &self,
            text_field_index: NSInteger,
        ) -> Option<Retained<UITextField>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIAlertView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uialertviewdelegate?language=objc)
    pub unsafe trait UIAlertViewDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "Use UIAlertController instead."]
        #[optional]
        #[method(alertView:clickedButtonAtIndex:)]
        unsafe fn alertView_clickedButtonAtIndex(
            &self,
            alert_view: &UIAlertView,
            button_index: NSInteger,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "Use UIAlertController instead."]
        #[optional]
        #[method(alertViewCancel:)]
        unsafe fn alertViewCancel(&self, alert_view: &UIAlertView);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "Use UIAlertController instead."]
        #[optional]
        #[method(willPresentAlertView:)]
        unsafe fn willPresentAlertView(&self, alert_view: &UIAlertView);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "Use UIAlertController instead."]
        #[optional]
        #[method(didPresentAlertView:)]
        unsafe fn didPresentAlertView(&self, alert_view: &UIAlertView);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "Use UIAlertController instead."]
        #[optional]
        #[method(alertView:willDismissWithButtonIndex:)]
        unsafe fn alertView_willDismissWithButtonIndex(
            &self,
            alert_view: &UIAlertView,
            button_index: NSInteger,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "Use UIAlertController instead."]
        #[optional]
        #[method(alertView:didDismissWithButtonIndex:)]
        unsafe fn alertView_didDismissWithButtonIndex(
            &self,
            alert_view: &UIAlertView,
            button_index: NSInteger,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "Use UIAlertController instead."]
        #[optional]
        #[method(alertViewShouldEnableFirstOtherButton:)]
        unsafe fn alertViewShouldEnableFirstOtherButton(&self, alert_view: &UIAlertView) -> bool;
    }

    unsafe impl ProtocolType for dyn UIAlertViewDelegate {}
);
