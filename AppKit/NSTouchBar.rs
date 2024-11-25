//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstouchbarcustomizationidentifier?language=objc)
pub type NSTouchBarCustomizationIdentifier = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstouchbar?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTouchBar;
);

unsafe impl NSCoding for NSTouchBar {}

unsafe impl NSObjectProtocol for NSTouchBar {}

extern_methods!(
    unsafe impl NSTouchBar {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other customizationIdentifier)]
        pub unsafe fn customizationIdentifier(
            &self,
        ) -> Option<Retained<NSTouchBarCustomizationIdentifier>>;

        #[method(setCustomizationIdentifier:)]
        pub unsafe fn setCustomizationIdentifier(
            &self,
            customization_identifier: Option<&NSTouchBarCustomizationIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[method_id(@__retain_semantics Other customizationAllowedItemIdentifiers)]
        pub unsafe fn customizationAllowedItemIdentifiers(
            &self,
        ) -> Retained<NSArray<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        #[method(setCustomizationAllowedItemIdentifiers:)]
        pub unsafe fn setCustomizationAllowedItemIdentifiers(
            &self,
            customization_allowed_item_identifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[method_id(@__retain_semantics Other customizationRequiredItemIdentifiers)]
        pub unsafe fn customizationRequiredItemIdentifiers(
            &self,
        ) -> Retained<NSArray<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        #[method(setCustomizationRequiredItemIdentifiers:)]
        pub unsafe fn setCustomizationRequiredItemIdentifiers(
            &self,
            customization_required_item_identifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[method_id(@__retain_semantics Other defaultItemIdentifiers)]
        pub unsafe fn defaultItemIdentifiers(&self) -> Retained<NSArray<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        #[method(setDefaultItemIdentifiers:)]
        pub unsafe fn setDefaultItemIdentifiers(
            &self,
            default_item_identifiers: &NSArray<NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[method_id(@__retain_semantics Other itemIdentifiers)]
        pub unsafe fn itemIdentifiers(&self) -> Retained<NSArray<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        #[method_id(@__retain_semantics Other principalItemIdentifier)]
        pub unsafe fn principalItemIdentifier(&self) -> Option<Retained<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        #[method(setPrincipalItemIdentifier:)]
        pub unsafe fn setPrincipalItemIdentifier(
            &self,
            principal_item_identifier: Option<&NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[method_id(@__retain_semantics Other escapeKeyReplacementItemIdentifier)]
        pub unsafe fn escapeKeyReplacementItemIdentifier(
            &self,
        ) -> Option<Retained<NSTouchBarItemIdentifier>>;

        #[cfg(feature = "NSTouchBarItem")]
        #[method(setEscapeKeyReplacementItemIdentifier:)]
        pub unsafe fn setEscapeKeyReplacementItemIdentifier(
            &self,
            escape_key_replacement_item_identifier: Option<&NSTouchBarItemIdentifier>,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[method_id(@__retain_semantics Other templateItems)]
        pub unsafe fn templateItems(&self) -> Retained<NSSet<NSTouchBarItem>>;

        #[cfg(feature = "NSTouchBarItem")]
        #[method(setTemplateItems:)]
        pub unsafe fn setTemplateItems(&self, template_items: &NSSet<NSTouchBarItem>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSTouchBarDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSTouchBarDelegate>>);

        #[cfg(feature = "NSTouchBarItem")]
        #[method_id(@__retain_semantics Other itemForIdentifier:)]
        pub unsafe fn itemForIdentifier(
            &self,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Option<Retained<NSTouchBarItem>>;

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(isAutomaticCustomizeTouchBarMenuItemEnabled)]
        pub unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled(mtm: MainThreadMarker) -> bool;

        #[method(setAutomaticCustomizeTouchBarMenuItemEnabled:)]
        pub unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled(
            automatic_customize_touch_bar_menu_item_enabled: bool,
            mtm: MainThreadMarker,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTouchBar {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstouchbardelegate?language=objc)
    pub unsafe trait NSTouchBarDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "NSTouchBarItem")]
        #[optional]
        #[method_id(@__retain_semantics Other touchBar:makeItemForIdentifier:)]
        unsafe fn touchBar_makeItemForIdentifier(
            &self,
            touch_bar: &NSTouchBar,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Option<Retained<NSTouchBarItem>>;
    }

    unsafe impl ProtocolType for dyn NSTouchBarDelegate {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstouchbarprovider?language=objc)
    pub unsafe trait NSTouchBarProvider: NSObjectProtocol + MainThreadOnly {
        #[method_id(@__retain_semantics Other touchBar)]
        unsafe fn touchBar(&self) -> Option<Retained<NSTouchBar>>;
    }

    unsafe impl ProtocolType for dyn NSTouchBarProvider {}
);

extern_methods!(
    /// NSTouchBarProvider
    #[cfg(feature = "NSResponder")]
    unsafe impl NSResponder {
        #[method_id(@__retain_semantics Other touchBar)]
        pub unsafe fn touchBar(&self) -> Option<Retained<NSTouchBar>>;

        #[method(setTouchBar:)]
        pub unsafe fn setTouchBar(&self, touch_bar: Option<&NSTouchBar>);

        #[method_id(@__retain_semantics Other makeTouchBar)]
        pub unsafe fn makeTouchBar(&self) -> Option<Retained<NSTouchBar>>;
    }
);

#[cfg(feature = "NSResponder")]
unsafe impl NSTouchBarProvider for NSResponder {}

extern_methods!(
    /// NSTouchBarCustomization
    #[cfg(all(feature = "NSApplication", feature = "NSResponder"))]
    unsafe impl NSApplication {
        #[method(isAutomaticCustomizeTouchBarMenuItemEnabled)]
        pub unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled(&self) -> bool;

        #[method(setAutomaticCustomizeTouchBarMenuItemEnabled:)]
        pub unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled(
            &self,
            automatic_customize_touch_bar_menu_item_enabled: bool,
        );

        #[method(toggleTouchBarCustomizationPalette:)]
        pub unsafe fn toggleTouchBarCustomizationPalette(&self, sender: Option<&AnyObject>);
    }
);
