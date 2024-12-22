//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uibarbuttonitemgroup?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIBarButtonItemGroup;
);

unsafe impl NSCoding for UIBarButtonItemGroup {}

unsafe impl NSObjectProtocol for UIBarButtonItemGroup {}

extern_methods!(
    unsafe impl UIBarButtonItemGroup {
        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        /// Create a new bar button item group with the given items. When bar button item layout is done, either the group's barButtonItems or its representativeItem is displayed (if it exists).
        #[method_id(@__retain_semantics Init initWithBarButtonItems:representativeItem:)]
        pub unsafe fn initWithBarButtonItems_representativeItem(
            this: Allocated<Self>,
            bar_button_items: &NSArray<UIBarButtonItem>,
            representative_item: Option<&UIBarButtonItem>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        /// Construct a UIBarButtonItemGroup that cannot be moved or removed under UINavigationBar customization.
        #[method_id(@__retain_semantics Other fixedGroupWithRepresentativeItem:items:)]
        pub unsafe fn fixedGroupWithRepresentativeItem_items(
            representative_item: Option<&UIBarButtonItem>,
            items: &NSArray<UIBarButtonItem>,
            mtm: MainThreadMarker,
        ) -> Retained<UIBarButtonItemGroup>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        /// Construct a UIBarButtonItemGroup that can be moved but cannot be removed under UINavigationBar customization.
        #[method_id(@__retain_semantics Other movableGroupWithCustomizationIdentifier:representativeItem:items:)]
        pub unsafe fn movableGroupWithCustomizationIdentifier_representativeItem_items(
            customization_identifier: &NSString,
            representative_item: Option<&UIBarButtonItem>,
            items: &NSArray<UIBarButtonItem>,
            mtm: MainThreadMarker,
        ) -> Retained<UIBarButtonItemGroup>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        /// Construct a UIBarButtonItemGroup that can be moved or added/removed under UINavigationBar customization.
        #[method_id(@__retain_semantics Other optionalGroupWithCustomizationIdentifier:inDefaultCustomization:representativeItem:items:)]
        pub unsafe fn optionalGroupWithCustomizationIdentifier_inDefaultCustomization_representativeItem_items(
            customization_identifier: &NSString,
            in_default_customization: bool,
            representative_item: Option<&UIBarButtonItem>,
            items: &NSArray<UIBarButtonItem>,
            mtm: MainThreadMarker,
        ) -> Retained<UIBarButtonItemGroup>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        /// The bar button items associated with this group. Changing these items will affect the bar displaying these items without needing to re-set the groups that are in that bar. Any UIBarButtonItems that are already in group will be removed from that group.
        #[method_id(@__retain_semantics Other barButtonItems)]
        pub unsafe fn barButtonItems(&self) -> Retained<NSArray<UIBarButtonItem>>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        /// Setter for [`barButtonItems`][Self::barButtonItems].
        #[method(setBarButtonItems:)]
        pub unsafe fn setBarButtonItems(&self, bar_button_items: &NSArray<UIBarButtonItem>);

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        /// In order to display as many items as possible, bars that support UIBarButtonItemGroup may choose to collapse items associated with groups to the representativeItem specified by the group.
        /// A bar will only collapse groups that have a representativeItem set, but may still choose to use an alternate presentation of these items.
        /// A UIBarButtonItem may only be either the representativeItem or a member of the barButtonItems of a single UIBarButtonItemGroup and may only represent a single group.
        /// If the representativeItem has an action, then that action will be invoked, otherwise the bar will present a standard UI to allow selection of the barButtonItems in the representedItem's group.
        #[method_id(@__retain_semantics Other representativeItem)]
        pub unsafe fn representativeItem(&self) -> Option<Retained<UIBarButtonItem>>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        /// Setter for [`representativeItem`][Self::representativeItem].
        #[method(setRepresentativeItem:)]
        pub unsafe fn setRepresentativeItem(&self, representative_item: Option<&UIBarButtonItem>);

        /// Returns YES if the representativeItem of this group is currently being displayed, rather than its barButtonItems.
        #[method(isDisplayingRepresentativeItem)]
        pub unsafe fn isDisplayingRepresentativeItem(&self) -> bool;

        /// Instructs UIKit to ensure that the functionality in this group is made available to the user regardless of customization status. On iPhone and iPad idioms, UIKit currently places these items in the overflow menu; this property has no effect on macOS idiom.
        #[method(alwaysAvailable)]
        pub unsafe fn alwaysAvailable(&self) -> bool;

        /// Setter for [`alwaysAvailable`][Self::alwaysAvailable].
        #[method(setAlwaysAvailable:)]
        pub unsafe fn setAlwaysAvailable(&self, always_available: bool);

        #[cfg(feature = "UIMenuElement")]
        /// A UIMenuElement that should substitute for the UIBarButtonItemGroup when displayed in a menu.
        #[method_id(@__retain_semantics Other menuRepresentation)]
        pub unsafe fn menuRepresentation(&self) -> Option<Retained<UIMenuElement>>;

        #[cfg(feature = "UIMenuElement")]
        /// Setter for [`menuRepresentation`][Self::menuRepresentation].
        #[method(setMenuRepresentation:)]
        pub unsafe fn setMenuRepresentation(&self, menu_representation: Option<&UIMenuElement>);

        /// If the group should be hidden from display.
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        /// Setter for [`isHidden`][Self::isHidden].
        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIBarButtonItemGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// UIBarButtonItemGroup
    #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
    unsafe impl UIBarButtonItem {
        /// The group that the UIBarButtonItem is currently associated with, either as a member of the barButtonItems array or as that group's representativeItem.
        #[method_id(@__retain_semantics Other buttonGroup)]
        pub unsafe fn buttonGroup(&self) -> Option<Retained<UIBarButtonItemGroup>>;
    }
);
