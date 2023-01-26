//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSStatusBarButton")]
    pub struct NSStatusBarButton;

    #[cfg(feature = "AppKit_NSStatusBarButton")]
    unsafe impl ClassType for NSStatusBarButton {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSButton;
    }
);

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSAccessibility for NSStatusBarButton {}

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSAccessibilityButton for NSStatusBarButton {}

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSAccessibilityElement for NSStatusBarButton {}

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSAnimatablePropertyContainer for NSStatusBarButton {}

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSAppearanceCustomization for NSStatusBarButton {}

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSCoding for NSStatusBarButton {}

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSDraggingDestination for NSStatusBarButton {}

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSObjectProtocol for NSStatusBarButton {}

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSUserInterfaceCompression for NSStatusBarButton {}

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSUserInterfaceItemIdentification for NSStatusBarButton {}

#[cfg(feature = "AppKit_NSStatusBarButton")]
unsafe impl NSUserInterfaceValidations for NSStatusBarButton {}

extern_methods!(
    #[cfg(feature = "AppKit_NSStatusBarButton")]
    unsafe impl NSStatusBarButton {
        #[method(appearsDisabled)]
        pub unsafe fn appearsDisabled(&self) -> bool;

        #[method(setAppearsDisabled:)]
        pub unsafe fn setAppearsDisabled(&self, appears_disabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSButton`
    #[cfg(feature = "AppKit_NSStatusBarButton")]
    unsafe impl NSStatusBarButton {
        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other buttonWithTitle:image:target:action:)]
        pub unsafe fn buttonWithTitle_image_target_action(
            title: &NSString,
            image: &NSImage,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other buttonWithTitle:target:action:)]
        pub unsafe fn buttonWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other buttonWithImage:target:action:)]
        pub unsafe fn buttonWithImage_target_action(
            image: &NSImage,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other checkboxWithTitle:target:action:)]
        pub unsafe fn checkboxWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other radioButtonWithTitle:target:action:)]
        pub unsafe fn radioButtonWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSStatusBarButton")]
    unsafe impl NSStatusBarButton {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
