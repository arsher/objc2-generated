//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstackviewgravity?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSStackViewGravity(pub NSInteger);
impl NSStackViewGravity {
    #[doc(alias = "NSStackViewGravityTop")]
    pub const Top: Self = Self(1);
    #[doc(alias = "NSStackViewGravityLeading")]
    pub const Leading: Self = Self(1);
    #[doc(alias = "NSStackViewGravityCenter")]
    pub const Center: Self = Self(2);
    #[doc(alias = "NSStackViewGravityBottom")]
    pub const Bottom: Self = Self(3);
    #[doc(alias = "NSStackViewGravityTrailing")]
    pub const Trailing: Self = Self(3);
}

unsafe impl Encode for NSStackViewGravity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSStackViewGravity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstackviewdistribution?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSStackViewDistribution(pub NSInteger);
impl NSStackViewDistribution {
    /// Default value. NSStackView will not have any special distribution behavior, relying on behavior described by gravity areas and set hugging priorities along the stacking axis.
    #[doc(alias = "NSStackViewDistributionGravityAreas")]
    pub const GravityAreas: Self = Self(-1);
    /// The effective hugging priority in the stacking axis is NSLayoutPriorityRequired, causing the stacked views to tightly fill the container along the stacking axis.
    #[doc(alias = "NSStackViewDistributionFill")]
    pub const Fill: Self = Self(0);
    /// Stacked views will have sizes maintained to be equal as much as possible along the stacking axis. The effective hugging priority in the stacking axis is NSLayoutPriorityRequired.
    #[doc(alias = "NSStackViewDistributionFillEqually")]
    pub const FillEqually: Self = Self(1);
    /// Stacked views will have sizes maintained to be equal, proportionally to their intrinsicContentSizes, as much as possible. The effective hugging priority in the stacking axis is NSLayoutPriorityRequired.
    #[doc(alias = "NSStackViewDistributionFillProportionally")]
    pub const FillProportionally: Self = Self(2);
    /// The space separating stacked views along the stacking axis are maintained to be equal as much as possible while still maintaining the minimum spacing.
    #[doc(alias = "NSStackViewDistributionEqualSpacing")]
    pub const EqualSpacing: Self = Self(3);
    /// Equal center-to-center spacing of the items is maintained as much as possible while still maintaining the minimum spacing between each view.
    #[doc(alias = "NSStackViewDistributionEqualCentering")]
    pub const EqualCentering: Self = Self(4);
}

unsafe impl Encode for NSStackViewDistribution {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSStackViewDistribution {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstackviewvisibilitypriority?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSStackViewVisibilityPriority = c_float;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstackviewvisibilityprioritymusthold?language=objc)
pub static NSStackViewVisibilityPriorityMustHold: NSStackViewVisibilityPriority = 1000 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstackviewvisibilityprioritydetachonlyifnecessary?language=objc)
pub static NSStackViewVisibilityPriorityDetachOnlyIfNecessary: NSStackViewVisibilityPriority =
    900 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstackviewvisibilityprioritynotvisible?language=objc)
