//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsgridcellplacement?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSGridCellPlacement(pub NSInteger);
impl NSGridCellPlacement {
    #[doc(alias = "NSGridCellPlacementInherited")]
    pub const Inherited: Self = Self(0);
    #[doc(alias = "NSGridCellPlacementNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "NSGridCellPlacementLeading")]
    pub const Leading: Self = Self(2);
    #[doc(alias = "NSGridCellPlacementTop")]
    pub const Top: Self = Self(NSGridCellPlacement::Leading.0);
    #[doc(alias = "NSGridCellPlacementTrailing")]
    pub const Trailing: Self = Self(3);
    #[doc(alias = "NSGridCellPlacementBottom")]
    pub const Bottom: Self = Self(NSGridCellPlacement::Trailing.0);
    #[doc(alias = "NSGridCellPlacementCenter")]
    pub const Center: Self = Self(4);
    #[doc(alias = "NSGridCellPlacementFill")]
    pub const Fill: Self = Self(5);
}

unsafe impl Encode for NSGridCellPlacement {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSGridCellPlacement {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsgridrowalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSGridRowAlignment(pub NSInteger);
impl NSGridRowAlignment {
    #[doc(alias = "NSGridRowAlignmentInherited")]
    pub const Inherited: Self = Self(0);
    #[doc(alias = "NSGridRowAlignmentNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "NSGridRowAlignmentFirstBaseline")]
    pub const FirstBaseline: Self = Self(2);
    #[doc(alias = "NSGridRowAlignmentLastBaseline")]
    pub const LastBaseline: Self = Self(3);
}

unsafe impl Encode for NSGridRowAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSGridRowAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsgridviewsizeforcontent?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSGridViewSizeForContent: CGFloat;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsgridview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSGridView;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSGridView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSGridView {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSGridView {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSGridView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSGridView {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSGridView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSGridView {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSGridView {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSGridView {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other gridViewWithNumberOfColumns:rows:)]
        pub unsafe fn gridViewWithNumberOfColumns_rows(
            column_count: NSInteger,
            row_count: NSInteger,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other gridViewWithViews:)]
        pub unsafe fn gridViewWithViews(
            rows: &NSArray<NSArray<NSView>>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method(numberOfRows)]
        pub unsafe fn numberOfRows(&self) -> NSInteger;

        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other rowAtIndex:)]
        pub unsafe fn rowAtIndex(&self, index: NSInteger) -> Retained<NSGridRow>;

        #[method(indexOfRow:)]
        pub unsafe fn indexOfRow(&self, row: &NSGridRow) -> NSInteger;

        #[method_id(@__retain_semantics Other columnAtIndex:)]
        pub unsafe fn columnAtIndex(&self, index: NSInteger) -> Retained<NSGridColumn>;

        #[method(indexOfColumn:)]
        pub unsafe fn indexOfColumn(&self, column: &NSGridColumn) -> NSInteger;

        #[method_id(@__retain_semantics Other cellAtColumnIndex:rowIndex:)]
        pub unsafe fn cellAtColumnIndex_rowIndex(
            &self,
            column_index: NSInteger,
            row_index: NSInteger,
        ) -> Retained<NSGridCell>;

        #[method_id(@__retain_semantics Other cellForView:)]
        pub unsafe fn cellForView(&self, view: &NSView) -> Option<Retained<NSGridCell>>;

        #[method_id(@__retain_semantics Other addRowWithViews:)]
        pub unsafe fn addRowWithViews(&self, views: &NSArray<NSView>) -> Retained<NSGridRow>;

        #[method_id(@__retain_semantics Other insertRowAtIndex:withViews:)]
        pub unsafe fn insertRowAtIndex_withViews(
            &self,
            index: NSInteger,
            views: &NSArray<NSView>,
        ) -> Retained<NSGridRow>;

        #[method(moveRowAtIndex:toIndex:)]
        pub unsafe fn moveRowAtIndex_toIndex(&self, from_index: NSInteger, to_index: NSInteger);

        #[method(removeRowAtIndex:)]
        pub unsafe fn removeRowAtIndex(&self, index: NSInteger);

        #[method_id(@__retain_semantics Other addColumnWithViews:)]
        pub unsafe fn addColumnWithViews(&self, views: &NSArray<NSView>) -> Retained<NSGridColumn>;

        #[method_id(@__retain_semantics Other insertColumnAtIndex:withViews:)]
        pub unsafe fn insertColumnAtIndex_withViews(
            &self,
            index: NSInteger,
            views: &NSArray<NSView>,
        ) -> Retained<NSGridColumn>;

        #[method(moveColumnAtIndex:toIndex:)]
        pub unsafe fn moveColumnAtIndex_toIndex(&self, from_index: NSInteger, to_index: NSInteger);

        #[method(removeColumnAtIndex:)]
        pub unsafe fn removeColumnAtIndex(&self, index: NSInteger);

        #[method(xPlacement)]
        pub unsafe fn xPlacement(&self) -> NSGridCellPlacement;

        #[method(setXPlacement:)]
        pub unsafe fn setXPlacement(&self, x_placement: NSGridCellPlacement);

        #[method(yPlacement)]
        pub unsafe fn yPlacement(&self) -> NSGridCellPlacement;

        #[method(setYPlacement:)]
        pub unsafe fn setYPlacement(&self, y_placement: NSGridCellPlacement);

