//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMenu;

    unsafe impl ClassType for NSMenu {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSMenu")]
    unsafe impl NSMenu {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithTitle:)]
        pub unsafe fn initWithTitle(
            this: Option<Allocated<Self>>,
            title: &Foundation::NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &Foundation::NSString);

        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSView"))]
        #[method(popUpContextMenu:withEvent:forView:)]
        pub unsafe fn popUpContextMenu_withEvent_forView(
            menu: &AppKit::NSMenu,
            event: &AppKit::NSEvent,
            view: &AppKit::NSView,
        );

        #[cfg(all(
            feature = "AppKit_NSEvent",
            feature = "AppKit_NSFont",
            feature = "AppKit_NSView"
        ))]
        #[method(popUpContextMenu:withEvent:forView:withFont:)]
        pub unsafe fn popUpContextMenu_withEvent_forView_withFont(
            menu: &AppKit::NSMenu,
            event: &AppKit::NSEvent,
            view: &AppKit::NSView,
            font: Option<&AppKit::NSFont>,
        );

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "AppKit_NSView"))]
        #[method(popUpMenuPositioningItem:atLocation:inView:)]
        pub unsafe fn popUpMenuPositioningItem_atLocation_inView(
            &self,
            item: Option<&AppKit::NSMenuItem>,
            location: NSPoint,
            view: Option<&AppKit::NSView>,
        ) -> bool;

        #[method(setMenuBarVisible:)]
        pub unsafe fn setMenuBarVisible(visible: bool);

        #[method(menuBarVisible)]
        pub unsafe fn menuBarVisible() -> bool;

        #[method_id(@__retain_semantics Other supermenu)]
        pub unsafe fn supermenu(&self) -> Option<Id<AppKit::NSMenu, Shared>>;

        #[method(setSupermenu:)]
        pub unsafe fn setSupermenu(&self, supermenu: Option<&AppKit::NSMenu>);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(insertItem:atIndex:)]
        pub unsafe fn insertItem_atIndex(&self, newItem: &AppKit::NSMenuItem, index: NSInteger);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(addItem:)]
        pub unsafe fn addItem(&self, newItem: &AppKit::NSMenuItem);

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other insertItemWithTitle:action:keyEquivalent:atIndex:)]
        pub unsafe fn insertItemWithTitle_action_keyEquivalent_atIndex(
            &self,
            string: &Foundation::NSString,
            selector: Option<Sel>,
            charCode: &Foundation::NSString,
            index: NSInteger,
        ) -> Id<AppKit::NSMenuItem, Shared>;

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other addItemWithTitle:action:keyEquivalent:)]
        pub unsafe fn addItemWithTitle_action_keyEquivalent(
            &self,
            string: &Foundation::NSString,
            selector: Option<Sel>,
            charCode: &Foundation::NSString,
        ) -> Id<AppKit::NSMenuItem, Shared>;

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &AppKit::NSMenuItem);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(setSubmenu:forItem:)]
        pub unsafe fn setSubmenu_forItem(
            &self,
            menu: Option<&AppKit::NSMenu>,
            item: &AppKit::NSMenuItem,
        );

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other itemArray)]
        pub unsafe fn itemArray(&self) -> Id<Foundation::NSArray<AppKit::NSMenuItem>, Shared>;

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSArray"))]
        #[method(setItemArray:)]
        pub unsafe fn setItemArray(&self, itemArray: &Foundation::NSArray<AppKit::NSMenuItem>);

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Id<AppKit::NSMenuItem, Shared>>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(indexOfItem:)]
        pub unsafe fn indexOfItem(&self, item: &AppKit::NSMenuItem) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(indexOfItemWithTitle:)]
        pub unsafe fn indexOfItemWithTitle(&self, title: &Foundation::NSString) -> NSInteger;

        #[method(indexOfItemWithTag:)]
        pub unsafe fn indexOfItemWithTag(&self, tag: NSInteger) -> NSInteger;

        #[method(indexOfItemWithRepresentedObject:)]
        pub unsafe fn indexOfItemWithRepresentedObject(&self, object: Option<&Object>)
            -> NSInteger;

        #[method(indexOfItemWithSubmenu:)]
        pub unsafe fn indexOfItemWithSubmenu(&self, submenu: Option<&AppKit::NSMenu>) -> NSInteger;

        #[method(indexOfItemWithTarget:andAction:)]
        pub unsafe fn indexOfItemWithTarget_andAction(
            &self,
            target: Option<&Object>,
            actionSelector: Option<Sel>,
        ) -> NSInteger;

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other itemWithTitle:)]
        pub unsafe fn itemWithTitle(
            &self,
            title: &Foundation::NSString,
        ) -> Option<Id<AppKit::NSMenuItem, Shared>>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other itemWithTag:)]
        pub unsafe fn itemWithTag(&self, tag: NSInteger) -> Option<Id<AppKit::NSMenuItem, Shared>>;

        #[method(autoenablesItems)]
        pub unsafe fn autoenablesItems(&self) -> bool;

        #[method(setAutoenablesItems:)]
        pub unsafe fn setAutoenablesItems(&self, autoenablesItems: bool);

        #[method(update)]
        pub unsafe fn update(&self);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, event: &AppKit::NSEvent) -> bool;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(itemChanged:)]
        pub unsafe fn itemChanged(&self, item: &AppKit::NSMenuItem);

        #[method(performActionForItemAtIndex:)]
        pub unsafe fn performActionForItemAtIndex(&self, index: NSInteger);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AppKit::NSMenuDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AppKit::NSMenuDelegate>);

        #[method(menuBarHeight)]
        pub unsafe fn menuBarHeight(&self) -> CGFloat;

        #[method(cancelTracking)]
        pub unsafe fn cancelTracking(&self);

        #[method(cancelTrackingWithoutAnimation)]
        pub unsafe fn cancelTrackingWithoutAnimation(&self);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other highlightedItem)]
        pub unsafe fn highlightedItem(&self) -> Option<Id<AppKit::NSMenuItem, Shared>>;

        #[method(minimumWidth)]
        pub unsafe fn minimumWidth(&self) -> CGFloat;

        #[method(setMinimumWidth:)]
        pub unsafe fn setMinimumWidth(&self, minimumWidth: CGFloat);

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<AppKit::NSFont, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&AppKit::NSFont>);

        #[method(allowsContextMenuPlugIns)]
        pub unsafe fn allowsContextMenuPlugIns(&self) -> bool;

        #[method(setAllowsContextMenuPlugIns:)]
        pub unsafe fn setAllowsContextMenuPlugIns(&self, allowsContextMenuPlugIns: bool);

        #[method(showsStateColumn)]
        pub unsafe fn showsStateColumn(&self) -> bool;

        #[method(setShowsStateColumn:)]
        pub unsafe fn setShowsStateColumn(&self, showsStateColumn: bool);

        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;

        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            userInterfaceLayoutDirection: NSUserInterfaceLayoutDirection,
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
    pub struct NSMenuItemValidation;

    unsafe impl ProtocolType for NSMenuItemValidation {
        #[method(validateMenuItem:)]
        pub unsafe fn validateMenuItem(&self, menuItem: &AppKit::NSMenuItem) -> bool;
    }
);

