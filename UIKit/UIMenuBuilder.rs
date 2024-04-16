//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UIMenuBuilder: IsMainThreadOnly {
        #[cfg(feature = "UIMenuSystem")]
        #[method_id(@__retain_semantics Other system)]
        unsafe fn system(&self) -> Id<UIMenuSystem>;

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other menuForIdentifier:)]
        unsafe fn menuForIdentifier(&self, identifier: &UIMenuIdentifier) -> Option<Id<UIMenu>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other actionForIdentifier:)]
        unsafe fn actionForIdentifier(
            &self,
            identifier: &UIActionIdentifier,
        ) -> Option<Id<UIAction>>;

        #[cfg(all(feature = "UICommand", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other commandForAction:propertyList:)]
        unsafe fn commandForAction_propertyList(
            &self,
            action: Sel,
            property_list: Option<&AnyObject>,
        ) -> Option<Id<UICommand>>;

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method(replaceMenuForIdentifier:withMenu:)]
        unsafe fn replaceMenuForIdentifier_withMenu(
            &self,
            replaced_identifier: &UIMenuIdentifier,
            replacement_menu: &UIMenu,
        );

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement", feature = "block2"))]
        #[method(replaceChildrenOfMenuForIdentifier:fromChildrenBlock:)]
        unsafe fn replaceChildrenOfMenuForIdentifier_fromChildrenBlock(
            &self,
            parent_identifier: &UIMenuIdentifier,
            children_block: &Block<
                dyn Fn(NonNull<NSArray<UIMenuElement>>) -> NonNull<NSArray<UIMenuElement>> + '_,
            >,
        );

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method(insertSiblingMenu:beforeMenuForIdentifier:)]
        unsafe fn insertSiblingMenu_beforeMenuForIdentifier(
            &self,
            sibling_menu: &UIMenu,
            sibling_identifier: &UIMenuIdentifier,
        );

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method(insertSiblingMenu:afterMenuForIdentifier:)]
        unsafe fn insertSiblingMenu_afterMenuForIdentifier(
            &self,
            sibling_menu: &UIMenu,
            sibling_identifier: &UIMenuIdentifier,
        );

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method(insertChildMenu:atStartOfMenuForIdentifier:)]
        unsafe fn insertChildMenu_atStartOfMenuForIdentifier(
            &self,
            child_menu: &UIMenu,
            parent_identifier: &UIMenuIdentifier,
        );

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method(insertChildMenu:atEndOfMenuForIdentifier:)]
        unsafe fn insertChildMenu_atEndOfMenuForIdentifier(
            &self,
            child_menu: &UIMenu,
            parent_identifier: &UIMenuIdentifier,
        );

        #[cfg(feature = "UIMenu")]
        #[method(removeMenuForIdentifier:)]
        unsafe fn removeMenuForIdentifier(&self, removed_identifier: &UIMenuIdentifier);
    }

    unsafe impl ProtocolType for dyn UIMenuBuilder {}
);