        #[method(rowAlignment)]
        pub unsafe fn rowAlignment(&self) -> NSGridRowAlignment;

        #[method(setRowAlignment:)]
        pub unsafe fn setRowAlignment(&self, row_alignment: NSGridRowAlignment);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rowSpacing)]
        pub unsafe fn rowSpacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setRowSpacing:)]
        pub unsafe fn setRowSpacing(&self, row_spacing: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(columnSpacing)]
        pub unsafe fn columnSpacing(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setColumnSpacing:)]
        pub unsafe fn setColumnSpacing(&self, column_spacing: CGFloat);

        #[method(mergeCellsInHorizontalRange:verticalRange:)]
        pub unsafe fn mergeCellsInHorizontalRange_verticalRange(
            &self,
            h_range: NSRange,
            v_range: NSRange,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSGridView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSGridView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsgridrow?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSGridRow;
);

unsafe impl NSCoding for NSGridRow {}

unsafe impl NSObjectProtocol for NSGridRow {}

extern_methods!(
    unsafe impl NSGridRow {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other gridView)]
        pub unsafe fn gridView(&self) -> Option<Retained<NSGridView>>;

        #[method(numberOfCells)]
        pub unsafe fn numberOfCells(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other cellAtIndex:)]
        pub unsafe fn cellAtIndex(&self, index: NSInteger) -> Retained<NSGridCell>;

        #[method(yPlacement)]
        pub unsafe fn yPlacement(&self) -> NSGridCellPlacement;

        #[method(setYPlacement:)]
        pub unsafe fn setYPlacement(&self, y_placement: NSGridCellPlacement);

        #[method(rowAlignment)]
        pub unsafe fn rowAlignment(&self) -> NSGridRowAlignment;

        #[method(setRowAlignment:)]
        pub unsafe fn setRowAlignment(&self, row_alignment: NSGridRowAlignment);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(height)]
        pub unsafe fn height(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(topPadding)]
        pub unsafe fn topPadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setTopPadding:)]
        pub unsafe fn setTopPadding(&self, top_padding: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bottomPadding)]
        pub unsafe fn bottomPadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setBottomPadding:)]
        pub unsafe fn setBottomPadding(&self, bottom_padding: CGFloat);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method(mergeCellsInRange:)]
        pub unsafe fn mergeCellsInRange(&self, range: NSRange);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSGridRow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsgridcolumn?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSGridColumn;
);

unsafe impl NSCoding for NSGridColumn {}

unsafe impl NSObjectProtocol for NSGridColumn {}

extern_methods!(
    unsafe impl NSGridColumn {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other gridView)]
        pub unsafe fn gridView(&self) -> Option<Retained<NSGridView>>;

        #[method(numberOfCells)]
        pub unsafe fn numberOfCells(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other cellAtIndex:)]
        pub unsafe fn cellAtIndex(&self, index: NSInteger) -> Retained<NSGridCell>;

        #[method(xPlacement)]
        pub unsafe fn xPlacement(&self) -> NSGridCellPlacement;

        #[method(setXPlacement:)]
        pub unsafe fn setXPlacement(&self, x_placement: NSGridCellPlacement);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(width)]
        pub unsafe fn width(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(leadingPadding)]
        pub unsafe fn leadingPadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setLeadingPadding:)]
        pub unsafe fn setLeadingPadding(&self, leading_padding: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(trailingPadding)]
        pub unsafe fn trailingPadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setTrailingPadding:)]
        pub unsafe fn setTrailingPadding(&self, trailing_padding: CGFloat);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method(mergeCellsInRange:)]
        pub unsafe fn mergeCellsInRange(&self, range: NSRange);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSGridColumn {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsgridcell?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSGridCell;
);

unsafe impl NSCoding for NSGridCell {}

unsafe impl NSObjectProtocol for NSGridCell {}

extern_methods!(
    unsafe impl NSGridCell {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, content_view: Option<&NSView>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other emptyContentView)]
        pub unsafe fn emptyContentView(mtm: MainThreadMarker) -> Retained<NSView>;

        #[method_id(@__retain_semantics Other row)]
        pub unsafe fn row(&self) -> Option<Retained<NSGridRow>>;

        #[method_id(@__retain_semantics Other column)]
        pub unsafe fn column(&self) -> Option<Retained<NSGridColumn>>;

        #[method(xPlacement)]
        pub unsafe fn xPlacement(&self) -> NSGridCellPlacement;

        #[method(setXPlacement:)]
        pub unsafe fn setXPlacement(&self, x_placement: NSGridCellPlacement);

        #[method(yPlacement)]
        pub unsafe fn yPlacement(&self) -> NSGridCellPlacement;

        #[method(setYPlacement:)]
        pub unsafe fn setYPlacement(&self, y_placement: NSGridCellPlacement);

        #[method(rowAlignment)]
        pub unsafe fn rowAlignment(&self) -> NSGridRowAlignment;

        #[method(setRowAlignment:)]
        pub unsafe fn setRowAlignment(&self, row_alignment: NSGridRowAlignment);

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other customPlacementConstraints)]
        pub unsafe fn customPlacementConstraints(&self) -> Retained<NSArray<NSLayoutConstraint>>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method(setCustomPlacementConstraints:)]
        pub unsafe fn setCustomPlacementConstraints(
            &self,
            custom_placement_constraints: &NSArray<NSLayoutConstraint>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSGridCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
