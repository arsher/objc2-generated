//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSMatrixMode {
        NSRadioModeMatrix = 0,
        NSHighlightModeMatrix = 1,
        NSListModeMatrix = 2,
        NSTrackModeMatrix = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSMatrix")]
    pub struct NSMatrix;

    #[cfg(feature = "AppKit_NSMatrix")]
    unsafe impl ClassType for NSMatrix {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSMatrix")]
unsafe impl NSAccessibility for NSMatrix {}

#[cfg(feature = "AppKit_NSMatrix")]
unsafe impl NSAccessibilityElementProtocol for NSMatrix {}

#[cfg(feature = "AppKit_NSMatrix")]
unsafe impl NSAnimatablePropertyContainer for NSMatrix {}

#[cfg(feature = "AppKit_NSMatrix")]
unsafe impl NSAppearanceCustomization for NSMatrix {}

#[cfg(feature = "AppKit_NSMatrix")]
unsafe impl NSCoding for NSMatrix {}

#[cfg(feature = "AppKit_NSMatrix")]
unsafe impl NSDraggingDestination for NSMatrix {}

#[cfg(feature = "AppKit_NSMatrix")]
unsafe impl NSObjectProtocol for NSMatrix {}

#[cfg(feature = "AppKit_NSMatrix")]
unsafe impl NSUserInterfaceItemIdentification for NSMatrix {}

#[cfg(feature = "AppKit_NSMatrix")]
unsafe impl NSUserInterfaceValidations for NSMatrix {}

#[cfg(feature = "AppKit_NSMatrix")]
unsafe impl NSViewToolTipOwner for NSMatrix {}

extern_methods!(
    #[cfg(feature = "AppKit_NSMatrix")]
    unsafe impl NSMatrix {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Init initWithFrame:mode:prototype:numberOfRows:numberOfColumns:)]
        pub unsafe fn initWithFrame_mode_prototype_numberOfRows_numberOfColumns(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
            mode: NSMatrixMode,
            cell: &NSCell,
            rows_high: NSInteger,
            cols_wide: NSInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithFrame:mode:cellClass:numberOfRows:numberOfColumns:)]
        pub unsafe fn initWithFrame_mode_cellClass_numberOfRows_numberOfColumns(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
            mode: NSMatrixMode,
            factory_id: Option<&AnyClass>,
            rows_high: NSInteger,
            cols_wide: NSInteger,
        ) -> Id<Self>;

        #[method(cellClass)]
        pub unsafe fn cellClass(&self) -> &'static AnyClass;

        #[method(setCellClass:)]
        pub unsafe fn setCellClass(&self, cell_class: &AnyClass);

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other prototype)]
        pub unsafe fn prototype(&self) -> Option<Id<NSCell>>;

        #[cfg(feature = "AppKit_NSCell")]
        #[method(setPrototype:)]
        pub unsafe fn setPrototype(&self, prototype: Option<&NSCell>);

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other makeCellAtRow:column:)]
        pub unsafe fn makeCellAtRow_column(&self, row: NSInteger, col: NSInteger) -> Id<NSCell>;

        #[method(mode)]
        pub unsafe fn mode(&self) -> NSMatrixMode;

        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSMatrixMode);

        #[method(allowsEmptySelection)]
        pub unsafe fn allowsEmptySelection(&self) -> bool;

        #[method(setAllowsEmptySelection:)]
        pub unsafe fn setAllowsEmptySelection(&self, allows_empty_selection: bool);

        #[method(sendAction:to:forAllCells:)]
        pub unsafe fn sendAction_to_forAllCells(
            &self,
            selector: Sel,
            object: &AnyObject,
            flag: bool,
        );

        #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other cells)]
        pub unsafe fn cells(&self) -> Id<NSArray<NSCell>>;

        #[method(sortUsingSelector:)]
        pub unsafe fn sortUsingSelector(&self, comparator: Sel);

        #[method(sortUsingFunction:context:)]
        pub unsafe fn sortUsingFunction_context(
            &self,
            compare: unsafe extern "C" fn(
                NonNull<AnyObject>,
                NonNull<AnyObject>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
        );

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other selectedCell)]
        pub unsafe fn selectedCell(&self) -> Option<Id<NSCell>>;

        #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other selectedCells)]
        pub unsafe fn selectedCells(&self) -> Id<NSArray<NSCell>>;

        #[method(selectedRow)]
        pub unsafe fn selectedRow(&self) -> NSInteger;

        #[method(selectedColumn)]
        pub unsafe fn selectedColumn(&self) -> NSInteger;

        #[method(isSelectionByRect)]
        pub unsafe fn isSelectionByRect(&self) -> bool;

        #[method(setSelectionByRect:)]
        pub unsafe fn setSelectionByRect(&self, selection_by_rect: bool);

        #[method(setSelectionFrom:to:anchor:highlight:)]
        pub unsafe fn setSelectionFrom_to_anchor_highlight(
            &self,
            start_pos: NSInteger,
            end_pos: NSInteger,
            anchor_pos: NSInteger,
            lit: bool,
        );

        #[method(deselectSelectedCell)]
        pub unsafe fn deselectSelectedCell(&self);

        #[method(deselectAllCells)]
        pub unsafe fn deselectAllCells(&self);

        #[method(selectCellAtRow:column:)]
        pub unsafe fn selectCellAtRow_column(&self, row: NSInteger, col: NSInteger);

        #[method(selectAll:)]
        pub unsafe fn selectAll(&self, sender: Option<&AnyObject>);

        #[method(selectCellWithTag:)]
        pub unsafe fn selectCellWithTag(&self, tag: NSInteger) -> bool;

        #[method(cellSize)]
        pub unsafe fn cellSize(&self) -> NSSize;

        #[method(setCellSize:)]
        pub unsafe fn setCellSize(&self, cell_size: NSSize);

        #[method(intercellSpacing)]
        pub unsafe fn intercellSpacing(&self) -> NSSize;

        #[method(setIntercellSpacing:)]
        pub unsafe fn setIntercellSpacing(&self, intercell_spacing: NSSize);

        #[method(setScrollable:)]
        pub unsafe fn setScrollable(&self, flag: bool);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other cellBackgroundColor)]
        pub unsafe fn cellBackgroundColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setCellBackgroundColor:)]
        pub unsafe fn setCellBackgroundColor(&self, cell_background_color: Option<&NSColor>);

        #[method(drawsCellBackground)]
        pub unsafe fn drawsCellBackground(&self) -> bool;

        #[method(setDrawsCellBackground:)]
        pub unsafe fn setDrawsCellBackground(&self, draws_cell_background: bool);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[method(setState:atRow:column:)]
        pub unsafe fn setState_atRow_column(
            &self,
            value: NSInteger,
            row: NSInteger,
            col: NSInteger,
        );

        #[method(getNumberOfRows:columns:)]
        pub unsafe fn getNumberOfRows_columns(
            &self,
            row_count: *mut NSInteger,
            col_count: *mut NSInteger,
        );

        #[method(numberOfRows)]
        pub unsafe fn numberOfRows(&self) -> NSInteger;

        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other cellAtRow:column:)]
        pub unsafe fn cellAtRow_column(&self, row: NSInteger, col: NSInteger)
            -> Option<Id<NSCell>>;

        #[method(cellFrameAtRow:column:)]
        pub unsafe fn cellFrameAtRow_column(&self, row: NSInteger, col: NSInteger) -> NSRect;

        #[cfg(feature = "AppKit_NSCell")]
        #[method(getRow:column:ofCell:)]
        pub unsafe fn getRow_column_ofCell(
            &self,
            row: NonNull<NSInteger>,
            col: NonNull<NSInteger>,
            cell: &NSCell,
        ) -> bool;

        #[method(getRow:column:forPoint:)]
        pub unsafe fn getRow_column_forPoint(
            &self,
            row: NonNull<NSInteger>,
            col: NonNull<NSInteger>,
            point: NSPoint,
        ) -> bool;

        #[method(renewRows:columns:)]
        pub unsafe fn renewRows_columns(&self, new_rows: NSInteger, new_cols: NSInteger);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(putCell:atRow:column:)]
        pub unsafe fn putCell_atRow_column(
            &self,
            new_cell: &NSCell,
            row: NSInteger,
            col: NSInteger,
        );

        #[method(addRow)]
        pub unsafe fn addRow(&self);

        #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSArray"))]
        #[method(addRowWithCells:)]
        pub unsafe fn addRowWithCells(&self, new_cells: &NSArray<NSCell>);

        #[method(insertRow:)]
        pub unsafe fn insertRow(&self, row: NSInteger);

        #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSArray"))]
        #[method(insertRow:withCells:)]
        pub unsafe fn insertRow_withCells(
            &self,
            row: NSInteger,
            new_cells: Option<&NSArray<NSCell>>,
        );

        #[method(removeRow:)]
        pub unsafe fn removeRow(&self, row: NSInteger);

        #[method(addColumn)]
        pub unsafe fn addColumn(&self);

        #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSArray"))]
        #[method(addColumnWithCells:)]
        pub unsafe fn addColumnWithCells(&self, new_cells: &NSArray<NSCell>);

        #[method(insertColumn:)]
        pub unsafe fn insertColumn(&self, column: NSInteger);

        #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSArray"))]
        #[method(insertColumn:withCells:)]
        pub unsafe fn insertColumn_withCells(
            &self,
            column: NSInteger,
            new_cells: Option<&NSArray<NSCell>>,
        );

        #[method(removeColumn:)]
        pub unsafe fn removeColumn(&self, col: NSInteger);

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other cellWithTag:)]
        pub unsafe fn cellWithTag(&self, tag: NSInteger) -> Option<Id<NSCell>>;

        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, double_action: Option<Sel>);

        #[method(autosizesCells)]
        pub unsafe fn autosizesCells(&self) -> bool;

        #[method(setAutosizesCells:)]
        pub unsafe fn setAutosizesCells(&self, autosizes_cells: bool);

        #[method(sizeToCells)]
        pub unsafe fn sizeToCells(&self);

        #[method(setValidateSize:)]
        pub unsafe fn setValidateSize(&self, flag: bool);

        #[method(drawCellAtRow:column:)]
        pub unsafe fn drawCellAtRow_column(&self, row: NSInteger, col: NSInteger);

        #[method(highlightCell:atRow:column:)]
        pub unsafe fn highlightCell_atRow_column(&self, flag: bool, row: NSInteger, col: NSInteger);

        #[method(isAutoscroll)]
        pub unsafe fn isAutoscroll(&self) -> bool;

        #[method(setAutoscroll:)]
        pub unsafe fn setAutoscroll(&self, autoscroll: bool);

        #[method(scrollCellToVisibleAtRow:column:)]
        pub unsafe fn scrollCellToVisibleAtRow_column(&self, row: NSInteger, col: NSInteger);

        #[method(mouseDownFlags)]
        pub unsafe fn mouseDownFlags(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;

        #[method(sendAction)]
        pub unsafe fn sendAction(&self) -> bool;

        #[method(sendDoubleAction)]
        pub unsafe fn sendDoubleAction(&self);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSMatrixDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSMatrixDelegate>>);

        #[cfg(feature = "AppKit_NSText")]
        #[method(textShouldBeginEditing:)]
        pub unsafe fn textShouldBeginEditing(&self, text_object: &NSText) -> bool;

        #[cfg(feature = "AppKit_NSText")]
        #[method(textShouldEndEditing:)]
        pub unsafe fn textShouldEndEditing(&self, text_object: &NSText) -> bool;

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(textDidBeginEditing:)]
        pub unsafe fn textDidBeginEditing(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(textDidEndEditing:)]
        pub unsafe fn textDidEndEditing(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(textDidChange:)]
        pub unsafe fn textDidChange(&self, notification: &NSNotification);

        #[method(selectText:)]
        pub unsafe fn selectText(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other selectTextAtRow:column:)]
        pub unsafe fn selectTextAtRow_column(
            &self,
            row: NSInteger,
            col: NSInteger,
        ) -> Option<Id<NSCell>>;

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(acceptsFirstMouse:)]
        pub unsafe fn acceptsFirstMouse(&self, event: Option<&NSEvent>) -> bool;

        #[method(resetCursorRects)]
        pub unsafe fn resetCursorRects(&self);

        #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSString"))]
        #[method(setToolTip:forCell:)]
        pub unsafe fn setToolTip_forCell(&self, tool_tip_string: Option<&NSString>, cell: &NSCell);

        #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other toolTipForCell:)]
        pub unsafe fn toolTipForCell(&self, cell: &NSCell) -> Option<Id<NSString>>;

        #[method(autorecalculatesCellSize)]
        pub unsafe fn autorecalculatesCellSize(&self) -> bool;

        #[method(setAutorecalculatesCellSize:)]
        pub unsafe fn setAutorecalculatesCellSize(&self, autorecalculates_cell_size: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSMatrix")]
    unsafe impl NSMatrix {
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
    #[cfg(feature = "AppKit_NSMatrix")]
    unsafe impl NSMatrix {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSMatrix")]
    unsafe impl NSMatrix {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSKeyboardUI
    #[cfg(feature = "AppKit_NSMatrix")]
    unsafe impl NSMatrix {
        #[method(tabKeyTraversesCells)]
        pub unsafe fn tabKeyTraversesCells(&self) -> bool;

        #[method(setTabKeyTraversesCells:)]
        pub unsafe fn setTabKeyTraversesCells(&self, tab_key_traverses_cells: bool);

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other keyCell)]
        pub unsafe fn keyCell(&self) -> Option<Id<NSCell>>;

        #[cfg(feature = "AppKit_NSCell")]
        #[method(setKeyCell:)]
        pub unsafe fn setKeyCell(&self, key_cell: Option<&NSCell>);
    }
);

extern_protocol!(
    pub unsafe trait NSMatrixDelegate: NSControlTextEditingDelegate {}

    unsafe impl ProtocolType for dyn NSMatrixDelegate {}
);
