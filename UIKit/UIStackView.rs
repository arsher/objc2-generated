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

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uistackviewdistribution?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIStackViewDistribution(pub NSInteger);
impl UIStackViewDistribution {
    #[doc(alias = "UIStackViewDistributionFill")]
    pub const Fill: Self = Self(0);
    #[doc(alias = "UIStackViewDistributionFillEqually")]
    pub const FillEqually: Self = Self(1);
    #[doc(alias = "UIStackViewDistributionFillProportionally")]
    pub const FillProportionally: Self = Self(2);
    #[doc(alias = "UIStackViewDistributionEqualSpacing")]
    pub const EqualSpacing: Self = Self(3);
    #[doc(alias = "UIStackViewDistributionEqualCentering")]
    pub const EqualCentering: Self = Self(4);
}

unsafe impl Encode for UIStackViewDistribution {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIStackViewDistribution {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uistackviewalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIStackViewAlignment(pub NSInteger);
impl UIStackViewAlignment {
    #[doc(alias = "UIStackViewAlignmentFill")]
    pub const Fill: Self = Self(0);
    #[doc(alias = "UIStackViewAlignmentLeading")]
    pub const Leading: Self = Self(1);
    #[doc(alias = "UIStackViewAlignmentTop")]
    pub const Top: Self = Self(UIStackViewAlignment::Leading.0);
    #[doc(alias = "UIStackViewAlignmentFirstBaseline")]
    pub const FirstBaseline: Self = Self(2);
    #[doc(alias = "UIStackViewAlignmentCenter")]
    pub const Center: Self = Self(3);
    #[doc(alias = "UIStackViewAlignmentTrailing")]
    pub const Trailing: Self = Self(4);
    #[doc(alias = "UIStackViewAlignmentBottom")]
    pub const Bottom: Self = Self(UIStackViewAlignment::Trailing.0);
    #[doc(alias = "UIStackViewAlignmentLastBaseline")]
    pub const LastBaseline: Self = Self(5);
}

unsafe impl Encode for UIStackViewAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIStackViewAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uistackviewspacingusedefault?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub static UIStackViewSpacingUseDefault: CGFloat = c_float::MAX as _;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uistackviewspacingusesystem?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub static UIStackViewSpacingUseSystem: CGFloat =
    0.000000000000000000000000000000000000011754943508222875 as _;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uistackview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIStackView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIStackView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIStackView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIStackView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIStackView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIStackView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIStackView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIStackView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIStackView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIStackView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIStackView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIStackView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIStackView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIStackView {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithArrangedSubviews:)]
        pub unsafe fn initWithArrangedSubviews(
            this: Allocated<Self>,
            views: &NSArray<UIView>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other arrangedSubviews)]
        pub unsafe fn arrangedSubviews(&self) -> Retained<NSArray<UIView>>;

        #[method(addArrangedSubview:)]
        pub unsafe fn addArrangedSubview(&self, view: &UIView);

        #[method(removeArrangedSubview:)]
        pub unsafe fn removeArrangedSubview(&self, view: &UIView);

        #[method(insertArrangedSubview:atIndex:)]
        pub unsafe fn insertArrangedSubview_atIndex(&self, view: &UIView, stack_index: NSUInteger);

        #[method(axis)]
        pub unsafe fn axis(&self) -> UILayoutConstraintAxis;

        #[method(setAxis:)]
        pub unsafe fn setAxis(&self, axis: UILayoutConstraintAxis);

        #[method(distribution)]
        pub unsafe fn distribution(&self) -> UIStackViewDistribution;

        #[method(setDistribution:)]
        pub unsafe fn setDistribution(&self, distribution: UIStackViewDistribution);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> UIStackViewAlignment;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: UIStackViewAlignment);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(spacing)]
        pub unsafe fn spacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setSpacing:)]
        pub unsafe fn setSpacing(&self, spacing: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setCustomSpacing:afterView:)]
        pub unsafe fn setCustomSpacing_afterView(
            &self,
            spacing: CGFloat,
            arranged_subview: &UIView,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(customSpacingAfterView:)]
        pub unsafe fn customSpacingAfterView(&self, arranged_subview: &UIView) -> CGFloat;

        #[method(isBaselineRelativeArrangement)]
        pub unsafe fn isBaselineRelativeArrangement(&self) -> bool;

        #[method(setBaselineRelativeArrangement:)]
        pub unsafe fn setBaselineRelativeArrangement(&self, baseline_relative_arrangement: bool);

        #[method(isLayoutMarginsRelativeArrangement)]
        pub unsafe fn isLayoutMarginsRelativeArrangement(&self) -> bool;

        #[method(setLayoutMarginsRelativeArrangement:)]
        pub unsafe fn setLayoutMarginsRelativeArrangement(
            &self,
            layout_margins_relative_arrangement: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIStackView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
