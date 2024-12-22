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

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipopoverbackgroundviewmethods?language=objc)
    pub unsafe trait UIPopoverBackgroundViewMethods {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(arrowBase)]
        unsafe fn arrowBase() -> CGFloat;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(contentViewInsets)]
        unsafe fn contentViewInsets() -> UIEdgeInsets;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(arrowHeight)]
        unsafe fn arrowHeight() -> CGFloat;
    }

    unsafe impl ProtocolType for dyn UIPopoverBackgroundViewMethods {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipopoverbackgroundview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIPopoverBackgroundView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIPopoverBackgroundView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIPopoverBackgroundView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIPopoverBackgroundView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIPopoverBackgroundView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIPopoverBackgroundView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIPopoverBackgroundView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIPopoverBackgroundView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIPopoverBackgroundView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIPopoverBackgroundView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIPopoverBackgroundView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIPopoverBackgroundViewMethods for UIPopoverBackgroundView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIPopoverBackgroundView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIPopoverBackgroundView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIPopoverBackgroundView {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(arrowOffset)]
        pub unsafe fn arrowOffset(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`arrowOffset`][Self::arrowOffset].
        #[method(setArrowOffset:)]
        pub unsafe fn setArrowOffset(&self, arrow_offset: CGFloat);

        #[cfg(feature = "UIPopoverSupport")]
        #[method(arrowDirection)]
        pub unsafe fn arrowDirection(&self) -> UIPopoverArrowDirection;

        #[cfg(feature = "UIPopoverSupport")]
        /// Setter for [`arrowDirection`][Self::arrowDirection].
        #[method(setArrowDirection:)]
        pub unsafe fn setArrowDirection(&self, arrow_direction: UIPopoverArrowDirection);

        #[deprecated = "No longer supported"]
        #[method(wantsDefaultContentAppearance)]
        pub unsafe fn wantsDefaultContentAppearance(mtm: MainThreadMarker) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIPopoverBackgroundView {
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
    unsafe impl UIPopoverBackgroundView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
