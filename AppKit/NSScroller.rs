//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSUsableScrollerParts {
        NSNoScrollerParts = 0,
        #[deprecated = "Scroller arrows are not used anymore."]
        NSOnlyScrollerArrows = 1,
        NSAllScrollerParts = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSScrollerPart {
        NSScrollerNoPart = 0,
        NSScrollerDecrementPage = 1,
        NSScrollerKnob = 2,
        NSScrollerIncrementPage = 3,
        #[deprecated = "Scroller arrows are not used anymore."]
        NSScrollerDecrementLine = 4,
        #[deprecated = "Scroller arrows are not used anymore."]
        NSScrollerIncrementLine = 5,
        NSScrollerKnobSlot = 6,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSScrollerStyle {
        NSScrollerStyleLegacy = 0,
        NSScrollerStyleOverlay = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSScrollerKnobStyle {
        NSScrollerKnobStyleDefault = 0,
        NSScrollerKnobStyleDark = 1,
        NSScrollerKnobStyleLight = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScroller")]
    pub struct NSScroller;

    #[cfg(feature = "AppKit_NSScroller")]
    unsafe impl ClassType for NSScroller {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
    }
);

#[cfg(feature = "AppKit_NSScroller")]
unsafe impl NSAccessibility for NSScroller {}

#[cfg(feature = "AppKit_NSScroller")]
unsafe impl NSAccessibilityElement for NSScroller {}

#[cfg(feature = "AppKit_NSScroller")]
unsafe impl NSAnimatablePropertyContainer for NSScroller {}

#[cfg(feature = "AppKit_NSScroller")]
unsafe impl NSAppearanceCustomization for NSScroller {}

#[cfg(feature = "AppKit_NSScroller")]
unsafe impl NSCoding for NSScroller {}

#[cfg(feature = "AppKit_NSScroller")]
unsafe impl NSDraggingDestination for NSScroller {}

#[cfg(feature = "AppKit_NSScroller")]
unsafe impl NSObjectProtocol for NSScroller {}

#[cfg(feature = "AppKit_NSScroller")]
unsafe impl NSUserInterfaceItemIdentification for NSScroller {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScroller")]
    unsafe impl NSScroller {
        #[method(isCompatibleWithOverlayScrollers)]
        pub unsafe fn isCompatibleWithOverlayScrollers() -> bool;

        #[method(scrollerWidthForControlSize:scrollerStyle:)]
        pub unsafe fn scrollerWidthForControlSize_scrollerStyle(
            control_size: NSControlSize,
            scroller_style: NSScrollerStyle,
        ) -> CGFloat;

        #[method(preferredScrollerStyle)]
        pub unsafe fn preferredScrollerStyle() -> NSScrollerStyle;

        #[method(scrollerStyle)]
        pub unsafe fn scrollerStyle(&self) -> NSScrollerStyle;

        #[method(setScrollerStyle:)]
        pub unsafe fn setScrollerStyle(&self, scroller_style: NSScrollerStyle);

        #[method(knobStyle)]
        pub unsafe fn knobStyle(&self) -> NSScrollerKnobStyle;

        #[method(setKnobStyle:)]
        pub unsafe fn setKnobStyle(&self, knob_style: NSScrollerKnobStyle);

        #[method(rectForPart:)]
        pub unsafe fn rectForPart(&self, part_code: NSScrollerPart) -> NSRect;

        #[method(checkSpaceForParts)]
        pub unsafe fn checkSpaceForParts(&self);

        #[method(usableParts)]
        pub unsafe fn usableParts(&self) -> NSUsableScrollerParts;

        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, control_size: NSControlSize);

        #[method(drawKnob)]
        pub unsafe fn drawKnob(&self);

        #[method(drawKnobSlotInRect:highlight:)]
        pub unsafe fn drawKnobSlotInRect_highlight(&self, slot_rect: NSRect, flag: bool);

        #[method(testPart:)]
        pub unsafe fn testPart(&self, point: NSPoint) -> NSScrollerPart;

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(trackKnob:)]
        pub unsafe fn trackKnob(&self, event: &NSEvent);

        #[method(hitPart)]
        pub unsafe fn hitPart(&self) -> NSScrollerPart;

        #[method(knobProportion)]
        pub unsafe fn knobProportion(&self) -> CGFloat;

        #[method(setKnobProportion:)]
        pub unsafe fn setKnobProportion(&self, knob_proportion: CGFloat);
    }
);

extern_static!(NSPreferredScrollerStyleDidChangeNotification: &'static NSNotificationName);

ns_enum!(
    #[underlying(NSUInteger)]
    #[deprecated = "Scroller arrows are not used anymore."]
    pub enum NSScrollArrowPosition {
        NSScrollerArrowsMaxEnd = 0,
        NSScrollerArrowsMinEnd = 1,
        NSScrollerArrowsDefaultSetting = 0,
        NSScrollerArrowsNone = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[deprecated = "Scroller arrows are not used anymore."]
    pub enum NSScrollerArrow {
        NSScrollerIncrementArrow = 0,
        NSScrollerDecrementArrow = 1,
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSScroller")]
    unsafe impl NSScroller {
        #[deprecated = "Use +scrollerWidthForControlSize:scrollerStyle: instead"]
        #[method(scrollerWidthForControlSize:)]
        pub unsafe fn scrollerWidthForControlSize(control_size: NSControlSize) -> CGFloat;

        #[deprecated = "Use +scrollerWidthForControlSize:scrollerStyle: instead"]
        #[method(scrollerWidth)]
        pub unsafe fn scrollerWidth() -> CGFloat;

        #[deprecated]
        #[method(setFloatValue:knobProportion:)]
        pub unsafe fn setFloatValue_knobProportion(&self, value: c_float, proportion: CGFloat);

        #[deprecated = "Has had no effect since 10.7"]
        #[method(arrowsPosition)]
        pub unsafe fn arrowsPosition(&self) -> NSScrollArrowPosition;

        #[deprecated = "Has had no effect since 10.7"]
        #[method(setArrowsPosition:)]
        pub unsafe fn setArrowsPosition(&self, arrows_position: NSScrollArrowPosition);

        #[deprecated = "Has had no effect since 10.7"]
        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;

        #[deprecated = "Has had no effect since 10.7"]
        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, control_tint: NSControlTint);

        #[deprecated = "Has had no effect since 10.7"]
        #[method(highlight:)]
        pub unsafe fn highlight(&self, flag: bool);

        #[cfg(feature = "AppKit_NSEvent")]
        #[deprecated = "Not invoked since 10.7"]
        #[method(trackScrollButtons:)]
        pub unsafe fn trackScrollButtons(&self, event: &NSEvent);

        #[deprecated = "Not invoked on any macOS version"]
        #[method(drawParts)]
        pub unsafe fn drawParts(&self);

        #[deprecated = "Scrollers don't have arrows as of 10.7"]
        #[method(drawArrow:highlight:)]
        pub unsafe fn drawArrow_highlight(&self, which_arrow: NSScrollerArrow, flag: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSScroller")]
    unsafe impl NSScroller {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
