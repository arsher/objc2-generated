//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspopupbutton?language=objc)
    #[unsafe(super(NSButton, NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    pub struct NSPopUpButton;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSPopUpButton {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityButton for NSPopUpButton {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSPopUpButton {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSPopUpButton {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSPopUpButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSCoding for NSPopUpButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSPopUpButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSObjectProtocol for NSPopUpButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceCompression",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceCompression for NSPopUpButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSPopUpButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceValidation",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceValidations for NSPopUpButton {}

extern_methods!(
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    unsafe impl NSPopUpButton {
        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other popUpButtonWithMenu:target:action:)]
        pub unsafe fn popUpButtonWithMenu_target_action(
            menu: &NSMenu,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other pullDownButtonWithTitle:menu:)]
        pub unsafe fn pullDownButtonWithTitle_menu(
            title: &NSString,
            menu: &NSMenu,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSImage", feature = "NSMenu"))]
        #[method_id(@__retain_semantics Other pullDownButtonWithImage:menu:)]
        pub unsafe fn pullDownButtonWithImage_menu(
            image: &NSImage,
            menu: &NSMenu,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSImage", feature = "NSMenu"))]
        #[method_id(@__retain_semantics Other pullDownButtonWithTitle:image:menu:)]
        pub unsafe fn pullDownButtonWithTitle_image_menu(
            title: &NSString,
            image: &NSImage,
            menu: &NSMenu,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:pullsDown:)]
        pub unsafe fn initWithFrame_pullsDown(
            this: Allocated<Self>,
            button_frame: NSRect,
            flag: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method(pullsDown)]
        pub unsafe fn pullsDown(&self) -> bool;

        #[method(setPullsDown:)]
        pub unsafe fn setPullsDown(&self, pulls_down: bool);

        #[method(autoenablesItems)]
        pub unsafe fn autoenablesItems(&self) -> bool;

        #[method(setAutoenablesItems:)]
        pub unsafe fn setAutoenablesItems(&self, autoenables_items: bool);

        #[method(preferredEdge)]
        pub unsafe fn preferredEdge(&self) -> NSRectEdge;

        #[method(setPreferredEdge:)]
        pub unsafe fn setPreferredEdge(&self, preferred_edge: NSRectEdge);

        #[method(usesItemFromMenu)]
        pub unsafe fn usesItemFromMenu(&self) -> bool;

        #[method(setUsesItemFromMenu:)]
        pub unsafe fn setUsesItemFromMenu(&self, uses_item_from_menu: bool);

        #[method(altersStateOfSelectedItem)]
        pub unsafe fn altersStateOfSelectedItem(&self) -> bool;

        #[method(setAltersStateOfSelectedItem:)]
        pub unsafe fn setAltersStateOfSelectedItem(&self, alters_state_of_selected_item: bool);

        #[method(addItemWithTitle:)]
        pub unsafe fn addItemWithTitle(&self, title: &NSString);

        #[method(addItemsWithTitles:)]
        pub unsafe fn addItemsWithTitles(&self, item_titles: &NSArray<NSString>);

        #[method(insertItemWithTitle:atIndex:)]
        pub unsafe fn insertItemWithTitle_atIndex(&self, title: &NSString, index: NSInteger);

        #[method(removeItemWithTitle:)]
        pub unsafe fn removeItemWithTitle(&self, title: &NSString);

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[cfg(feature = "NSMenuItem")]
        #[method_id(@__retain_semantics Other itemArray)]
        pub unsafe fn itemArray(&self) -> Retained<NSArray<NSMenuItem>>;

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[cfg(feature = "NSMenuItem")]
        #[method(indexOfItem:)]
        pub unsafe fn indexOfItem(&self, item: &NSMenuItem) -> NSInteger;

        #[method(indexOfItemWithTitle:)]
        pub unsafe fn indexOfItemWithTitle(&self, title: &NSString) -> NSInteger;

        #[method(indexOfItemWithTag:)]
        pub unsafe fn indexOfItemWithTag(&self, tag: NSInteger) -> NSInteger;

        #[method(indexOfItemWithRepresentedObject:)]
        pub unsafe fn indexOfItemWithRepresentedObject(&self, obj: Option<&AnyObject>)
            -> NSInteger;

        #[method(indexOfItemWithTarget:andAction:)]
        pub unsafe fn indexOfItemWithTarget_andAction(
            &self,
            target: Option<&AnyObject>,
            action_selector: Option<Sel>,
        ) -> NSInteger;

        #[cfg(feature = "NSMenuItem")]
        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(&self, index: NSInteger) -> Option<Retained<NSMenuItem>>;

        #[cfg(feature = "NSMenuItem")]
        #[method_id(@__retain_semantics Other itemWithTitle:)]
        pub unsafe fn itemWithTitle(&self, title: &NSString) -> Option<Retained<NSMenuItem>>;

        #[cfg(feature = "NSMenuItem")]
        #[method_id(@__retain_semantics Other lastItem)]
        pub unsafe fn lastItem(&self) -> Option<Retained<NSMenuItem>>;

        #[cfg(feature = "NSMenuItem")]
        #[method(selectItem:)]
        pub unsafe fn selectItem(&self, item: Option<&NSMenuItem>);

        #[method(selectItemAtIndex:)]
        pub unsafe fn selectItemAtIndex(&self, index: NSInteger);

        #[method(selectItemWithTitle:)]
        pub unsafe fn selectItemWithTitle(&self, title: &NSString);

        #[method(selectItemWithTag:)]
        pub unsafe fn selectItemWithTag(&self, tag: NSInteger) -> bool;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, string: &NSString);

        #[cfg(feature = "NSMenuItem")]
        #[method_id(@__retain_semantics Other selectedItem)]
        pub unsafe fn selectedItem(&self) -> Option<Retained<NSMenuItem>>;

        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[method(selectedTag)]
        pub unsafe fn selectedTag(&self) -> NSInteger;

        #[method(synchronizeTitleAndSelectedItem)]
        pub unsafe fn synchronizeTitleAndSelectedItem(&self);

        #[method_id(@__retain_semantics Other itemTitleAtIndex:)]
        pub unsafe fn itemTitleAtIndex(&self, index: NSInteger) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other itemTitles)]
        pub unsafe fn itemTitles(&self) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other titleOfSelectedItem)]
        pub unsafe fn titleOfSelectedItem(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSButton`
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    unsafe impl NSPopUpButton {
        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other buttonWithTitle:image:target:action:)]
        pub unsafe fn buttonWithTitle_image_target_action(
            title: &NSString,
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other buttonWithTitle:target:action:)]
        pub unsafe fn buttonWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other buttonWithImage:target:action:)]
        pub unsafe fn buttonWithImage_target_action(
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other checkboxWithTitle:target:action:)]
        pub unsafe fn checkboxWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other radioButtonWithTitle:target:action:)]
        pub unsafe fn radioButtonWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    unsafe impl NSPopUpButton {
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
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    unsafe impl NSPopUpButton {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    unsafe impl NSPopUpButton {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspopupbuttonwillpopupnotification?language=objc)
    pub static NSPopUpButtonWillPopUpNotification: &'static NSNotificationName;
}
