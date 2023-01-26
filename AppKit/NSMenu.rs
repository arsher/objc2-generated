//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSMenu")]
    pub struct NSMenu;

    #[cfg(feature = "AppKit_NSMenu")]
    unsafe impl ClassType for NSMenu {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSAccessibility for NSMenu {}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSAccessibilityElement for NSMenu {}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSAppearanceCustomization for NSMenu {}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSCoding for NSMenu {}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSObjectProtocol for NSMenu {}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSUserInterfaceItemIdentification for NSMenu {}

extern_methods!(
    #[cfg(feature = "AppKit_NSMenu")]
    unsafe impl NSMenu {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithTitle:)]
        pub unsafe fn initWithTitle(
            this: Option<Allocated<Self>>,
            title: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSView"))]
        #[method(popUpContextMenu:withEvent:forView:)]
        pub unsafe fn popUpContextMenu_withEvent_forView(
            menu: &NSMenu,
            event: &NSEvent,
            view: &NSView,
        );

        #[cfg(all(
            feature = "AppKit_NSEvent",
            feature = "AppKit_NSFont",
            feature = "AppKit_NSView"
        ))]
        #[method(popUpContextMenu:withEvent:forView:withFont:)]
        pub unsafe fn popUpContextMenu_withEvent_forView_withFont(
            menu: &NSMenu,
            event: &NSEvent,
            view: &NSView,
            font: Option<&NSFont>,
        );

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "AppKit_NSView"))]
        #[method(popUpMenuPositioningItem:atLocation:inView:)]
        pub unsafe fn popUpMenuPositioningItem_atLocation_inView(
            &self,
            item: Option<&NSMenuItem>,
            location: NSPoint,
            view: Option<&NSView>,
        ) -> bool;

        #[method(setMenuBarVisible:)]
        pub unsafe fn setMenuBarVisible(visible: bool);

        #[method(menuBarVisible)]
        pub unsafe fn menuBarVisible() -> bool;

        #[method_id(@__retain_semantics Other supermenu)]
        pub unsafe fn supermenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(setSupermenu:)]
        pub unsafe fn setSupermenu(&self, supermenu: Option<&NSMenu>);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(insertItem:atIndex:)]
        pub unsafe fn insertItem_atIndex(&self, new_item: &NSMenuItem, index: NSInteger);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(addItem:)]
        pub unsafe fn addItem(&self, new_item: &NSMenuItem);

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other insertItemWithTitle:action:keyEquivalent:atIndex:)]
        pub unsafe fn insertItemWithTitle_action_keyEquivalent_atIndex(
            &self,
            string: &NSString,
            selector: Option<Sel>,
            char_code: &NSString,
            index: NSInteger,
        ) -> Id<NSMenuItem, Shared>;

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other addItemWithTitle:action:keyEquivalent:)]
        pub unsafe fn addItemWithTitle_action_keyEquivalent(
            &self,
            string: &NSString,
            selector: Option<Sel>,
            char_code: &NSString,
        ) -> Id<NSMenuItem, Shared>;

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &NSMenuItem);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(setSubmenu:forItem:)]
        pub unsafe fn setSubmenu_forItem(&self, menu: Option<&NSMenu>, item: &NSMenuItem);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other itemArray)]
        pub unsafe fn itemArray(&self) -> Id<NSArray<NSMenuItem>, Shared>;

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSArray"))]
        #[method(setItemArray:)]
        pub unsafe fn setItemArray(&self, item_array: &NSArray<NSMenuItem>);

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(&self, index: NSInteger) -> Option<Id<NSMenuItem, Shared>>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(indexOfItem:)]
        pub unsafe fn indexOfItem(&self, item: &NSMenuItem) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(indexOfItemWithTitle:)]
        pub unsafe fn indexOfItemWithTitle(&self, title: &NSString) -> NSInteger;

        #[method(indexOfItemWithTag:)]
        pub unsafe fn indexOfItemWithTag(&self, tag: NSInteger) -> NSInteger;

        #[method(indexOfItemWithRepresentedObject:)]
        pub unsafe fn indexOfItemWithRepresentedObject(&self, object: Option<&Object>)
            -> NSInteger;

        #[method(indexOfItemWithSubmenu:)]
        pub unsafe fn indexOfItemWithSubmenu(&self, submenu: Option<&NSMenu>) -> NSInteger;

        #[method(indexOfItemWithTarget:andAction:)]
        pub unsafe fn indexOfItemWithTarget_andAction(
            &self,
            target: Option<&Object>,
            action_selector: Option<Sel>,
        ) -> NSInteger;

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other itemWithTitle:)]
        pub unsafe fn itemWithTitle(&self, title: &NSString) -> Option<Id<NSMenuItem, Shared>>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other itemWithTag:)]
        pub unsafe fn itemWithTag(&self, tag: NSInteger) -> Option<Id<NSMenuItem, Shared>>;

        #[method(autoenablesItems)]
        pub unsafe fn autoenablesItems(&self) -> bool;

        #[method(setAutoenablesItems:)]
        pub unsafe fn setAutoenablesItems(&self, autoenables_items: bool);

        #[method(update)]
        pub unsafe fn update(&self);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(itemChanged:)]
        pub unsafe fn itemChanged(&self, item: &NSMenuItem);

        #[method(performActionForItemAtIndex:)]
        pub unsafe fn performActionForItemAtIndex(&self, index: NSInteger);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSMenuDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSMenuDelegate>>);

        #[method(menuBarHeight)]
        pub unsafe fn menuBarHeight(&self) -> CGFloat;

        #[method(cancelTracking)]
        pub unsafe fn cancelTracking(&self);

        #[method(cancelTrackingWithoutAnimation)]
        pub unsafe fn cancelTrackingWithoutAnimation(&self);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other highlightedItem)]
        pub unsafe fn highlightedItem(&self) -> Option<Id<NSMenuItem, Shared>>;

        #[method(minimumWidth)]
        pub unsafe fn minimumWidth(&self) -> CGFloat;

        #[method(setMinimumWidth:)]
        pub unsafe fn setMinimumWidth(&self, minimum_width: CGFloat);

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[method(allowsContextMenuPlugIns)]
        pub unsafe fn allowsContextMenuPlugIns(&self) -> bool;

        #[method(setAllowsContextMenuPlugIns:)]
        pub unsafe fn setAllowsContextMenuPlugIns(&self, allows_context_menu_plug_ins: bool);

        #[method(showsStateColumn)]
        pub unsafe fn showsStateColumn(&self) -> bool;

        #[method(setShowsStateColumn:)]
        pub unsafe fn setShowsStateColumn(&self, shows_state_column: bool);

        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;

        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            user_interface_layout_direction: NSUserInterfaceLayoutDirection,
        );
    }
);