pub static NSStackViewVisibilityPriorityNotVisible: NSStackViewVisibilityPriority = 0 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstackviewspacingusedefault?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub static NSStackViewSpacingUseDefault: CGFloat = c_float::MAX as _;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstackview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSStackView;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSStackView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSStackView {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSStackView {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSStackView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSStackView {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSStackView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSStackView {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSStackView {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSStackView {
        #[method_id(@__retain_semantics Other stackViewWithViews:)]
        pub unsafe fn stackViewWithViews(
            views: &NSArray<NSView>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSStackViewDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSStackViewDelegate>>,
        );

        #[cfg(feature = "NSUserInterfaceLayout")]
        /// Orientation of the StackView, defaults to NSUserInterfaceLayoutOrientationHorizontal
        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSUserInterfaceLayoutOrientation;

        #[cfg(feature = "NSUserInterfaceLayout")]
        /// Setter for [`orientation`][Self::orientation].
        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSUserInterfaceLayoutOrientation);

        #[cfg(feature = "NSLayoutConstraint")]
        /// Describes how subviews are aligned within the StackView, defaults to `NSLayoutAttributeCenterY` for horizontal stacks, `NSLayoutAttributeCenterX` for vertical stacks. Setting `NSLayoutAttributeNotAnAttribute` will cause the internal alignment constraints to not be created, and could result in an ambiguous layout. Setting an inapplicable attribute for the set orientation will result in the alignment being ignored (similar to its handling with NSLayoutAttributeNotAnAttribute). The alignment constraints are established at a priority of `NSLayoutPriorityDefaultLow` and are overridable for individual views using external constraints.
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSLayoutAttribute;

        #[cfg(feature = "NSLayoutConstraint")]
        /// Setter for [`alignment`][Self::alignment].
        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSLayoutAttribute);

        #[cfg(feature = "objc2-core-foundation")]
        /// Default padding inside the StackView, around all of the subviews.
        #[method(edgeInsets)]
        pub unsafe fn edgeInsets(&self) -> NSEdgeInsets;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`edgeInsets`][Self::edgeInsets].
        #[method(setEdgeInsets:)]
        pub unsafe fn setEdgeInsets(&self, edge_insets: NSEdgeInsets);

        /// The spacing and sizing distribution of stacked views along the primary axis. Defaults to GravityAreas.
        #[method(distribution)]
        pub unsafe fn distribution(&self) -> NSStackViewDistribution;

        /// Setter for [`distribution`][Self::distribution].
        #[method(setDistribution:)]
        pub unsafe fn setDistribution(&self, distribution: NSStackViewDistribution);

        #[cfg(feature = "objc2-core-foundation")]
        /// Default (minimum) spacing between each view
        #[method(spacing)]
        pub unsafe fn spacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`spacing`][Self::spacing].
        #[method(setSpacing:)]
        pub unsafe fn setSpacing(&self, spacing: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setCustomSpacing:afterView:)]
        pub unsafe fn setCustomSpacing_afterView(&self, spacing: CGFloat, view: &NSView);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(customSpacingAfterView:)]
        pub unsafe fn customSpacingAfterView(&self, view: &NSView) -> CGFloat;

        /// If YES, when a stacked view's `hidden` property is set to YES, the view will be detached from the stack and reattached when set to NO. Similarly, if the view has a lowered visibility priority and is detached from the stack view, it will be set as `hidden` rather than removed from the view hierarchy. Defaults to YES for apps linked on the 10.11 SDK or later.
        #[method(detachesHiddenViews)]
        pub unsafe fn detachesHiddenViews(&self) -> bool;

        /// Setter for [`detachesHiddenViews`][Self::detachesHiddenViews].
        #[method(setDetachesHiddenViews:)]
        pub unsafe fn setDetachesHiddenViews(&self, detaches_hidden_views: bool);

        /// The list of views that are arranged in a stack by the receiver. They are a subset of
        /// `-subviews,`with potential difference in ordering.
        #[method_id(@__retain_semantics Other arrangedSubviews)]
        pub unsafe fn arrangedSubviews(&self) -> Retained<NSArray<NSView>>;

        /// Adds a view to the end of the arrangedSubviews list. If the view is not a subview of the receiver, it will be added as one.
        #[method(addArrangedSubview:)]
        pub unsafe fn addArrangedSubview(&self, view: &NSView);

        /// Adds a view to the arrangedSubviews list at a specific index.
        /// If the view is already in the arrangedSubviews list, it will move the view to the specified index (but not change the subview index).
        /// If the view is not a subview of the receiver, it will be added as one (not necessarily at the same index).
        #[method(insertArrangedSubview:atIndex:)]
        pub unsafe fn insertArrangedSubview_atIndex(&self, view: &NSView, index: NSInteger);

        /// Removes a subview from the list of arranged subviews without removing it as a subview of the receiver.
        /// Removing the view as a subview (either by -[view removeFromSuperview] or setting the receiver's subviews) will automatically remove it as an arranged subview.
        #[method(removeArrangedSubview:)]
        pub unsafe fn removeArrangedSubview(&self, view: &NSView);

        /// The arrangedSubviews that are currently detached/hidden.
        #[method_id(@__retain_semantics Other detachedViews)]
        pub unsafe fn detachedViews(&self) -> Retained<NSArray<NSView>>;

        #[method(setVisibilityPriority:forView:)]
        pub unsafe fn setVisibilityPriority_forView(
            &self,
            priority: NSStackViewVisibilityPriority,
            view: &NSView,
        );

        #[method(visibilityPriorityForView:)]
        pub unsafe fn visibilityPriorityForView(
            &self,
            view: &NSView,
        ) -> NSStackViewVisibilityPriority;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method(clippingResistancePriorityForOrientation:)]
        pub unsafe fn clippingResistancePriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method(setClippingResistancePriority:forOrientation:)]
        pub unsafe fn setClippingResistancePriority_forOrientation(
            &self,
            clipping_resistance_priority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );

        #[cfg(feature = "NSLayoutConstraint")]
        #[method(huggingPriorityForOrientation:)]
        pub unsafe fn huggingPriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method(setHuggingPriority:forOrientation:)]
        pub unsafe fn setHuggingPriority_forOrientation(
            &self,
            hugging_priority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSStackView {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSStackView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSStackView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstackviewdelegate?language=objc)
    pub unsafe trait NSStackViewDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(stackView:willDetachViews:)]
        unsafe fn stackView_willDetachViews(
            &self,
            stack_view: &NSStackView,
            views: &NSArray<NSView>,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(stackView:didReattachViews:)]
        unsafe fn stackView_didReattachViews(
            &self,
            stack_view: &NSStackView,
            views: &NSArray<NSView>,
        );
    }

    unsafe impl ProtocolType for dyn NSStackViewDelegate {}
);

