//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTokenStyle {
        NSTokenStyleDefault = 0,
        NSTokenStyleNone = 1,
        NSTokenStyleRounded = 2,
        NSTokenStyleSquared = 3,
        NSTokenStylePlainSquared = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTokenFieldCell;

    unsafe impl ClassType for NSTokenFieldCell {
        #[inherits(AppKit::NSActionCell, AppKit::NSCell, NSObject)]
        type Super = AppKit::NSTextFieldCell;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTokenFieldCell")]
    unsafe impl NSTokenFieldCell {
        #[method(tokenStyle)]
        pub unsafe fn tokenStyle(&self) -> NSTokenStyle;

        #[method(setTokenStyle:)]
        pub unsafe fn setTokenStyle(&self, tokenStyle: NSTokenStyle);

        #[method(completionDelay)]
        pub unsafe fn completionDelay(&self) -> NSTimeInterval;

        #[method(setCompletionDelay:)]
        pub unsafe fn setCompletionDelay(&self, completionDelay: NSTimeInterval);

        #[method(defaultCompletionDelay)]
        pub unsafe fn defaultCompletionDelay() -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other tokenizingCharacterSet)]
        pub unsafe fn tokenizingCharacterSet(&self) -> Id<Foundation::NSCharacterSet, Shared>;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method(setTokenizingCharacterSet:)]
        pub unsafe fn setTokenizingCharacterSet(
            &self,
            tokenizingCharacterSet: Option<&Foundation::NSCharacterSet>,
        );

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other defaultTokenizingCharacterSet)]
        pub unsafe fn defaultTokenizingCharacterSet() -> Id<Foundation::NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AppKit::NSTokenFieldCellDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AppKit::NSTokenFieldCellDelegate>);
    }
);

extern_protocol!(
    pub struct NSTokenFieldCellDelegate;

    unsafe impl ProtocolType for NSTokenFieldCellDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:completionsForSubstring:indexOfToken:indexOfSelectedItem:)]
        pub unsafe fn tokenFieldCell_completionsForSubstring_indexOfToken_indexOfSelectedItem(
            &self,
            tokenFieldCell: &AppKit::NSTokenFieldCell,
            substring: &Foundation::NSString,
            tokenIndex: NSInteger,
            selectedIndex: NonNull<NSInteger>,
        ) -> Id<Foundation::NSArray, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:shouldAddObjects:atIndex:)]
        pub unsafe fn tokenFieldCell_shouldAddObjects_atIndex(
            &self,
            tokenFieldCell: &AppKit::NSTokenFieldCell,
            tokens: &Foundation::NSArray,
            index: NSUInteger,
        ) -> Id<Foundation::NSArray, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:displayStringForRepresentedObject:)]
        pub unsafe fn tokenFieldCell_displayStringForRepresentedObject(
            &self,
            tokenFieldCell: &AppKit::NSTokenFieldCell,
            representedObject: &Object,
        ) -> Option<Id<Foundation::NSString, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:editingStringForRepresentedObject:)]
        pub unsafe fn tokenFieldCell_editingStringForRepresentedObject(
            &self,
            tokenFieldCell: &AppKit::NSTokenFieldCell,
            representedObject: &Object,
        ) -> Option<Id<Foundation::NSString, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:representedObjectForEditingString:)]
        pub unsafe fn tokenFieldCell_representedObjectForEditingString(
            &self,
            tokenFieldCell: &AppKit::NSTokenFieldCell,
            editingString: &Foundation::NSString,
        ) -> Option<Id<Object, Shared>>;

        #[optional]
        #[method(tokenFieldCell:writeRepresentedObjects:toPasteboard:)]
        pub unsafe fn tokenFieldCell_writeRepresentedObjects_toPasteboard(
            &self,
            tokenFieldCell: &AppKit::NSTokenFieldCell,
            objects: &Foundation::NSArray,
            pboard: &AppKit::NSPasteboard,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:readFromPasteboard:)]
        pub unsafe fn tokenFieldCell_readFromPasteboard(
            &self,
            tokenFieldCell: &AppKit::NSTokenFieldCell,
            pboard: &AppKit::NSPasteboard,
        ) -> Option<Id<Foundation::NSArray, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:menuForRepresentedObject:)]
        pub unsafe fn tokenFieldCell_menuForRepresentedObject(
            &self,
            tokenFieldCell: &AppKit::NSTokenFieldCell,
            representedObject: &Object,
        ) -> Option<Id<AppKit::NSMenu, Shared>>;

        #[optional]
        #[method(tokenFieldCell:hasMenuForRepresentedObject:)]
        pub unsafe fn tokenFieldCell_hasMenuForRepresentedObject(
            &self,
            tokenFieldCell: &AppKit::NSTokenFieldCell,
            representedObject: &Object,
        ) -> bool;

        #[optional]
        #[method(tokenFieldCell:styleForRepresentedObject:)]
        pub unsafe fn tokenFieldCell_styleForRepresentedObject(
            &self,
            tokenFieldCell: &AppKit::NSTokenFieldCell,
            representedObject: &Object,
        ) -> NSTokenStyle;
    }
);

extern_static!(NSDefaultTokenStyle: NSTokenStyle = NSTokenStyleDefault);

extern_static!(NSPlainTextTokenStyle: NSTokenStyle = NSTokenStyleNone);

extern_static!(NSRoundedTokenStyle: NSTokenStyle = NSTokenStyleRounded);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSTokenFieldCell")]
    unsafe impl AppKit::NSTokenFieldCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &Foundation::NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&AppKit::NSImage>,
        ) -> Id<Self, Shared>;
    }
);
