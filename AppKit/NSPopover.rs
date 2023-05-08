//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated]
    pub enum NSPopoverAppearance {
        #[deprecated]
        NSPopoverAppearanceMinimal = 0,
        #[deprecated]
        NSPopoverAppearanceHUD = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPopoverBehavior {
        NSPopoverBehaviorApplicationDefined = 0,
        NSPopoverBehaviorTransient = 1,
        NSPopoverBehaviorSemitransient = 2,
    }
);

#[cfg(feature = "AppKit_NSPopover")]
unsafe impl NSAccessibility for NSPopover {}

#[cfg(feature = "AppKit_NSPopover")]
unsafe impl NSAccessibilityElementProtocol for NSPopover {}

#[cfg(feature = "AppKit_NSPopover")]
unsafe impl NSCoding for NSPopover {}

#[cfg(feature = "AppKit_NSPopover")]
unsafe impl NSObjectProtocol for NSPopover {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPopover")]
    unsafe impl NSPopover {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSPopoverDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSPopoverDelegate>>);

        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSPopoverBehavior;

        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: NSPopoverBehavior);

        #[method(animates)]
        pub unsafe fn animates(&self) -> bool;

        #[method(setAnimates:)]
        pub unsafe fn setAnimates(&self, animates: bool);

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other contentViewController)]
        pub unsafe fn contentViewController(&self) -> Option<Id<NSViewController>>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(
            &self,
            content_view_controller: Option<&NSViewController>,
        );

        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;

        #[method(setContentSize:)]
        pub unsafe fn setContentSize(&self, content_size: NSSize);

        #[method(isShown)]
        pub unsafe fn isShown(&self) -> bool;

        #[method(isDetached)]
        pub unsafe fn isDetached(&self) -> bool;

        #[method(positioningRect)]
        pub unsafe fn positioningRect(&self) -> NSRect;

        #[method(setPositioningRect:)]
        pub unsafe fn setPositioningRect(&self, positioning_rect: NSRect);

        #[cfg(feature = "AppKit_NSView")]
        #[method(showRelativeToRect:ofView:preferredEdge:)]
        pub unsafe fn showRelativeToRect_ofView_preferredEdge(
            &self,
            positioning_rect: NSRect,
            positioning_view: &NSView,
            preferred_edge: NSRectEdge,
        );

        #[method(performClose:)]
        pub unsafe fn performClose(&self, sender: Option<&Object>);

        #[method(close)]
        pub unsafe fn close(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSPopover")]
    unsafe impl NSPopover {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSPopoverCloseReasonKey: &'static NSString);

typed_enum!(
    pub type NSPopoverCloseReasonValue = NSString;
);

extern_static!(NSPopoverCloseReasonStandard: &'static NSPopoverCloseReasonValue);

extern_static!(NSPopoverCloseReasonDetachToWindow: &'static NSPopoverCloseReasonValue);

extern_static!(NSPopoverWillShowNotification: &'static NSNotificationName);

extern_static!(NSPopoverDidShowNotification: &'static NSNotificationName);

extern_static!(NSPopoverWillCloseNotification: &'static NSNotificationName);

extern_static!(NSPopoverDidCloseNotification: &'static NSNotificationName);

extern_protocol!(
    pub unsafe trait NSPopoverDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSPopover")]
        #[optional]
        #[method(popoverShouldClose:)]
        unsafe fn popoverShouldClose(&self, popover: &NSPopover) -> bool;

        #[cfg(feature = "AppKit_NSPopover")]
        #[optional]
        #[method(popoverShouldDetach:)]
        unsafe fn popoverShouldDetach(&self, popover: &NSPopover) -> bool;

        #[cfg(feature = "AppKit_NSPopover")]
        #[optional]
        #[method(popoverDidDetach:)]
        unsafe fn popoverDidDetach(&self, popover: &NSPopover);

        #[cfg(all(feature = "AppKit_NSPopover", feature = "AppKit_NSWindow"))]
        #[optional]
        #[method_id(@__retain_semantics Other detachableWindowForPopover:)]
        unsafe fn detachableWindowForPopover(&self, popover: &NSPopover) -> Option<Id<NSWindow>>;

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(popoverWillShow:)]
        unsafe fn popoverWillShow(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(popoverDidShow:)]
        unsafe fn popoverDidShow(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(popoverWillClose:)]
        unsafe fn popoverWillClose(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(popoverDidClose:)]
        unsafe fn popoverDidClose(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSPopoverDelegate {}
);
