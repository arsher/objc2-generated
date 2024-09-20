//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITabSidebarItem;

    unsafe impl ClassType for UITabSidebarItem {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSCopying for UITabSidebarItem {}

unsafe impl CopyingHelper for UITabSidebarItem {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UITabSidebarItem {}

extern_methods!(
    unsafe impl UITabSidebarItem {
        #[cfg(feature = "UITab")]
        #[method_id(@__retain_semantics Other tab)]
        pub unsafe fn tab(&self) -> Option<Retained<UITab>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other action)]
        pub unsafe fn action(&self) -> Option<Retained<UIAction>>;

        #[cfg(all(
            feature = "UICellConfigurationState",
            feature = "UIViewConfigurationState"
        ))]
        #[method_id(@__retain_semantics Other configurationState)]
        pub unsafe fn configurationState(&self) -> Retained<UICellConfigurationState>;

        #[cfg(feature = "UIContentConfiguration")]
        #[method_id(@__retain_semantics Other contentConfiguration)]
        pub unsafe fn contentConfiguration(
            &self,
        ) -> Retained<ProtocolObject<dyn UIContentConfiguration>>;

        #[cfg(feature = "UIContentConfiguration")]
        #[method(setContentConfiguration:)]
        pub unsafe fn setContentConfiguration(
            &self,
            content_configuration: &ProtocolObject<dyn UIContentConfiguration>,
        );

        #[cfg(feature = "UIBackgroundConfiguration")]
        #[method_id(@__retain_semantics Other backgroundConfiguration)]
        pub unsafe fn backgroundConfiguration(&self) -> Retained<UIBackgroundConfiguration>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        #[method(setBackgroundConfiguration:)]
        pub unsafe fn setBackgroundConfiguration(
            &self,
            background_configuration: &UIBackgroundConfiguration,
        );

        #[cfg(feature = "UICellAccessory")]
        #[method_id(@__retain_semantics Other accessories)]
        pub unsafe fn accessories(&self) -> Retained<NSArray<UICellAccessory>>;

        #[cfg(feature = "UICellAccessory")]
        #[method(setAccessories:)]
        pub unsafe fn setAccessories(&self, accessories: &NSArray<UICellAccessory>);

        #[cfg(feature = "UIListContentConfiguration")]
        #[method_id(@__retain_semantics Other defaultContentConfiguration)]
        pub unsafe fn defaultContentConfiguration(&self) -> Retained<UIListContentConfiguration>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        #[method_id(@__retain_semantics Other defaultBackgroundConfiguration)]
        pub unsafe fn defaultBackgroundConfiguration(&self) -> Retained<UIBackgroundConfiguration>;

        #[method_id(@__retain_semantics Other itemFromRequest:)]
        pub unsafe fn itemFromRequest(request: &UITabSidebarItemRequest) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITabSidebarItemRequest;

    unsafe impl ClassType for UITabSidebarItemRequest {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITabSidebarItemRequest {}

extern_methods!(
    unsafe impl UITabSidebarItemRequest {
        #[cfg(feature = "UITab")]
        #[method_id(@__retain_semantics Other tab)]
        pub unsafe fn tab(&self) -> Option<Retained<UITab>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other action)]
        pub unsafe fn action(&self) -> Option<Retained<UIAction>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