extern_methods!(
    /// NSSubmenuAction
    #[cfg(feature = "AppKit_NSMenu")]
    unsafe impl NSMenu {
        #[method(submenuAction:)]
        pub unsafe fn submenuAction(&self, sender: Option<&Object>);
    }
);

extern_protocol!(
    pub unsafe trait NSMenuItemValidation: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(validateMenuItem:)]
        unsafe fn validateMenuItem(&self, menu_item: &NSMenuItem) -> bool;
    }

    unsafe impl ProtocolType for dyn NSMenuItemValidation {}
);

extern_protocol!(
    pub unsafe trait NSMenuDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSMenu")]
        #[optional]
        #[method(menuNeedsUpdate:)]
        unsafe fn menuNeedsUpdate(&self, menu: &NSMenu);

        #[cfg(feature = "AppKit_NSMenu")]
        #[optional]
        #[method(numberOfItemsInMenu:)]
        unsafe fn numberOfItemsInMenu(&self, menu: &NSMenu) -> NSInteger;

        #[cfg(all(feature = "AppKit_NSMenu", feature = "AppKit_NSMenuItem"))]
        #[optional]
        #[method(menu:updateItem:atIndex:shouldCancel:)]
        unsafe fn menu_updateItem_atIndex_shouldCancel(
            &self,
            menu: &NSMenu,
            item: &NSMenuItem,
            index: NSInteger,
            should_cancel: bool,
        ) -> bool;

        #[cfg(feature = "AppKit_NSMenu")]
        #[optional]
        #[method(menuWillOpen:)]
        unsafe fn menuWillOpen(&self, menu: &NSMenu);

        #[cfg(feature = "AppKit_NSMenu")]
        #[optional]
        #[method(menuDidClose:)]
        unsafe fn menuDidClose(&self, menu: &NSMenu);

        #[cfg(all(feature = "AppKit_NSMenu", feature = "AppKit_NSMenuItem"))]
        #[optional]
        #[method(menu:willHighlightItem:)]
        unsafe fn menu_willHighlightItem(&self, menu: &NSMenu, item: Option<&NSMenuItem>);

        #[cfg(all(feature = "AppKit_NSMenu", feature = "AppKit_NSScreen"))]
        #[optional]
        #[method(confinementRectForMenu:onScreen:)]
        unsafe fn confinementRectForMenu_onScreen(
            &self,
            menu: &NSMenu,
            screen: Option<&NSScreen>,
        ) -> NSRect;
    }

    unsafe impl ProtocolType for dyn NSMenuDelegate {}
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSMenuProperties {
        NSMenuPropertyItemTitle = 1 << 0,
        NSMenuPropertyItemAttributedTitle = 1 << 1,
        NSMenuPropertyItemKeyEquivalent = 1 << 2,
        NSMenuPropertyItemImage = 1 << 3,
        NSMenuPropertyItemEnabled = 1 << 4,
        NSMenuPropertyItemAccessibilityDescription = 1 << 5,
    }
);

