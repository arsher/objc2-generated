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
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSTextContainer {}

unsafe impl NSObjectProtocol for NSTextContainer {}

unsafe impl NSSecureCoding for NSTextContainer {}

#[cfg(feature = "NSLayoutManager")]
unsafe impl NSTextLayoutOrientationProvider for NSTextContainer {}

extern_methods!(
    unsafe impl NSTextContainer {
        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(this: Allocated<Self>, size: CGSize) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "NSLayoutManager")]
        #[method_id(@__retain_semantics Other layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Id<NSLayoutManager>>;

        #[cfg(feature = "NSLayoutManager")]
        #[method(setLayoutManager:)]
        pub unsafe fn setLayoutManager(&self, layout_manager: Option<&NSLayoutManager>);

        #[cfg(feature = "NSLayoutManager")]
        #[method(replaceLayoutManager:)]
        pub unsafe fn replaceLayoutManager(&self, new_layout_manager: &NSLayoutManager);

        #[cfg(feature = "NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager>>;

        #[method(size)]
        pub unsafe fn size(&self) -> CGSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: CGSize);

        #[cfg(feature = "UIBezierPath")]
        #[method_id(@__retain_semantics Other exclusionPaths)]
        pub unsafe fn exclusionPaths(&self) -> Id<NSArray<UIBezierPath>>;

        #[cfg(feature = "UIBezierPath")]
        #[method(setExclusionPaths:)]
        pub unsafe fn setExclusionPaths(&self, exclusion_paths: &NSArray<UIBezierPath>);

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
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);