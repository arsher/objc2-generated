//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTabViewControllerTabStyle {
        NSTabViewControllerTabStyleSegmentedControlOnTop = 0,
        NSTabViewControllerTabStyleSegmentedControlOnBottom = 1,
        NSTabViewControllerTabStyleToolbar = 2,
        NSTabViewControllerTabStyleUnspecified = -1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTabViewController")]
    pub struct NSTabViewController;

    #[cfg(feature = "AppKit_NSTabViewController")]
    unsafe impl ClassType for NSTabViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

#[cfg(feature = "AppKit_NSTabViewController")]
unsafe impl NSCoding for NSTabViewController {}

#[cfg(feature = "AppKit_NSTabViewController")]
unsafe impl NSEditor for NSTabViewController {}

#[cfg(feature = "AppKit_NSTabViewController")]
unsafe impl NSObjectProtocol for NSTabViewController {}

#[cfg(feature = "AppKit_NSTabViewController")]
unsafe impl NSSeguePerforming for NSTabViewController {}

#[cfg(feature = "AppKit_NSTabViewController")]
unsafe impl NSTabViewDelegate for NSTabViewController {}

#[cfg(feature = "AppKit_NSTabViewController")]
unsafe impl NSToolbarDelegate for NSTabViewController {}

#[cfg(feature = "AppKit_NSTabViewController")]
unsafe impl NSUserInterfaceItemIdentification for NSTabViewController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTabViewController")]
    unsafe impl NSTabViewController {
        #[method(tabStyle)]
        pub unsafe fn tabStyle(&self) -> NSTabViewControllerTabStyle;

        #[method(setTabStyle:)]
        pub unsafe fn setTabStyle(&self, tab_style: NSTabViewControllerTabStyle);

        #[cfg(feature = "AppKit_NSTabView")]
        #[method_id(@__retain_semantics Other tabView)]
        pub unsafe fn tabView(&self) -> Id<NSTabView, Shared>;

        #[cfg(feature = "AppKit_NSTabView")]
        #[method(setTabView:)]
        pub unsafe fn setTabView(&self, tab_view: &NSTabView);

        #[method(transitionOptions)]
        pub unsafe fn transitionOptions(&self) -> NSViewControllerTransitionOptions;

        #[method(setTransitionOptions:)]
        pub unsafe fn setTransitionOptions(
            &self,
            transition_options: NSViewControllerTransitionOptions,
        );

        #[method(canPropagateSelectedChildViewControllerTitle)]
        pub unsafe fn canPropagateSelectedChildViewControllerTitle(&self) -> bool;

        #[method(setCanPropagateSelectedChildViewControllerTitle:)]
        pub unsafe fn setCanPropagateSelectedChildViewControllerTitle(
            &self,
            can_propagate_selected_child_view_controller_title: bool,
        );

        #[cfg(all(feature = "AppKit_NSTabViewItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other tabViewItems)]
        pub unsafe fn tabViewItems(&self) -> Id<NSArray<NSTabViewItem>, Shared>;

        #[cfg(all(feature = "AppKit_NSTabViewItem", feature = "Foundation_NSArray"))]
        #[method(setTabViewItems:)]
        pub unsafe fn setTabViewItems(&self, tab_view_items: &NSArray<NSTabViewItem>);

        #[method(selectedTabViewItemIndex)]
        pub unsafe fn selectedTabViewItemIndex(&self) -> NSInteger;

        #[method(setSelectedTabViewItemIndex:)]
        pub unsafe fn setSelectedTabViewItemIndex(&self, selected_tab_view_item_index: NSInteger);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(addTabViewItem:)]
        pub unsafe fn addTabViewItem(&self, tab_view_item: &NSTabViewItem);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(insertTabViewItem:atIndex:)]
        pub unsafe fn insertTabViewItem_atIndex(
            &self,
            tab_view_item: &NSTabViewItem,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(removeTabViewItem:)]
        pub unsafe fn removeTabViewItem(&self, tab_view_item: &NSTabViewItem);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method_id(@__retain_semantics Other tabViewItemForViewController:)]
        pub unsafe fn tabViewItemForViewController(
            &self,
            view_controller: &NSViewController,
        ) -> Option<Id<NSTabViewItem, Shared>>;

        #[method(viewDidLoad)]
        pub unsafe fn viewDidLoad(&self);

        #[cfg(all(feature = "AppKit_NSTabView", feature = "AppKit_NSTabViewItem"))]
        #[method(tabView:willSelectTabViewItem:)]
        pub unsafe fn tabView_willSelectTabViewItem(
            &self,
            tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        );

        #[cfg(all(feature = "AppKit_NSTabView", feature = "AppKit_NSTabViewItem"))]
        #[method(tabView:didSelectTabViewItem:)]
        pub unsafe fn tabView_didSelectTabViewItem(
            &self,
            tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        );

        #[cfg(all(feature = "AppKit_NSTabView", feature = "AppKit_NSTabViewItem"))]
        #[method(tabView:shouldSelectTabViewItem:)]
        pub unsafe fn tabView_shouldSelectTabViewItem(
            &self,
            tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSToolbar", feature = "AppKit_NSToolbarItem"))]
        #[method_id(@__retain_semantics Other toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:)]
        pub unsafe fn toolbar_itemForItemIdentifier_willBeInsertedIntoToolbar(
            &self,
            toolbar: &NSToolbar,
            item_identifier: &NSToolbarItemIdentifier,
            flag: bool,
        ) -> Option<Id<NSToolbarItem, Shared>>;

        #[cfg(all(feature = "AppKit_NSToolbar", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other toolbarDefaultItemIdentifiers:)]
        pub unsafe fn toolbarDefaultItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Id<NSArray<NSToolbarItemIdentifier>, Shared>;

        #[cfg(all(feature = "AppKit_NSToolbar", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other toolbarAllowedItemIdentifiers:)]
        pub unsafe fn toolbarAllowedItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Id<NSArray<NSToolbarItemIdentifier>, Shared>;

        #[cfg(all(feature = "AppKit_NSToolbar", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other toolbarSelectableItemIdentifiers:)]
        pub unsafe fn toolbarSelectableItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Id<NSArray<NSToolbarItemIdentifier>, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "AppKit_NSTabViewController")]
    unsafe impl NSTabViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self, Shared>;
    }
);
