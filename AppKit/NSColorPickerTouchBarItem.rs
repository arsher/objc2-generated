//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
    pub struct NSColorPickerTouchBarItem;

    #[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
    unsafe impl ClassType for NSColorPickerTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

#[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
unsafe impl NSCoding for NSColorPickerTouchBarItem {}

#[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
unsafe impl NSObjectProtocol for NSColorPickerTouchBarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
    unsafe impl NSColorPickerTouchBarItem {
        #[method_id(@__retain_semantics Other colorPickerWithIdentifier:)]
        pub unsafe fn colorPickerWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other textColorPickerWithIdentifier:)]
        pub unsafe fn textColorPickerWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other strokeColorPickerWithIdentifier:)]
        pub unsafe fn strokeColorPickerWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other colorPickerWithIdentifier:buttonImage:)]
        pub unsafe fn colorPickerWithIdentifier_buttonImage(
            identifier: &NSTouchBarItemIdentifier,
            image: &NSImage,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Id<NSColor, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);

        #[method(showsAlpha)]
        pub unsafe fn showsAlpha(&self) -> bool;

        #[method(setShowsAlpha:)]
        pub unsafe fn setShowsAlpha(&self, shows_alpha: bool);

        #[cfg(all(feature = "AppKit_NSColorSpace", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other allowedColorSpaces)]
        pub unsafe fn allowedColorSpaces(&self) -> Option<Id<NSArray<NSColorSpace>, Shared>>;

        #[cfg(all(feature = "AppKit_NSColorSpace", feature = "Foundation_NSArray"))]
        #[method(setAllowedColorSpaces:)]
        pub unsafe fn setAllowedColorSpaces(
            &self,
            allowed_color_spaces: Option<&NSArray<NSColorSpace>>,
        );

        #[cfg(feature = "AppKit_NSColorList")]
        #[method_id(@__retain_semantics Other colorList)]
        pub unsafe fn colorList(&self) -> Option<Id<NSColorList, Shared>>;

        #[cfg(feature = "AppKit_NSColorList")]
        #[method(setColorList:)]
        pub unsafe fn setColorList(&self, color_list: Option<&NSColorList>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
    unsafe impl NSColorPickerTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);
