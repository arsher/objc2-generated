//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilayoutpriority?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type UILayoutPriority = c_float;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilayoutpriorityrequired?language=objc)
pub static UILayoutPriorityRequired: UILayoutPriority = 1000 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilayoutprioritydefaulthigh?language=objc)
pub static UILayoutPriorityDefaultHigh: UILayoutPriority = 750 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilayoutprioritydragthatcanresizescene?language=objc)
pub static UILayoutPriorityDragThatCanResizeScene: UILayoutPriority = 510 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilayoutpriorityscenesizestayput?language=objc)
pub static UILayoutPrioritySceneSizeStayPut: UILayoutPriority = 500 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilayoutprioritydragthatcannotresizescene?language=objc)
pub static UILayoutPriorityDragThatCannotResizeScene: UILayoutPriority = 490 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilayoutprioritydefaultlow?language=objc)
pub static UILayoutPriorityDefaultLow: UILayoutPriority = 250 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilayoutpriorityfittingsizelevel?language=objc)
pub static UILayoutPriorityFittingSizeLevel: UILayoutPriority = 50 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nslayoutrelation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutRelation(pub NSInteger);
impl NSLayoutRelation {
    #[doc(alias = "NSLayoutRelationLessThanOrEqual")]
    pub const LessThanOrEqual: Self = Self(-1);
    #[doc(alias = "NSLayoutRelationEqual")]
    pub const Equal: Self = Self(0);
    #[doc(alias = "NSLayoutRelationGreaterThanOrEqual")]
    pub const GreaterThanOrEqual: Self = Self(1);
}

unsafe impl Encode for NSLayoutRelation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSLayoutRelation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nslayoutattribute?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutAttribute(pub NSInteger);
impl NSLayoutAttribute {
    #[doc(alias = "NSLayoutAttributeLeft")]
    pub const Left: Self = Self(1);
    #[doc(alias = "NSLayoutAttributeRight")]
    pub const Right: Self = Self(2);
    #[doc(alias = "NSLayoutAttributeTop")]
    pub const Top: Self = Self(3);
    #[doc(alias = "NSLayoutAttributeBottom")]
    pub const Bottom: Self = Self(4);
    #[doc(alias = "NSLayoutAttributeLeading")]
    pub const Leading: Self = Self(5);
    #[doc(alias = "NSLayoutAttributeTrailing")]
    pub const Trailing: Self = Self(6);
    #[doc(alias = "NSLayoutAttributeWidth")]
    pub const Width: Self = Self(7);
    #[doc(alias = "NSLayoutAttributeHeight")]
    pub const Height: Self = Self(8);
    #[doc(alias = "NSLayoutAttributeCenterX")]
    pub const CenterX: Self = Self(9);
    #[doc(alias = "NSLayoutAttributeCenterY")]
    pub const CenterY: Self = Self(10);
    #[doc(alias = "NSLayoutAttributeLastBaseline")]
    pub const LastBaseline: Self = Self(11);
    #[doc(alias = "NSLayoutAttributeBaseline")]
    pub const Baseline: Self = Self(NSLayoutAttribute::LastBaseline.0);
    #[doc(alias = "NSLayoutAttributeFirstBaseline")]
    pub const FirstBaseline: Self = Self(12);
    #[doc(alias = "NSLayoutAttributeLeftMargin")]
    pub const LeftMargin: Self = Self(13);
    #[doc(alias = "NSLayoutAttributeRightMargin")]
    pub const RightMargin: Self = Self(14);
    #[doc(alias = "NSLayoutAttributeTopMargin")]
    pub const TopMargin: Self = Self(15);
    #[doc(alias = "NSLayoutAttributeBottomMargin")]
    pub const BottomMargin: Self = Self(16);
    #[doc(alias = "NSLayoutAttributeLeadingMargin")]
    pub const LeadingMargin: Self = Self(17);
    #[doc(alias = "NSLayoutAttributeTrailingMargin")]
    pub const TrailingMargin: Self = Self(18);
    #[doc(alias = "NSLayoutAttributeCenterXWithinMargins")]
    pub const CenterXWithinMargins: Self = Self(19);
    #[doc(alias = "NSLayoutAttributeCenterYWithinMargins")]
    pub const CenterYWithinMargins: Self = Self(20);
    #[doc(alias = "NSLayoutAttributeNotAnAttribute")]
    pub const NotAnAttribute: Self = Self(0);
}

