//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTableHeaderView")]
    pub struct NSTableHeaderView;

    #[cfg(feature = "AppKit_NSTableHeaderView")]
    unsafe impl ClassType for NSTableHeaderView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSTableHeaderView")]
unsafe impl NSAccessibility for NSTableHeaderView {}

#[cfg(feature = "AppKit_NSTableHeaderView")]
unsafe impl NSAccessibilityElementProtocol for NSTableHeaderView {}

#[cfg(feature = "AppKit_NSTableHeaderView")]
unsafe impl NSAnimatablePropertyContainer for NSTableHeaderView {}

#[cfg(feature = "AppKit_NSTableHeaderView")]
unsafe impl NSAppearanceCustomization for NSTableHeaderView {}

#[cfg(feature = "AppKit_NSTableHeaderView")]
unsafe impl NSCoding for NSTableHeaderView {}

#[cfg(feature = "AppKit_NSTableHeaderView")]
unsafe impl NSDraggingDestination for NSTableHeaderView {}

#[cfg(feature = "AppKit_NSTableHeaderView")]
unsafe impl NSObjectProtocol for NSTableHeaderView {}

#[cfg(feature = "AppKit_NSTableHeaderView")]
unsafe impl NSUserInterfaceItemIdentification for NSTableHeaderView {}

#[cfg(feature = "AppKit_NSTableHeaderView")]
unsafe impl NSViewToolTipOwner for NSTableHeaderView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTableHeaderView")]
    unsafe impl NSTableHeaderView {
        #[cfg(feature = "AppKit_NSTableView")]
        #[method_id(@__retain_semantics Other tableView)]
        pub unsafe fn tableView(&self) -> Option<Id<NSTableView>>;

        #[cfg(feature = "AppKit_NSTableView")]
        #[method(setTableView:)]
        pub unsafe fn setTableView(&self, table_view: Option<&NSTableView>);

        #[method(draggedColumn)]
        pub unsafe fn draggedColumn(&self) -> NSInteger;

        #[method(draggedDistance)]
        pub unsafe fn draggedDistance(&self) -> CGFloat;

        #[method(resizedColumn)]
        pub unsafe fn resizedColumn(&self) -> NSInteger;

        #[method(headerRectOfColumn:)]
        pub unsafe fn headerRectOfColumn(&self, column: NSInteger) -> NSRect;

        #[method(columnAtPoint:)]
        pub unsafe fn columnAtPoint(&self, point: NSPoint) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSTableHeaderView")]
    unsafe impl NSTableHeaderView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSTableHeaderView")]
    unsafe impl NSTableHeaderView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTableHeaderView")]
    unsafe impl NSTableHeaderView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
