//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSharingServicePickerTouchBarItem")]
    pub struct NSSharingServicePickerTouchBarItem;

    #[cfg(feature = "AppKit_NSSharingServicePickerTouchBarItem")]
    unsafe impl ClassType for NSSharingServicePickerTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSharingServicePickerTouchBarItem")]
    unsafe impl NSSharingServicePickerTouchBarItem {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<NSSharingServicePickerTouchBarItemDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&NSSharingServicePickerTouchBarItemDelegate>,
        );

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other buttonTitle)]
        pub unsafe fn buttonTitle(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setButtonTitle:)]
        pub unsafe fn setButtonTitle(&self, buttonTitle: &NSString);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other buttonImage)]
        pub unsafe fn buttonImage(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setButtonImage:)]
        pub unsafe fn setButtonImage(&self, buttonImage: Option<&NSImage>);
    }
);

extern_protocol!(
    pub struct NSSharingServicePickerTouchBarItemDelegate;

    unsafe impl ProtocolType for NSSharingServicePickerTouchBarItemDelegate {
        #[cfg(all(
            feature = "AppKit_NSSharingServicePickerTouchBarItem",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other itemsForSharingServicePickerTouchBarItem:)]
        pub unsafe fn itemsForSharingServicePickerTouchBarItem(
            &self,
            pickerTouchBarItem: &NSSharingServicePickerTouchBarItem,
        ) -> Id<NSArray, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSSharingServicePickerTouchBarItem")]
    unsafe impl NSSharingServicePickerTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);
