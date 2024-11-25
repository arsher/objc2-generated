//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
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
        #[method_id(@__retain_semantics Other fixedGroupWithRepresentativeItem:items:)]
        pub unsafe fn fixedGroupWithRepresentativeItem_items(
            representative_item: Option<&UIBarButtonItem>,
            items: &NSArray<UIBarButtonItem>,
            mtm: MainThreadMarker,
        ) -> Retained<UIBarButtonItemGroup>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method_id(@__retain_semantics Other movableGroupWithCustomizationIdentifier:representativeItem:items:)]
        pub unsafe fn movableGroupWithCustomizationIdentifier_representativeItem_items(
            customization_identifier: &NSString,
            representative_item: Option<&UIBarButtonItem>,
            items: &NSArray<UIBarButtonItem>,
            mtm: MainThreadMarker,
        ) -> Retained<UIBarButtonItemGroup>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method_id(@__retain_semantics Other optionalGroupWithCustomizationIdentifier:inDefaultCustomization:representativeItem:items:)]
        pub unsafe fn optionalGroupWithCustomizationIdentifier_inDefaultCustomization_representativeItem_items(
            customization_identifier: &NSString,
            in_default_customization: bool,
            representative_item: Option<&UIBarButtonItem>,
            items: &NSArray<UIBarButtonItem>,
            mtm: MainThreadMarker,
        ) -> Retained<UIBarButtonItemGroup>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method_id(@__retain_semantics Other barButtonItems)]
        pub unsafe fn barButtonItems(&self) -> Retained<NSArray<UIBarButtonItem>>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method(setBarButtonItems:)]
        pub unsafe fn setBarButtonItems(&self, bar_button_items: &NSArray<UIBarButtonItem>);

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method_id(@__retain_semantics Other representativeItem)]
        pub unsafe fn representativeItem(&self) -> Option<Retained<UIBarButtonItem>>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method(setRepresentativeItem:)]
        pub unsafe fn setRepresentativeItem(&self, representative_item: Option<&UIBarButtonItem>);

        #[method(isDisplayingRepresentativeItem)]
        pub unsafe fn isDisplayingRepresentativeItem(&self) -> bool;

        #[method(alwaysAvailable)]
        pub unsafe fn alwaysAvailable(&self) -> bool;

        #[method(setAlwaysAvailable:)]
        pub unsafe fn setAlwaysAvailable(&self, always_available: bool);

        #[cfg(feature = "UIMenuElement")]
        #[method_id(@__retain_semantics Other menuRepresentation)]
        pub unsafe fn menuRepresentation(&self) -> Option<Retained<UIMenuElement>>;

        #[cfg(feature = "UIMenuElement")]
        #[method(setMenuRepresentation:)]
        pub unsafe fn setMenuRepresentation(&self, menu_representation: Option<&UIMenuElement>);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

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
        #[method_id(@__retain_semantics Other buttonGroup)]
        pub unsafe fn buttonGroup(&self) -> Option<Retained<UIBarButtonItemGroup>>;
    }
);