extern_methods!(
    /// NSStackViewGravityAreas
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSStackView {
        #[method(addView:inGravity:)]
        pub unsafe fn addView_inGravity(&self, view: &NSView, gravity: NSStackViewGravity);

        #[method(insertView:atIndex:inGravity:)]
        pub unsafe fn insertView_atIndex_inGravity(
            &self,
            view: &NSView,
            index: NSUInteger,
            gravity: NSStackViewGravity,
        );

        #[method(removeView:)]
        pub unsafe fn removeView(&self, view: &NSView);

        #[method_id(@__retain_semantics Other viewsInGravity:)]
        pub unsafe fn viewsInGravity(
            &self,
            gravity: NSStackViewGravity,
        ) -> Retained<NSArray<NSView>>;

        #[method(setViews:inGravity:)]
        pub unsafe fn setViews_inGravity(
            &self,
            views: &NSArray<NSView>,
            gravity: NSStackViewGravity,
        );

        #[method_id(@__retain_semantics Other views)]
        pub unsafe fn views(&self) -> Retained<NSArray<NSView>>;
    }
);

extern_methods!(
    /// NSStackViewDeprecated
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSStackView {
        #[deprecated = "Set -distribution to NSStackViewDistributionEqualSpacing instead."]
        #[method(hasEqualSpacing)]
        pub unsafe fn hasEqualSpacing(&self) -> bool;

        /// Setter for [`hasEqualSpacing`][Self::hasEqualSpacing].
        #[deprecated = "Set -distribution to NSStackViewDistributionEqualSpacing instead."]
        #[method(setHasEqualSpacing:)]
        pub unsafe fn setHasEqualSpacing(&self, has_equal_spacing: bool);
    }
);