extern_protocol!(
    pub struct NSMenuDelegate;

    unsafe impl ProtocolType for NSMenuDelegate {
        #[optional]
        #[method(menuNeedsUpdate:)]
        pub unsafe fn menuNeedsUpdate(&self, menu: &AppKit::NSMenu);

        #[optional]
        #[method(numberOfItemsInMenu:)]
        pub unsafe fn numberOfItemsInMenu(&self, menu: &AppKit::NSMenu) -> NSInteger;

        #[optional]
        #[method(menu:updateItem:atIndex:shouldCancel:)]
        pub unsafe fn menu_updateItem_atIndex_shouldCancel(
            &self,
            menu: &AppKit::NSMenu,
            item: &AppKit::NSMenuItem,
            index: NSInteger,
            shouldCancel: bool,
        ) -> bool;

        #[optional]
        #[method(menuWillOpen:)]
        pub unsafe fn menuWillOpen(&self, menu: &AppKit::NSMenu);

        #[optional]
        #[method(menuDidClose:)]
        pub unsafe fn menuDidClose(&self, menu: &AppKit::NSMenu);

        #[optional]
        #[method(menu:willHighlightItem:)]
        pub unsafe fn menu_willHighlightItem(
            &self,
            menu: &AppKit::NSMenu,
            item: Option<&AppKit::NSMenuItem>,
        );

        #[optional]
        #[method(confinementRectForMenu:onScreen:)]
        pub unsafe fn confinementRectForMenu_onScreen(
            &self,
            menu: &AppKit::NSMenu,
            screen: Option<&AppKit::NSScreen>,
        ) -> NSRect;
    }
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

extern_static!(NSMenuWillSendActionNotification: &'static Foundation::NSNotificationName);

extern_static!(NSMenuDidSendActionNotification: &'static Foundation::NSNotificationName);

extern_static!(NSMenuDidAddItemNotification: &'static Foundation::NSNotificationName);

extern_static!(NSMenuDidRemoveItemNotification: &'static Foundation::NSNotificationName);

extern_static!(NSMenuDidChangeItemNotification: &'static Foundation::NSNotificationName);

extern_static!(NSMenuDidBeginTrackingNotification: &'static Foundation::NSNotificationName);

extern_static!(NSMenuDidEndTrackingNotification: &'static Foundation::NSNotificationName);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSMenu")]
    unsafe impl NSMenu {
        #[method(setMenuRepresentation:)]
        pub unsafe fn setMenuRepresentation(&self, menuRep: Option<&Object>);

        #[method_id(@__retain_semantics Other menuRepresentation)]
        pub unsafe fn menuRepresentation(&self) -> Option<Id<Object, Shared>>;

        #[method(setContextMenuRepresentation:)]
        pub unsafe fn setContextMenuRepresentation(&self, menuRep: Option<&Object>);

        #[method_id(@__retain_semantics Other contextMenuRepresentation)]
        pub unsafe fn contextMenuRepresentation(&self) -> Option<Id<Object, Shared>>;

        #[method(setTearOffMenuRepresentation:)]
        pub unsafe fn setTearOffMenuRepresentation(&self, menuRep: Option<&Object>);

        #[method_id(@__retain_semantics Other tearOffMenuRepresentation)]
        pub unsafe fn tearOffMenuRepresentation(&self) -> Option<Id<Object, Shared>>;

        #[method(menuZone)]
        pub unsafe fn menuZone() -> *mut NSZone;

        #[method(setMenuZone:)]
        pub unsafe fn setMenuZone(zone: *mut NSZone);

        #[method_id(@__retain_semantics Other attachedMenu)]
        pub unsafe fn attachedMenu(&self) -> Option<Id<AppKit::NSMenu, Shared>>;

        #[method(isAttached)]
        pub unsafe fn isAttached(&self) -> bool;

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(locationForSubmenu:)]
        pub unsafe fn locationForSubmenu(&self, submenu: Option<&AppKit::NSMenu>) -> NSPoint;

        #[method(menuChangedMessagesEnabled)]
        pub unsafe fn menuChangedMessagesEnabled(&self) -> bool;

        #[method(setMenuChangedMessagesEnabled:)]
        pub unsafe fn setMenuChangedMessagesEnabled(&self, menuChangedMessagesEnabled: bool);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(helpRequested:)]
        pub unsafe fn helpRequested(&self, eventPtr: &AppKit::NSEvent);

        #[method(isTornOff)]
        pub unsafe fn isTornOff(&self) -> bool;
    }
);
