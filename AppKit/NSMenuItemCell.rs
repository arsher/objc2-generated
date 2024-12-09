//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsmenuitemcell?language=objc)
    #[unsafe(super(NSButtonCell, NSActionCell, NSCell, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSActionCell", feature = "NSButtonCell", feature = "NSCell"))]
    pub struct NSMenuItemCell;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSButtonCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibility for NSMenuItemCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSButtonCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSMenuItemCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSButtonCell", feature = "NSCell"))]
unsafe impl NSCoding for NSMenuItemCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSButtonCell", feature = "NSCell"))]
unsafe impl NSCopying for NSMenuItemCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSButtonCell", feature = "NSCell"))]
unsafe impl CopyingHelper for NSMenuItemCell {
    type Result = Self;
}

#[cfg(all(feature = "NSActionCell", feature = "NSButtonCell", feature = "NSCell"))]
unsafe impl NSObjectProtocol for NSMenuItemCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSButtonCell",
    feature = "NSCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSMenuItemCell {}

extern_methods!(
    #[cfg(all(feature = "NSActionCell", feature = "NSButtonCell", feature = "NSCell"))]
    unsafe impl NSMenuItemCell {
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSMenuItem")]
        #[method_id(@__retain_semantics Other menuItem)]
        pub unsafe fn menuItem(&self) -> Option<Retained<NSMenuItem>>;

        #[cfg(feature = "NSMenuItem")]
        #[method(setMenuItem:)]
        pub unsafe fn setMenuItem(&self, menu_item: Option<&NSMenuItem>);

        #[method(needsSizing)]
        pub unsafe fn needsSizing(&self) -> bool;

        #[method(setNeedsSizing:)]
        pub unsafe fn setNeedsSizing(&self, needs_sizing: bool);

        #[method(calcSize)]
        pub unsafe fn calcSize(&self);

        #[method(needsDisplay)]
        pub unsafe fn needsDisplay(&self) -> bool;

        #[method(setNeedsDisplay:)]
        pub unsafe fn setNeedsDisplay(&self, needs_display: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(stateImageWidth)]
        pub unsafe fn stateImageWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(imageWidth)]
        pub unsafe fn imageWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(titleWidth)]
        pub unsafe fn titleWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(keyEquivalentWidth)]
        pub unsafe fn keyEquivalentWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(stateImageRectForBounds:)]
        pub unsafe fn stateImageRectForBounds(&self, cell_frame: NSRect) -> NSRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(titleRectForBounds:)]
        pub unsafe fn titleRectForBounds(&self, cell_frame: NSRect) -> NSRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(keyEquivalentRectForBounds:)]
        pub unsafe fn keyEquivalentRectForBounds(&self, cell_frame: NSRect) -> NSRect;

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSView",
            feature = "objc2-core-foundation"
        ))]
        #[method(drawSeparatorItemWithFrame:inView:)]
        pub unsafe fn drawSeparatorItemWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSView",
            feature = "objc2-core-foundation"
        ))]
        #[method(drawStateImageWithFrame:inView:)]
        pub unsafe fn drawStateImageWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSView",
            feature = "objc2-core-foundation"
        ))]
        #[method(drawImageWithFrame:inView:)]
        pub unsafe fn drawImageWithFrame_inView(&self, cell_frame: NSRect, control_view: &NSView);

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSView",
            feature = "objc2-core-foundation"
        ))]
        #[method(drawTitleWithFrame:inView:)]
        pub unsafe fn drawTitleWithFrame_inView(&self, cell_frame: NSRect, control_view: &NSView);

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSView",
            feature = "objc2-core-foundation"
        ))]
        #[method(drawKeyEquivalentWithFrame:inView:)]
        pub unsafe fn drawKeyEquivalentWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSView",
            feature = "objc2-core-foundation"
        ))]
        #[method(drawBorderAndBackgroundWithFrame:inView:)]
        pub unsafe fn drawBorderAndBackgroundWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSButtonCell`
    #[cfg(all(feature = "NSActionCell", feature = "NSButtonCell", feature = "NSCell"))]
    unsafe impl NSMenuItemCell {
        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Allocated<Self>,
            image: Option<&NSImage>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(feature = "NSActionCell", feature = "NSButtonCell", feature = "NSCell"))]
    unsafe impl NSMenuItemCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSActionCell", feature = "NSButtonCell", feature = "NSCell"))]
    unsafe impl NSMenuItemCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
