//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSTouchBarCustomizationIdentifier = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTouchBar")]
    pub struct NSTouchBar;

    #[cfg(feature = "AppKit_NSTouchBar")]
    unsafe impl ClassType for NSTouchBar {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTouchBar")]
unsafe impl NSCoding for NSTouchBar {}

#[cfg(feature = "AppKit_NSTouchBar")]
unsafe impl NSObjectProtocol for NSTouchBar {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTouchBar")]
    unsafe impl NSTouchBar {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other customizationIdentifier)]
        pub unsafe fn customizationIdentifier(
            &self,
        ) -> Option<Id<NSTouchBarCustomizationIdentifier, Shared>>;

        #[method(setCustomizationIdentifier:)]
        pub unsafe fn setCustomizationIdentifier(
            &self,
            customization_identifier: Option<&NSTouchBarCustomizationIdentifier>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other customizationAllowedItemIdentifiers)]
        pub unsafe fn customizationAllowedItemIdentifiers(
            &self,
        ) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setCustomizationAllowedItemIdentifiers:)]
        pub unsafe fn setCustomizationAllowedItemIdentifiers(
            &self,
            customization_allowed_item_identifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other customizationRequiredItemIdentifiers)]
        pub unsafe fn customizationRequiredItemIdentifiers(
            &self,
        ) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setCustomizationRequiredItemIdentifiers:)]
        pub unsafe fn setCustomizationRequiredItemIdentifiers(
            &self,
            customization_required_item_identifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other defaultItemIdentifiers)]
        pub unsafe fn defaultItemIdentifiers(
            &self,
        ) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setDefaultItemIdentifiers:)]
        pub unsafe fn setDefaultItemIdentifiers(
            &self,
            default_item_identifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other itemIdentifiers)]
        pub unsafe fn itemIdentifiers(&self) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;

        #[method_id(@__retain_semantics Other principalItemIdentifier)]
        pub unsafe fn principalItemIdentifier(
            &self,
        ) -> Option<Id<NSTouchBarItemIdentifier, Shared>>;

        #[method(setPrincipalItemIdentifier:)]
        pub unsafe fn setPrincipalItemIdentifier(
            &self,
            principal_item_identifier: Option<&NSTouchBarItemIdentifier>,
        );

        #[method_id(@__retain_semantics Other escapeKeyReplacementItemIdentifier)]
        pub unsafe fn escapeKeyReplacementItemIdentifier(
            &self,
        ) -> Option<Id<NSTouchBarItemIdentifier, Shared>>;

        #[method(setEscapeKeyReplacementItemIdentifier:)]
        pub unsafe fn setEscapeKeyReplacementItemIdentifier(
            &self,
            escape_key_replacement_item_identifier: Option<&NSTouchBarItemIdentifier>,
        );

        #[cfg(all(feature = "AppKit_NSTouchBarItem", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other templateItems)]
        pub unsafe fn templateItems(&self) -> Id<NSSet<NSTouchBarItem>, Shared>;

        #[cfg(all(feature = "AppKit_NSTouchBarItem", feature = "Foundation_NSSet"))]
        #[method(setTemplateItems:)]
        pub unsafe fn setTemplateItems(&self, template_items: &NSSet<NSTouchBarItem>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Id<ProtocolObject<dyn NSTouchBarDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSTouchBarDelegate>>);

        #[cfg(feature = "AppKit_NSTouchBarItem")]
        #[method_id(@__retain_semantics Other itemForIdentifier:)]
        pub unsafe fn itemForIdentifier(
            &self,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Option<Id<NSTouchBarItem, Shared>>;

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(isAutomaticCustomizeTouchBarMenuItemEnabled)]
        pub unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled() -> bool;

        #[method(setAutomaticCustomizeTouchBarMenuItemEnabled:)]
        pub unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled(
            automatic_customize_touch_bar_menu_item_enabled: bool,
        );
    }
);

extern_protocol!(
    pub unsafe trait NSTouchBarDelegate: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSTouchBar", feature = "AppKit_NSTouchBarItem"))]
        #[optional]
        #[method_id(@__retain_semantics Other touchBar:makeItemForIdentifier:)]
        unsafe fn touchBar_makeItemForIdentifier(
            &self,
            touch_bar: &NSTouchBar,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Option<Id<NSTouchBarItem, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSTouchBarDelegate {}
);

extern_protocol!(
    pub unsafe trait NSTouchBarProvider: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method_id(@__retain_semantics Other touchBar)]
        unsafe fn touchBar(&self) -> Option<Id<NSTouchBar, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSTouchBarProvider {}
);

extern_methods!(
    /// NSTouchBarProvider
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method_id(@__retain_semantics Other touchBar)]
        pub unsafe fn touchBar(&self) -> Option<Id<NSTouchBar, Shared>>;

        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method(setTouchBar:)]
        pub unsafe fn setTouchBar(&self, touch_bar: Option<&NSTouchBar>);

        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method_id(@__retain_semantics Other makeTouchBar)]
        pub unsafe fn makeTouchBar(&self) -> Option<Id<NSTouchBar, Shared>>;
    }
);

#[cfg(feature = "AppKit_NSResponder")]
unsafe impl NSTouchBarProvider for NSResponder {}

extern_methods!(
    /// NSTouchBarCustomization
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(isAutomaticCustomizeTouchBarMenuItemEnabled)]
        pub unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled(&self) -> bool;

        #[method(setAutomaticCustomizeTouchBarMenuItemEnabled:)]
        pub unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled(
            &self,
            automatic_customize_touch_bar_menu_item_enabled: bool,
        );

        #[method(toggleTouchBarCustomizationPalette:)]
        pub unsafe fn toggleTouchBarCustomizationPalette(&self, sender: Option<&Object>);
    }
);
