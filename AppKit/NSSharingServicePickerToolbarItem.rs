//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    pub struct NSSharingServicePickerToolbarItem;

    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    unsafe impl ClassType for NSSharingServicePickerToolbarItem {
        #[inherits(NSObject)]
        type Super = NSToolbarItem;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
unsafe impl NSCopying for NSSharingServicePickerToolbarItem {}

#[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
unsafe impl NSObjectProtocol for NSSharingServicePickerToolbarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    unsafe impl NSSharingServicePickerToolbarItem {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSSharingServicePickerToolbarItemDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSharingServicePickerToolbarItemDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSToolbarItem`
    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    unsafe impl NSSharingServicePickerToolbarItem {
        #[method_id(@__retain_semantics Init initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            this: Option<Allocated<Self>>,
            item_identifier: &NSToolbarItemIdentifier,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    unsafe impl NSSharingServicePickerToolbarItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSSharingServicePickerToolbarItemDelegate:
        NSSharingServicePickerDelegate
    {
        #[cfg(all(
            feature = "AppKit_NSSharingServicePickerToolbarItem",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other itemsForSharingServicePickerToolbarItem:)]
        unsafe fn itemsForSharingServicePickerToolbarItem(
            &self,
            picker_toolbar_item: &NSSharingServicePickerToolbarItem,
        ) -> Id<NSArray>;
    }

    unsafe impl ProtocolType for dyn NSSharingServicePickerToolbarItemDelegate {}
);