unsafe impl Encode for NSLayoutAttribute {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSLayoutAttribute {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nslayoutformatoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutFormatOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSLayoutFormatOptions: NSUInteger {
        const NSLayoutFormatAlignAllLeft = 1<<NSLayoutAttribute::Left.0;
        const NSLayoutFormatAlignAllRight = 1<<NSLayoutAttribute::Right.0;
        const NSLayoutFormatAlignAllTop = 1<<NSLayoutAttribute::Top.0;
        const NSLayoutFormatAlignAllBottom = 1<<NSLayoutAttribute::Bottom.0;
        const NSLayoutFormatAlignAllLeading = 1<<NSLayoutAttribute::Leading.0;
        const NSLayoutFormatAlignAllTrailing = 1<<NSLayoutAttribute::Trailing.0;
        const NSLayoutFormatAlignAllCenterX = 1<<NSLayoutAttribute::CenterX.0;
        const NSLayoutFormatAlignAllCenterY = 1<<NSLayoutAttribute::CenterY.0;
        const NSLayoutFormatAlignAllLastBaseline = 1<<NSLayoutAttribute::LastBaseline.0;
        const NSLayoutFormatAlignAllFirstBaseline = 1<<NSLayoutAttribute::FirstBaseline.0;
        const NSLayoutFormatAlignAllBaseline = NSLayoutFormatOptions::NSLayoutFormatAlignAllLastBaseline.0;
        const NSLayoutFormatAlignmentMask = 0xFFFF;
        const NSLayoutFormatDirectionLeadingToTrailing = 0<<16;
        const NSLayoutFormatDirectionLeftToRight = 1<<16;
        const NSLayoutFormatDirectionRightToLeft = 2<<16;
        const NSLayoutFormatDirectionMask = 0x3<<16;
        const NSLayoutFormatSpacingEdgeToEdge = 0<<19;
        const NSLayoutFormatSpacingBaselineToBaseline = 1<<19;
        const NSLayoutFormatSpacingMask = 0x1<<19;
    }
}

unsafe impl Encode for NSLayoutFormatOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSLayoutFormatOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nslayoutconstraint?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLayoutConstraint;
);

unsafe impl NSObjectProtocol for NSLayoutConstraint {}

extern_methods!(
    unsafe impl NSLayoutConstraint {
        #[method_id(@__retain_semantics Other constraintsWithVisualFormat:options:metrics:views:)]
        pub unsafe fn constraintsWithVisualFormat_options_metrics_views(
            format: &NSString,
            opts: NSLayoutFormatOptions,
            metrics: Option<&NSDictionary<NSString, AnyObject>>,
            views: &NSDictionary<NSString, AnyObject>,
            mtm: MainThreadMarker,
        ) -> Retained<NSArray<NSLayoutConstraint>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other constraintWithItem:attribute:relatedBy:toItem:attribute:multiplier:constant:)]
        pub unsafe fn constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant(
            view1: &AnyObject,
            attr1: NSLayoutAttribute,
            relation: NSLayoutRelation,
            view2: Option<&AnyObject>,
            attr2: NSLayoutAttribute,
            multiplier: CGFloat,
            c: CGFloat,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method(priority)]
        pub unsafe fn priority(&self) -> UILayoutPriority;

        #[method(setPriority:)]
        pub unsafe fn setPriority(&self, priority: UILayoutPriority);

        #[method(shouldBeArchived)]
        pub unsafe fn shouldBeArchived(&self) -> bool;

        #[method(setShouldBeArchived:)]
        pub unsafe fn setShouldBeArchived(&self, should_be_archived: bool);

        #[method_id(@__retain_semantics Other firstItem)]
        pub unsafe fn firstItem(&self) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other secondItem)]
        pub unsafe fn secondItem(&self) -> Option<Retained<AnyObject>>;

        #[method(firstAttribute)]
        pub unsafe fn firstAttribute(&self) -> NSLayoutAttribute;

        #[method(secondAttribute)]
        pub unsafe fn secondAttribute(&self) -> NSLayoutAttribute;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other firstAnchor)]
        pub unsafe fn firstAnchor(&self) -> Retained<NSLayoutAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other secondAnchor)]
        pub unsafe fn secondAnchor(&self) -> Option<Retained<NSLayoutAnchor>>;

        #[method(relation)]
        pub unsafe fn relation(&self) -> NSLayoutRelation;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(multiplier)]
        pub unsafe fn multiplier(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(constant)]
        pub unsafe fn constant(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setConstant:)]
        pub unsafe fn setConstant(&self, constant: CGFloat);

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[method(activateConstraints:)]
        pub unsafe fn activateConstraints(
            constraints: &NSArray<NSLayoutConstraint>,
            mtm: MainThreadMarker,
        );

        #[method(deactivateConstraints:)]
        pub unsafe fn deactivateConstraints(
            constraints: &NSArray<NSLayoutConstraint>,
            mtm: MainThreadMarker,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLayoutConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSIdentifier
    unsafe impl NSLayoutConstraint {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<NSString>>;

        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilayoutsupport?language=objc)
    pub unsafe trait UILayoutSupport: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(length)]
        unsafe fn length(&self) -> CGFloat;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other topAnchor)]
        unsafe fn topAnchor(&self) -> Retained<NSLayoutYAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other bottomAnchor)]
        unsafe fn bottomAnchor(&self) -> Retained<NSLayoutYAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other heightAnchor)]
        unsafe fn heightAnchor(&self) -> Retained<NSLayoutDimension>;
    }

    unsafe impl ProtocolType for dyn UILayoutSupport {}
);
