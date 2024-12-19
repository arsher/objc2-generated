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

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiactivityindicatorviewstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIActivityIndicatorViewStyle(pub NSInteger);
impl UIActivityIndicatorViewStyle {
    #[doc(alias = "UIActivityIndicatorViewStyleMedium")]
    pub const Medium: Self = Self(100);
    #[doc(alias = "UIActivityIndicatorViewStyleLarge")]
    pub const Large: Self = Self(101);
    #[deprecated]
    #[doc(alias = "UIActivityIndicatorViewStyleWhiteLarge")]
    pub const WhiteLarge: Self = Self(0);
    #[deprecated]
    #[doc(alias = "UIActivityIndicatorViewStyleWhite")]
    pub const White: Self = Self(1);
    #[deprecated]
    #[doc(alias = "UIActivityIndicatorViewStyleGray")]
    pub const Gray: Self = Self(2);
}

unsafe impl Encode for UIActivityIndicatorViewStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIActivityIndicatorViewStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiactivityindicatorview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIActivityIndicatorView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIActivityIndicatorView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIActivityIndicatorView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIActivityIndicatorView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIActivityIndicatorView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIActivityIndicatorView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIActivityIndicatorView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIActivityIndicatorView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIActivityIndicatorView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIActivityIndicatorView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIActivityIndicatorView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIActivityIndicatorView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIActivityIndicatorView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIActivityIndicatorView {
        #[method_id(@__retain_semantics Init initWithActivityIndicatorStyle:)]
        pub unsafe fn initWithActivityIndicatorStyle(
            this: Allocated<Self>,
            style: UIActivityIndicatorViewStyle,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[method(activityIndicatorViewStyle)]
        pub unsafe fn activityIndicatorViewStyle(&self) -> UIActivityIndicatorViewStyle;

        #[method(setActivityIndicatorViewStyle:)]
        pub unsafe fn setActivityIndicatorViewStyle(
            &self,
            activity_indicator_view_style: UIActivityIndicatorViewStyle,
        );

        #[method(hidesWhenStopped)]
        pub unsafe fn hidesWhenStopped(&self) -> bool;

        #[method(setHidesWhenStopped:)]
        pub unsafe fn setHidesWhenStopped(&self, hides_when_stopped: bool);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: Option<&UIColor>);

        #[method(startAnimating)]
        pub unsafe fn startAnimating(&self);

        #[method(stopAnimating)]
        pub unsafe fn stopAnimating(&self);

        #[method(isAnimating)]
        pub unsafe fn isAnimating(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIActivityIndicatorView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
