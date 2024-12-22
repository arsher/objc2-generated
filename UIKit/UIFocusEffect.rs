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

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifocuseffect?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFocusEffect;
);

unsafe impl NSCopying for UIFocusEffect {}

unsafe impl CopyingHelper for UIFocusEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIFocusEffect {}

extern_methods!(
    unsafe impl UIFocusEffect {
        /// Creates a default system effect using metrics inferred from the focus item.
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifocushaloeffectposition?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIFocusHaloEffectPosition(pub NSInteger);
impl UIFocusHaloEffectPosition {
    /// Automatically detects the best position depending on the the focus item and its containing view hierarchy.
    #[doc(alias = "UIFocusHaloEffectPositionAutomatic")]
    pub const Automatic: Self = Self(0);
    /// Draws the halo around the given shape.
    #[doc(alias = "UIFocusHaloEffectPositionOutside")]
    pub const Outside: Self = Self(1);
    /// Draws the halo inside the given shape.
    #[doc(alias = "UIFocusHaloEffectPositionInside")]
    pub const Inside: Self = Self(2);
}

unsafe impl Encode for UIFocusHaloEffectPosition {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIFocusHaloEffectPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifocushaloeffect?language=objc)
    #[unsafe(super(UIFocusEffect, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFocusHaloEffect;
);

unsafe impl NSCopying for UIFocusHaloEffect {}

unsafe impl CopyingHelper for UIFocusHaloEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIFocusHaloEffect {}

extern_methods!(
    unsafe impl UIFocusHaloEffect {
        #[cfg(feature = "objc2-core-foundation")]
        /// Creates a rectangular halo.
        #[method_id(@__retain_semantics Other effectWithRect:)]
        pub unsafe fn effectWithRect(rect: CGRect) -> Retained<Self>;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
        #[cfg(not(target_os = "watchos"))]
        /// Creates a rounded rect halo using the specified corner radius and corner curve.
        #[method_id(@__retain_semantics Other effectWithRoundedRect:cornerRadius:curve:)]
        pub unsafe fn effectWithRoundedRect_cornerRadius_curve(
            rect: CGRect,
            corner_radius: CGFloat,
            curve: &CALayerCornerCurve,
        ) -> Retained<Self>;

        #[cfg(feature = "UIBezierPath")]
        /// Creates a halo with the given bezier path.
        #[method_id(@__retain_semantics Other effectWithPath:)]
        pub unsafe fn effectWithPath(bezier_path: &UIBezierPath) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// Container view in which to place the effect. When not set, the container is determined automatically
        /// from the focus item that provided this effect and the
        /// `referenceView`(if present).
        #[method_id(@__retain_semantics Other containerView)]
        pub unsafe fn containerView(&self, mtm: MainThreadMarker) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`containerView`][Self::containerView].
        #[method(setContainerView:)]
        pub unsafe fn setContainerView(&self, container_view: Option<&UIView>);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// When set, the halo is placed above this view. If a
        /// `containerView`is also set, the
        /// `referenceView`must be a descendant
        /// of the
        /// `containerView.`The system will ensure that the halo is in the container but visually above the
        /// `referenceView.`
        #[method_id(@__retain_semantics Other referenceView)]
        pub unsafe fn referenceView(&self, mtm: MainThreadMarker) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`referenceView`][Self::referenceView].
        #[method(setReferenceView:)]
        pub unsafe fn setReferenceView(&self, reference_view: Option<&UIView>);

        /// Position of the halo relative to the specified shape. Defaults to
        /// `UIFocusHaloEffectPositionAutomatic.`
        #[method(position)]
        pub unsafe fn position(&self) -> UIFocusHaloEffectPosition;

        /// Setter for [`position`][Self::position].
        #[method(setPosition:)]
        pub unsafe fn setPosition(&self, position: UIFocusHaloEffectPosition);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIFocusEffect`
    unsafe impl UIFocusHaloEffect {
        /// Creates a default system effect using metrics inferred from the focus item.
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
