//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIContextMenuConfigurationElementOrder(pub NSInteger);
impl UIContextMenuConfigurationElementOrder {
    #[doc(alias = "UIContextMenuConfigurationElementOrderAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIContextMenuConfigurationElementOrderPriority")]
    pub const Priority: Self = Self(1);
    #[doc(alias = "UIContextMenuConfigurationElementOrderFixed")]
    pub const Fixed: Self = Self(2);
}

unsafe impl Encode for UIContextMenuConfigurationElementOrder {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIContextMenuConfigurationElementOrder {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(all(feature = "UIMenu", feature = "UIMenuElement", feature = "block2"))]
pub type UIContextMenuActionProvider =
    *mut Block<dyn Fn(NonNull<NSArray<UIMenuElement>>) -> *mut UIMenu>;

#[cfg(all(
    feature = "UIResponder",
    feature = "UIViewController",
    feature = "block2"
))]
pub type UIContextMenuContentPreviewProvider = *mut Block<dyn Fn() -> *mut UIViewController>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIContextMenuConfiguration;

    unsafe impl ClassType for UIContextMenuConfiguration {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIContextMenuConfiguration {}

extern_methods!(
    unsafe impl UIContextMenuConfiguration {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<ProtocolObject<dyn NSCopying>>;

        #[method_id(@__retain_semantics Other secondaryItemIdentifiers)]
        pub unsafe fn secondaryItemIdentifiers(&self) -> Id<NSSet<ProtocolObject<dyn NSCopying>>>;

        #[method(setSecondaryItemIdentifiers:)]
        pub unsafe fn setSecondaryItemIdentifiers(
            &self,
            secondary_item_identifiers: &NSSet<ProtocolObject<dyn NSCopying>>,
        );

        #[method(badgeCount)]
        pub unsafe fn badgeCount(&self) -> NSInteger;

        #[method(setBadgeCount:)]
        pub unsafe fn setBadgeCount(&self, badge_count: NSInteger);

        #[method(preferredMenuElementOrder)]
        pub unsafe fn preferredMenuElementOrder(&self) -> UIContextMenuConfigurationElementOrder;

        #[method(setPreferredMenuElementOrder:)]
        pub unsafe fn setPreferredMenuElementOrder(
            &self,
            preferred_menu_element_order: UIContextMenuConfigurationElementOrder,
        );

        #[cfg(all(
            feature = "UIMenu",
            feature = "UIMenuElement",
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other configurationWithIdentifier:previewProvider:actionProvider:)]
        pub unsafe fn configurationWithIdentifier_previewProvider_actionProvider(
            identifier: Option<&ProtocolObject<dyn NSCopying>>,
            preview_provider: UIContextMenuContentPreviewProvider,
            action_provider: UIContextMenuActionProvider,
            mtm: MainThreadMarker,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIContextMenuConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
