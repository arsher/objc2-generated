//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTrackingSeparatorToolbarItem")]
    pub struct NSTrackingSeparatorToolbarItem;

    #[cfg(feature = "AppKit_NSTrackingSeparatorToolbarItem")]
    unsafe impl ClassType for NSTrackingSeparatorToolbarItem {
        #[inherits(NSObject)]
        type Super = NSToolbarItem;
    }
);

#[cfg(feature = "AppKit_NSTrackingSeparatorToolbarItem")]
unsafe impl NSObjectProtocol for NSTrackingSeparatorToolbarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTrackingSeparatorToolbarItem")]
    unsafe impl NSTrackingSeparatorToolbarItem {
        #[cfg(feature = "AppKit_NSSplitView")]
        #[method_id(@__retain_semantics Other trackingSeparatorToolbarItemWithIdentifier:splitView:dividerIndex:)]
        pub unsafe fn trackingSeparatorToolbarItemWithIdentifier_splitView_dividerIndex(
            identifier: &NSToolbarItemIdentifier,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[method_id(@__retain_semantics Other splitView)]
        pub unsafe fn splitView(&self) -> Id<NSSplitView, Shared>;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[method(setSplitView:)]
        pub unsafe fn setSplitView(&self, split_view: &NSSplitView);

        #[method(dividerIndex)]
        pub unsafe fn dividerIndex(&self) -> NSInteger;

        #[method(setDividerIndex:)]
        pub unsafe fn setDividerIndex(&self, divider_index: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSToolbarItem`
    #[cfg(feature = "AppKit_NSTrackingSeparatorToolbarItem")]
    unsafe impl NSTrackingSeparatorToolbarItem {
        #[method_id(@__retain_semantics Init initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            this: Option<Allocated<Self>>,
            item_identifier: &NSToolbarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);
