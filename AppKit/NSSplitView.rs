//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewautosavename?language=objc)
pub type NSSplitViewAutosaveName = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewdividerstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSplitViewDividerStyle(pub NSInteger);
impl NSSplitViewDividerStyle {
    #[doc(alias = "NSSplitViewDividerStyleThick")]
    pub const Thick: Self = Self(1);
    #[doc(alias = "NSSplitViewDividerStyleThin")]
    pub const Thin: Self = Self(2);
    #[doc(alias = "NSSplitViewDividerStylePaneSplitter")]
    pub const PaneSplitter: Self = Self(3);
}

unsafe impl Encode for NSSplitViewDividerStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSSplitViewDividerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSSplitView;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSSplitView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSSplitView {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSSplitView {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSSplitView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSSplitView {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSSplitView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSSplitView {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSSplitView {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSSplitView {
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;

        #[method(setVertical:)]
        pub unsafe fn setVertical(&self, vertical: bool);

        #[method(dividerStyle)]
        pub unsafe fn dividerStyle(&self) -> NSSplitViewDividerStyle;

        #[method(setDividerStyle:)]
        pub unsafe fn setDividerStyle(&self, divider_style: NSSplitViewDividerStyle);

        #[method_id(@__retain_semantics Other autosaveName)]
        pub unsafe fn autosaveName(&self) -> Option<Retained<NSSplitViewAutosaveName>>;

        #[method(setAutosaveName:)]
        pub unsafe fn setAutosaveName(&self, autosave_name: Option<&NSSplitViewAutosaveName>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSSplitViewDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSplitViewDelegate>>,
        );

        #[method(drawDividerInRect:)]
        pub unsafe fn drawDividerInRect(&self, rect: NSRect);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other dividerColor)]
        pub unsafe fn dividerColor(&self) -> Retained<NSColor>;

        #[method(dividerThickness)]
        pub unsafe fn dividerThickness(&self) -> CGFloat;

        #[method(adjustSubviews)]
        pub unsafe fn adjustSubviews(&self);

        #[method(isSubviewCollapsed:)]
        pub unsafe fn isSubviewCollapsed(&self, subview: &NSView) -> bool;

        #[method(minPossiblePositionOfDividerAtIndex:)]
        pub unsafe fn minPossiblePositionOfDividerAtIndex(
            &self,
            divider_index: NSInteger,
        ) -> CGFloat;

        #[method(maxPossiblePositionOfDividerAtIndex:)]
        pub unsafe fn maxPossiblePositionOfDividerAtIndex(
            &self,
            divider_index: NSInteger,
        ) -> CGFloat;

        #[method(setPosition:ofDividerAtIndex:)]
        pub unsafe fn setPosition_ofDividerAtIndex(
            &self,
            position: CGFloat,
            divider_index: NSInteger,
        );

        #[cfg(feature = "NSLayoutConstraint")]
        #[method(holdingPriorityForSubviewAtIndex:)]
        pub unsafe fn holdingPriorityForSubviewAtIndex(
            &self,
            subview_index: NSInteger,
        ) -> NSLayoutPriority;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method(setHoldingPriority:forSubviewAtIndex:)]
        pub unsafe fn setHoldingPriority_forSubviewAtIndex(
            &self,
            priority: NSLayoutPriority,
            subview_index: NSInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSSplitView {
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
    unsafe impl NSSplitView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSSplitView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSSplitViewArrangedSubviews
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSSplitView {
        #[method(arrangesAllSubviews)]
        pub unsafe fn arrangesAllSubviews(&self) -> bool;

        #[method(setArrangesAllSubviews:)]
        pub unsafe fn setArrangesAllSubviews(&self, arranges_all_subviews: bool);

        #[method_id(@__retain_semantics Other arrangedSubviews)]
        pub unsafe fn arrangedSubviews(&self) -> Retained<NSArray<NSView>>;

        #[method(addArrangedSubview:)]
        pub unsafe fn addArrangedSubview(&self, view: &NSView);

        #[method(insertArrangedSubview:atIndex:)]
        pub unsafe fn insertArrangedSubview_atIndex(&self, view: &NSView, index: NSInteger);

        #[method(removeArrangedSubview:)]
        pub unsafe fn removeArrangedSubview(&self, view: &NSView);
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewdelegate?language=objc)
    pub unsafe trait NSSplitViewDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(splitView:canCollapseSubview:)]
        unsafe fn splitView_canCollapseSubview(
            &self,
            split_view: &NSSplitView,
            subview: &NSView,
        ) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "NSSplitView no longer supports collapsing sections via double-click. This delegate method is never called."]
        #[optional]
        #[method(splitView:shouldCollapseSubview:forDoubleClickOnDividerAtIndex:)]
        unsafe fn splitView_shouldCollapseSubview_forDoubleClickOnDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            subview: &NSView,
            divider_index: NSInteger,
        ) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(splitView:constrainMinCoordinate:ofSubviewAt:)]
        unsafe fn splitView_constrainMinCoordinate_ofSubviewAt(
            &self,
            split_view: &NSSplitView,
            proposed_minimum_position: CGFloat,
            divider_index: NSInteger,
        ) -> CGFloat;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(splitView:constrainMaxCoordinate:ofSubviewAt:)]
        unsafe fn splitView_constrainMaxCoordinate_ofSubviewAt(
            &self,
            split_view: &NSSplitView,
            proposed_maximum_position: CGFloat,
            divider_index: NSInteger,
        ) -> CGFloat;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(splitView:constrainSplitPosition:ofSubviewAt:)]
        unsafe fn splitView_constrainSplitPosition_ofSubviewAt(
            &self,
            split_view: &NSSplitView,
            proposed_position: CGFloat,
            divider_index: NSInteger,
        ) -> CGFloat;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(splitView:resizeSubviewsWithOldSize:)]
        unsafe fn splitView_resizeSubviewsWithOldSize(
            &self,
            split_view: &NSSplitView,
            old_size: NSSize,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(splitView:shouldAdjustSizeOfSubview:)]
        unsafe fn splitView_shouldAdjustSizeOfSubview(
            &self,
            split_view: &NSSplitView,
            view: &NSView,
        ) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(splitView:shouldHideDividerAtIndex:)]
        unsafe fn splitView_shouldHideDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(splitView:effectiveRect:forDrawnRect:ofDividerAtIndex:)]
        unsafe fn splitView_effectiveRect_forDrawnRect_ofDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            proposed_effective_rect: NSRect,
            drawn_rect: NSRect,
            divider_index: NSInteger,
        ) -> NSRect;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(splitView:additionalEffectiveRectOfDividerAtIndex:)]
        unsafe fn splitView_additionalEffectiveRectOfDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> NSRect;

        #[optional]
        #[method(splitViewWillResizeSubviews:)]
        unsafe fn splitViewWillResizeSubviews(&self, notification: &NSNotification);

        #[optional]
        #[method(splitViewDidResizeSubviews:)]
        unsafe fn splitViewDidResizeSubviews(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSSplitViewDelegate {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewwillresizesubviewsnotification?language=objc)
    pub static NSSplitViewWillResizeSubviewsNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssplitviewdidresizesubviewsnotification?language=objc)
    pub static NSSplitViewDidResizeSubviewsNotification: &'static NSNotificationName;
}

extern_methods!(
    /// NSDeprecated
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSSplitView {
        #[deprecated]
        #[method(setIsPaneSplitter:)]
        pub unsafe fn setIsPaneSplitter(&self, flag: bool);

        #[deprecated]
        #[method(isPaneSplitter)]
        pub unsafe fn isPaneSplitter(&self) -> bool;
    }
);
