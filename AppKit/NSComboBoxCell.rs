//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSComboBoxCell;

    unsafe impl ClassType for NSComboBoxCell {
        #[inherits(AppKit::NSActionCell, AppKit::NSCell, NSObject)]
        type Super = AppKit::NSTextFieldCell;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSComboBoxCell")]
    unsafe impl NSComboBoxCell {
        #[method(hasVerticalScroller)]
        pub unsafe fn hasVerticalScroller(&self) -> bool;

        #[method(setHasVerticalScroller:)]
        pub unsafe fn setHasVerticalScroller(&self, hasVerticalScroller: bool);

        #[method(intercellSpacing)]
        pub unsafe fn intercellSpacing(&self) -> NSSize;

        #[method(setIntercellSpacing:)]
        pub unsafe fn setIntercellSpacing(&self, intercellSpacing: NSSize);

        #[method(itemHeight)]
        pub unsafe fn itemHeight(&self) -> CGFloat;

        #[method(setItemHeight:)]
        pub unsafe fn setItemHeight(&self, itemHeight: CGFloat);

        #[method(numberOfVisibleItems)]
        pub unsafe fn numberOfVisibleItems(&self) -> NSInteger;

        #[method(setNumberOfVisibleItems:)]
        pub unsafe fn setNumberOfVisibleItems(&self, numberOfVisibleItems: NSInteger);

        #[method(isButtonBordered)]
        pub unsafe fn isButtonBordered(&self) -> bool;

        #[method(setButtonBordered:)]
        pub unsafe fn setButtonBordered(&self, buttonBordered: bool);

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[method(noteNumberOfItemsChanged)]
        pub unsafe fn noteNumberOfItemsChanged(&self);

        #[method(usesDataSource)]
        pub unsafe fn usesDataSource(&self) -> bool;

        #[method(setUsesDataSource:)]
        pub unsafe fn setUsesDataSource(&self, usesDataSource: bool);

        #[method(scrollItemAtIndexToTop:)]
        pub unsafe fn scrollItemAtIndexToTop(&self, index: NSInteger);

        #[method(scrollItemAtIndexToVisible:)]
        pub unsafe fn scrollItemAtIndexToVisible(&self, index: NSInteger);

        #[method(selectItemAtIndex:)]
        pub unsafe fn selectItemAtIndex(&self, index: NSInteger);

        #[method(deselectItemAtIndex:)]
        pub unsafe fn deselectItemAtIndex(&self, index: NSInteger);

        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(completes)]
        pub unsafe fn completes(&self) -> bool;

        #[method(setCompletes:)]
        pub unsafe fn setCompletes(&self, completes: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other completedString:)]
        pub unsafe fn completedString(
            &self,
            string: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSString, Shared>>;

        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<AppKit::NSComboBoxCellDataSource, Shared>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(&self, dataSource: Option<&AppKit::NSComboBoxCellDataSource>);

        #[method(addItemWithObjectValue:)]
        pub unsafe fn addItemWithObjectValue(&self, object: &Object);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(addItemsWithObjectValues:)]
        pub unsafe fn addItemsWithObjectValues(&self, objects: &Foundation::NSArray);

        #[method(insertItemWithObjectValue:atIndex:)]
        pub unsafe fn insertItemWithObjectValue_atIndex(&self, object: &Object, index: NSInteger);

        #[method(removeItemWithObjectValue:)]
        pub unsafe fn removeItemWithObjectValue(&self, object: &Object);

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[method(selectItemWithObjectValue:)]
        pub unsafe fn selectItemWithObjectValue(&self, object: Option<&Object>);

        #[method_id(@__retain_semantics Other itemObjectValueAtIndex:)]
        pub unsafe fn itemObjectValueAtIndex(&self, index: NSInteger) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other objectValueOfSelectedItem)]
        pub unsafe fn objectValueOfSelectedItem(&self) -> Option<Id<Object, Shared>>;

        #[method(indexOfItemWithObjectValue:)]
        pub unsafe fn indexOfItemWithObjectValue(&self, object: &Object) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other objectValues)]
        pub unsafe fn objectValues(&self) -> Id<Foundation::NSArray, Shared>;
    }
);

extern_protocol!(
    pub struct NSComboBoxCellDataSource;

    unsafe impl ProtocolType for NSComboBoxCellDataSource {
        #[optional]
        #[method(numberOfItemsInComboBoxCell:)]
        pub unsafe fn numberOfItemsInComboBoxCell(
            &self,
            comboBoxCell: &AppKit::NSComboBoxCell,
        ) -> NSInteger;

        #[optional]
        #[method_id(@__retain_semantics Other comboBoxCell:objectValueForItemAtIndex:)]
        pub unsafe fn comboBoxCell_objectValueForItemAtIndex(
            &self,
            comboBoxCell: &AppKit::NSComboBoxCell,
            index: NSInteger,
        ) -> Id<Object, Shared>;

        #[optional]
        #[method(comboBoxCell:indexOfItemWithStringValue:)]
        pub unsafe fn comboBoxCell_indexOfItemWithStringValue(
            &self,
            comboBoxCell: &AppKit::NSComboBoxCell,
            string: &Foundation::NSString,
        ) -> NSUInteger;

        #[optional]
        #[method_id(@__retain_semantics Other comboBoxCell:completedString:)]
        pub unsafe fn comboBoxCell_completedString(
            &self,
            comboBoxCell: &AppKit::NSComboBoxCell,
            uncompletedString: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSString, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSComboBoxCell")]
    unsafe impl AppKit::NSComboBoxCell {
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
