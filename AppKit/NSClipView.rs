//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsclipview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSClipView;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSClipView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSClipView {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSClipView {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSClipView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSClipView {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSClipView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSClipView {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSClipView {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSClipView {
        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[method_id(@__retain_semantics Other documentView)]
        pub unsafe fn documentView(&self) -> Option<Retained<NSView>>;

        #[method(setDocumentView:)]
        pub unsafe fn setDocumentView(&self, document_view: Option<&NSView>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(documentRect)]
        pub unsafe fn documentRect(&self) -> NSRect;

        #[cfg(feature = "NSCursor")]
        #[method_id(@__retain_semantics Other documentCursor)]
        pub unsafe fn documentCursor(&self) -> Option<Retained<NSCursor>>;

        #[cfg(feature = "NSCursor")]
        #[method(setDocumentCursor:)]
        pub unsafe fn setDocumentCursor(&self, document_cursor: Option<&NSCursor>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(documentVisibleRect)]
        pub unsafe fn documentVisibleRect(&self) -> NSRect;

        #[method(viewFrameChanged:)]
        pub unsafe fn viewFrameChanged(&self, notification: &NSNotification);

        #[method(viewBoundsChanged:)]
        pub unsafe fn viewBoundsChanged(&self, notification: &NSNotification);

        #[cfg(feature = "NSEvent")]
        #[method(autoscroll:)]
        pub unsafe fn autoscroll(&self, event: &NSEvent) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(scrollToPoint:)]
        pub unsafe fn scrollToPoint(&self, new_origin: NSPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(constrainBoundsRect:)]
        pub unsafe fn constrainBoundsRect(&self, proposed_bounds: NSRect) -> NSRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSEdgeInsets;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, content_insets: NSEdgeInsets);

        #[method(automaticallyAdjustsContentInsets)]
        pub unsafe fn automaticallyAdjustsContentInsets(&self) -> bool;

        #[method(setAutomaticallyAdjustsContentInsets:)]
        pub unsafe fn setAutomaticallyAdjustsContentInsets(
            &self,
            automatically_adjusts_content_insets: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSClipView {
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
    unsafe impl NSClipView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSClipView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSClipViewSuperview
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method(reflectScrolledClipView:)]
        pub unsafe fn reflectScrolledClipView(&self, clip_view: &NSClipView);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(scrollClipView:toPoint:)]
        pub unsafe fn scrollClipView_toPoint(&self, clip_view: &NSClipView, point: NSPoint);
    }
);

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSClipView {
        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "Use -constrainBoundsRect: instead."]
        #[method(constrainScrollPoint:)]
        pub unsafe fn constrainScrollPoint(&self, new_origin: NSPoint) -> NSPoint;

        #[deprecated = "Setting this property has no effect.  NSClipView will always minimize the area of the document view that is invalidated.  To force invalidation of the document view, use -[NSView setNeedsDisplayInRect:]."]
        #[method(copiesOnScroll)]
        pub unsafe fn copiesOnScroll(&self) -> bool;

        #[deprecated = "Setting this property has no effect.  NSClipView will always minimize the area of the document view that is invalidated.  To force invalidation of the document view, use -[NSView setNeedsDisplayInRect:]."]
        #[method(setCopiesOnScroll:)]
        pub unsafe fn setCopiesOnScroll(&self, copies_on_scroll: bool);
    }
);