extern_methods!(
    /// NSMenuPropertiesToUpdate
    #[cfg(feature = "AppKit_NSMenu")]
    unsafe impl NSMenu {
        #[method(propertiesToUpdate)]
        pub unsafe fn propertiesToUpdate(&self) -> NSMenuProperties;
    }
);

extern_static!(NSMenuWillSendActionNotification: &'static NSNotificationName);

extern_static!(NSMenuDidSendActionNotification: &'static NSNotificationName);

extern_static!(NSMenuDidAddItemNotification: &'static NSNotificationName);

extern_static!(NSMenuDidRemoveItemNotification: &'static NSNotificationName);

extern_static!(NSMenuDidChangeItemNotification: &'static NSNotificationName);

extern_static!(NSMenuDidBeginTrackingNotification: &'static NSNotificationName);

extern_static!(NSMenuDidEndTrackingNotification: &'static NSNotificationName);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSMenu")]
    unsafe impl NSMenu {
        #[deprecated]
        #[method(setMenuRepresentation:)]
        pub unsafe fn setMenuRepresentation(&self, menu_rep: Option<&Object>);

        #[deprecated]
        #[method_id(@__retain_semantics Other menuRepresentation)]
        pub unsafe fn menuRepresentation(&self) -> Option<Id<Object, Shared>>;

        #[deprecated]
        #[method(setContextMenuRepresentation:)]
        pub unsafe fn setContextMenuRepresentation(&self, menu_rep: Option<&Object>);

        #[deprecated]
        #[method_id(@__retain_semantics Other contextMenuRepresentation)]
        pub unsafe fn contextMenuRepresentation(&self) -> Option<Id<Object, Shared>>;

        #[deprecated]
        #[method(setTearOffMenuRepresentation:)]
        pub unsafe fn setTearOffMenuRepresentation(&self, menu_rep: Option<&Object>);

        #[deprecated]
        #[method_id(@__retain_semantics Other tearOffMenuRepresentation)]
        pub unsafe fn tearOffMenuRepresentation(&self) -> Option<Id<Object, Shared>>;

        #[deprecated]
        #[method(menuZone)]
        pub unsafe fn menuZone() -> *mut NSZone;

        #[deprecated]
        #[method(setMenuZone:)]
        pub unsafe fn setMenuZone(zone: *mut NSZone);

        #[deprecated]
        #[method_id(@__retain_semantics Other attachedMenu)]
        pub unsafe fn attachedMenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[deprecated]
        #[method(isAttached)]
        pub unsafe fn isAttached(&self) -> bool;

        #[deprecated]
        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[deprecated]
        #[method(locationForSubmenu:)]
        pub unsafe fn locationForSubmenu(&self, submenu: Option<&NSMenu>) -> NSPoint;

        #[deprecated]
        #[method(menuChangedMessagesEnabled)]
        pub unsafe fn menuChangedMessagesEnabled(&self) -> bool;

        #[deprecated]
        #[method(setMenuChangedMessagesEnabled:)]
        pub unsafe fn setMenuChangedMessagesEnabled(&self, menu_changed_messages_enabled: bool);

        #[cfg(feature = "AppKit_NSEvent")]
        #[deprecated]
        #[method(helpRequested:)]
        pub unsafe fn helpRequested(&self, event_ptr: &NSEvent);

        #[deprecated]
        #[method(isTornOff)]
        pub unsafe fn isTornOff(&self) -> bool;
    }
);
