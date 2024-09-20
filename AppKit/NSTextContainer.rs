//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextContainer;

    unsafe impl ClassType for NSTextContainer {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for NSTextContainer {}

unsafe impl NSObjectProtocol for NSTextContainer {}

unsafe impl NSSecureCoding for NSTextContainer {}

extern_methods!(
    unsafe impl NSTextContainer {
        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(this: Allocated<Self>, size: CGSize) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Retained<NSTextLayoutManager>>;

        #[method(size)]
        pub unsafe fn size(&self) -> CGSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: CGSize);

        #[cfg(feature = "NSParagraphStyle")]
        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[method(lineFragmentPadding)]
        pub unsafe fn lineFragmentPadding(&self) -> CGFloat;

        #[method(setLineFragmentPadding:)]
        pub unsafe fn setLineFragmentPadding(&self, line_fragment_padding: CGFloat);

        #[method(maximumNumberOfLines)]
        pub unsafe fn maximumNumberOfLines(&self) -> NSUInteger;

        #[method(setMaximumNumberOfLines:)]
        pub unsafe fn setMaximumNumberOfLines(&self, maximum_number_of_lines: NSUInteger);

        #[cfg(feature = "NSText")]
        #[method(lineFragmentRectForProposedRect:atIndex:writingDirection:remainingRect:)]
        pub unsafe fn lineFragmentRectForProposedRect_atIndex_writingDirection_remainingRect(
            &self,
            proposed_rect: CGRect,
            character_index: NSUInteger,
            base_writing_direction: NSWritingDirection,
            remaining_rect: *mut CGRect,
        ) -> CGRect;

        #[method(isSimpleRectangularTextContainer)]
        pub unsafe fn isSimpleRectangularTextContainer(&self) -> bool;

        #[method(widthTracksTextView)]
        pub unsafe fn widthTracksTextView(&self) -> bool;

        #[method(setWidthTracksTextView:)]
        pub unsafe fn setWidthTracksTextView(&self, width_tracks_text_view: bool);

        #[method(heightTracksTextView)]
        pub unsafe fn heightTracksTextView(&self) -> bool;

        #[method(setHeightTracksTextView:)]
        pub unsafe fn setHeightTracksTextView(&self, height_tracks_text_view: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextContainer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl NSTextContainer {
        #[cfg(feature = "NSLayoutManager")]
        #[method_id(@__retain_semantics Other layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Retained<NSLayoutManager>>;

        #[cfg(feature = "NSLayoutManager")]
        #[method(setLayoutManager:)]
        pub unsafe fn setLayoutManager(&self, layout_manager: Option<&NSLayoutManager>);

        #[cfg(feature = "NSLayoutManager")]
        #[method(replaceLayoutManager:)]
        pub unsafe fn replaceLayoutManager(&self, new_layout_manager: &NSLayoutManager);

        #[cfg(feature = "NSBezierPath")]
        #[method_id(@__retain_semantics Other exclusionPaths)]
        pub unsafe fn exclusionPaths(&self) -> Retained<NSArray<NSBezierPath>>;

        #[cfg(feature = "NSBezierPath")]
        #[method(setExclusionPaths:)]
        pub unsafe fn setExclusionPaths(&self, exclusion_paths: &NSArray<NSBezierPath>);

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSText",
            feature = "NSTextView",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other textView)]
        pub unsafe fn textView(&self, mtm: MainThreadMarker) -> Option<Retained<NSTextView>>;

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSText",
            feature = "NSTextView",
            feature = "NSView"
        ))]
        #[method(setTextView:)]
        pub unsafe fn setTextView(&self, text_view: Option<&NSTextView>);
    }
);

#[cfg(feature = "NSLayoutManager")]
unsafe impl NSTextLayoutOrientationProvider for NSTextContainer {}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLineSweepDirection(pub NSUInteger);
impl NSLineSweepDirection {
    pub const NSLineSweepLeft: Self = Self(0);
    pub const NSLineSweepRight: Self = Self(1);
    pub const NSLineSweepDown: Self = Self(2);
    pub const NSLineSweepUp: Self = Self(3);
}

unsafe impl Encode for NSLineSweepDirection {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSLineSweepDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLineMovementDirection(pub NSUInteger);
impl NSLineMovementDirection {
    pub const NSLineDoesntMove: Self = Self(0);
    pub const NSLineMovesLeft: Self = Self(1);
    pub const NSLineMovesRight: Self = Self(2);
    pub const NSLineMovesDown: Self = Self(3);
    pub const NSLineMovesUp: Self = Self(4);
}

unsafe impl Encode for NSLineMovementDirection {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSLineMovementDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSTextContainerDeprecated
    unsafe impl NSTextContainer {
        #[method_id(@__retain_semantics Init initWithContainerSize:)]
        pub unsafe fn initWithContainerSize(
            this: Allocated<Self>,
            a_container_size: NSSize,
        ) -> Retained<Self>;

        #[method(containerSize)]
        pub unsafe fn containerSize(&self) -> NSSize;

        #[method(setContainerSize:)]
        pub unsafe fn setContainerSize(&self, container_size: NSSize);

        #[method(lineFragmentRectForProposedRect:sweepDirection:movementDirection:remainingRect:)]
        pub unsafe fn lineFragmentRectForProposedRect_sweepDirection_movementDirection_remainingRect(
            &self,
            proposed_rect: NSRect,
            sweep_direction: NSLineSweepDirection,
            movement_direction: NSLineMovementDirection,
            remaining_rect: NSRectPointer,
        ) -> NSRect;

        #[deprecated]
        #[method(containsPoint:)]
        pub unsafe fn containsPoint(&self, point: NSPoint) -> bool;
    }
);
