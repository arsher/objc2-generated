//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextLayoutOrientation(pub NSInteger);
impl NSTextLayoutOrientation {
    #[doc(alias = "NSTextLayoutOrientationHorizontal")]
    pub const Horizontal: Self = Self(0);
    #[doc(alias = "NSTextLayoutOrientationVertical")]
    pub const Vertical: Self = Self(1);
}

unsafe impl Encode for NSTextLayoutOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextLayoutOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSGlyphProperty(pub NSInteger);
bitflags::bitflags! {
    impl NSGlyphProperty: NSInteger {
        #[doc(alias = "NSGlyphPropertyNull")]
        const Null = 1<<0;
        #[doc(alias = "NSGlyphPropertyControlCharacter")]
        const ControlCharacter = 1<<1;
        #[doc(alias = "NSGlyphPropertyElastic")]
        const Elastic = 1<<2;
        #[doc(alias = "NSGlyphPropertyNonBaseCharacter")]
        const NonBaseCharacter = 1<<3;
    }
}

unsafe impl Encode for NSGlyphProperty {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSGlyphProperty {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSControlCharacterAction(pub NSInteger);
bitflags::bitflags! {
    impl NSControlCharacterAction: NSInteger {
        #[doc(alias = "NSControlCharacterActionZeroAdvancement")]
        const ZeroAdvancement = 1<<0;
        #[doc(alias = "NSControlCharacterActionWhitespace")]
        const Whitespace = 1<<1;
        #[doc(alias = "NSControlCharacterActionHorizontalTab")]
        const HorizontalTab = 1<<2;
        #[doc(alias = "NSControlCharacterActionLineBreak")]
        const LineBreak = 1<<3;
        #[doc(alias = "NSControlCharacterActionParagraphBreak")]
        const ParagraphBreak = 1<<4;
        #[doc(alias = "NSControlCharacterActionContainerBreak")]
        const ContainerBreak = 1<<5;
    }
}

unsafe impl Encode for NSControlCharacterAction {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSControlCharacterAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSTextLayoutOrientationProvider {
        #[method(layoutOrientation)]
        unsafe fn layoutOrientation(&self) -> NSTextLayoutOrientation;
    }

    unsafe impl ProtocolType for dyn NSTextLayoutOrientationProvider {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLayoutManager;

    unsafe impl ClassType for NSLayoutManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSLayoutManager {}

unsafe impl NSObjectProtocol for NSLayoutManager {}

unsafe impl NSSecureCoding for NSLayoutManager {}

extern_methods!(
    unsafe impl NSLayoutManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(feature = "NSTextStorage")]
        #[method_id(@__retain_semantics Other textStorage)]
        pub unsafe fn textStorage(&self) -> Option<Id<NSTextStorage>>;

        #[cfg(feature = "NSTextStorage")]
        #[method(setTextStorage:)]
        pub unsafe fn setTextStorage(&self, text_storage: Option<&NSTextStorage>);

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Other textContainers)]
        pub unsafe fn textContainers(&self) -> Id<NSArray<NSTextContainer>>;

        #[cfg(feature = "NSTextContainer")]
        #[method(addTextContainer:)]
        pub unsafe fn addTextContainer(&self, container: &NSTextContainer);

        #[cfg(feature = "NSTextContainer")]
        #[method(insertTextContainer:atIndex:)]
        pub unsafe fn insertTextContainer_atIndex(
            &self,
            container: &NSTextContainer,
            index: NSUInteger,
        );

        #[method(removeTextContainerAtIndex:)]
        pub unsafe fn removeTextContainerAtIndex(&self, index: NSUInteger);

        #[cfg(feature = "NSTextContainer")]
        #[method(textContainerChangedGeometry:)]
        pub unsafe fn textContainerChangedGeometry(&self, container: &NSTextContainer);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSLayoutManagerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSLayoutManagerDelegate>>,
        );

        #[method(showsInvisibleCharacters)]
        pub unsafe fn showsInvisibleCharacters(&self) -> bool;

        #[method(setShowsInvisibleCharacters:)]
        pub unsafe fn setShowsInvisibleCharacters(&self, shows_invisible_characters: bool);

        #[method(showsControlCharacters)]
        pub unsafe fn showsControlCharacters(&self) -> bool;

        #[method(setShowsControlCharacters:)]
        pub unsafe fn setShowsControlCharacters(&self, shows_control_characters: bool);

        #[method(usesFontLeading)]
        pub unsafe fn usesFontLeading(&self) -> bool;

        #[method(setUsesFontLeading:)]
        pub unsafe fn setUsesFontLeading(&self, uses_font_leading: bool);

        #[method(allowsNonContiguousLayout)]
        pub unsafe fn allowsNonContiguousLayout(&self) -> bool;

        #[method(setAllowsNonContiguousLayout:)]
        pub unsafe fn setAllowsNonContiguousLayout(&self, allows_non_contiguous_layout: bool);

        #[method(hasNonContiguousLayout)]
        pub unsafe fn hasNonContiguousLayout(&self) -> bool;

        #[method(limitsLayoutForSuspiciousContents)]
        pub unsafe fn limitsLayoutForSuspiciousContents(&self) -> bool;

        #[method(setLimitsLayoutForSuspiciousContents:)]
        pub unsafe fn setLimitsLayoutForSuspiciousContents(
            &self,
            limits_layout_for_suspicious_contents: bool,
        );

        #[method(usesDefaultHyphenation)]
        pub unsafe fn usesDefaultHyphenation(&self) -> bool;

        #[method(setUsesDefaultHyphenation:)]
        pub unsafe fn setUsesDefaultHyphenation(&self, uses_default_hyphenation: bool);

        #[method(invalidateGlyphsForCharacterRange:changeInLength:actualCharacterRange:)]
        pub unsafe fn invalidateGlyphsForCharacterRange_changeInLength_actualCharacterRange(
            &self,
            char_range: NSRange,
            delta: NSInteger,
            actual_char_range: NSRangePointer,
        );

        #[method(invalidateLayoutForCharacterRange:actualCharacterRange:)]
        pub unsafe fn invalidateLayoutForCharacterRange_actualCharacterRange(
            &self,
            char_range: NSRange,
            actual_char_range: NSRangePointer,
        );

        #[method(invalidateDisplayForCharacterRange:)]
        pub unsafe fn invalidateDisplayForCharacterRange(&self, char_range: NSRange);

        #[method(invalidateDisplayForGlyphRange:)]
        pub unsafe fn invalidateDisplayForGlyphRange(&self, glyph_range: NSRange);

        #[cfg(feature = "NSTextStorage")]
        #[method(processEditingForTextStorage:edited:range:changeInLength:invalidatedRange:)]
        pub unsafe fn processEditingForTextStorage_edited_range_changeInLength_invalidatedRange(
            &self,
            text_storage: &NSTextStorage,
            edit_mask: NSTextStorageEditActions,
            new_char_range: NSRange,
            delta: NSInteger,
            invalidated_char_range: NSRange,
        );

        #[method(ensureGlyphsForCharacterRange:)]
        pub unsafe fn ensureGlyphsForCharacterRange(&self, char_range: NSRange);

        #[method(ensureGlyphsForGlyphRange:)]
        pub unsafe fn ensureGlyphsForGlyphRange(&self, glyph_range: NSRange);

        #[method(ensureLayoutForCharacterRange:)]
        pub unsafe fn ensureLayoutForCharacterRange(&self, char_range: NSRange);

        #[method(ensureLayoutForGlyphRange:)]
        pub unsafe fn ensureLayoutForGlyphRange(&self, glyph_range: NSRange);

        #[cfg(feature = "NSTextContainer")]
        #[method(ensureLayoutForTextContainer:)]
        pub unsafe fn ensureLayoutForTextContainer(&self, container: &NSTextContainer);

        #[cfg(feature = "NSTextContainer")]
        #[method(ensureLayoutForBoundingRect:inTextContainer:)]
        pub unsafe fn ensureLayoutForBoundingRect_inTextContainer(
            &self,
            bounds: CGRect,
            container: &NSTextContainer,
        );

        #[method(numberOfGlyphs)]
        pub unsafe fn numberOfGlyphs(&self) -> NSUInteger;

        #[method(isValidGlyphIndex:)]
        pub unsafe fn isValidGlyphIndex(&self, glyph_index: NSUInteger) -> bool;

        #[method(propertyForGlyphAtIndex:)]
        pub unsafe fn propertyForGlyphAtIndex(&self, glyph_index: NSUInteger) -> NSGlyphProperty;

        #[method(characterIndexForGlyphAtIndex:)]
        pub unsafe fn characterIndexForGlyphAtIndex(&self, glyph_index: NSUInteger) -> NSUInteger;

        #[method(glyphIndexForCharacterAtIndex:)]
        pub unsafe fn glyphIndexForCharacterAtIndex(&self, char_index: NSUInteger) -> NSUInteger;

        #[cfg(feature = "NSTextContainer")]
        #[method(setTextContainer:forGlyphRange:)]
        pub unsafe fn setTextContainer_forGlyphRange(
            &self,
            container: &NSTextContainer,
            glyph_range: NSRange,
        );

        #[method(setLineFragmentRect:forGlyphRange:usedRect:)]
        pub unsafe fn setLineFragmentRect_forGlyphRange_usedRect(
            &self,
            fragment_rect: CGRect,
            glyph_range: NSRange,
            used_rect: CGRect,
        );

        #[cfg(feature = "NSTextContainer")]
        #[method(setExtraLineFragmentRect:usedRect:textContainer:)]
        pub unsafe fn setExtraLineFragmentRect_usedRect_textContainer(
            &self,
            fragment_rect: CGRect,
            used_rect: CGRect,
            container: &NSTextContainer,
        );

        #[method(setLocation:forStartOfGlyphRange:)]
        pub unsafe fn setLocation_forStartOfGlyphRange(
            &self,
            location: CGPoint,
            glyph_range: NSRange,
        );

        #[method(setNotShownAttribute:forGlyphAtIndex:)]
        pub unsafe fn setNotShownAttribute_forGlyphAtIndex(
            &self,
            flag: bool,
            glyph_index: NSUInteger,
        );

        #[method(setDrawsOutsideLineFragment:forGlyphAtIndex:)]
        pub unsafe fn setDrawsOutsideLineFragment_forGlyphAtIndex(
            &self,
            flag: bool,
            glyph_index: NSUInteger,
        );

        #[method(setAttachmentSize:forGlyphRange:)]
        pub unsafe fn setAttachmentSize_forGlyphRange(
            &self,
            attachment_size: CGSize,
            glyph_range: NSRange,
        );

        #[method(getFirstUnlaidCharacterIndex:glyphIndex:)]
        pub unsafe fn getFirstUnlaidCharacterIndex_glyphIndex(
            &self,
            char_index: *mut NSUInteger,
            glyph_index: *mut NSUInteger,
        );

        #[method(firstUnlaidCharacterIndex)]
        pub unsafe fn firstUnlaidCharacterIndex(&self) -> NSUInteger;

        #[method(firstUnlaidGlyphIndex)]
        pub unsafe fn firstUnlaidGlyphIndex(&self) -> NSUInteger;

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Other textContainerForGlyphAtIndex:effectiveRange:)]
        pub unsafe fn textContainerForGlyphAtIndex_effectiveRange(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
        ) -> Option<Id<NSTextContainer>>;

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Other textContainerForGlyphAtIndex:effectiveRange:withoutAdditionalLayout:)]
        pub unsafe fn textContainerForGlyphAtIndex_effectiveRange_withoutAdditionalLayout(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
            flag: bool,
        ) -> Option<Id<NSTextContainer>>;

        #[cfg(feature = "NSTextContainer")]
        #[method(usedRectForTextContainer:)]
        pub unsafe fn usedRectForTextContainer(&self, container: &NSTextContainer) -> CGRect;

        #[method(lineFragmentRectForGlyphAtIndex:effectiveRange:)]
        pub unsafe fn lineFragmentRectForGlyphAtIndex_effectiveRange(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
        ) -> CGRect;

        #[method(lineFragmentRectForGlyphAtIndex:effectiveRange:withoutAdditionalLayout:)]
        pub unsafe fn lineFragmentRectForGlyphAtIndex_effectiveRange_withoutAdditionalLayout(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
            flag: bool,
        ) -> CGRect;

        #[method(lineFragmentUsedRectForGlyphAtIndex:effectiveRange:)]
        pub unsafe fn lineFragmentUsedRectForGlyphAtIndex_effectiveRange(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
        ) -> CGRect;

        #[method(lineFragmentUsedRectForGlyphAtIndex:effectiveRange:withoutAdditionalLayout:)]
        pub unsafe fn lineFragmentUsedRectForGlyphAtIndex_effectiveRange_withoutAdditionalLayout(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
            flag: bool,
        ) -> CGRect;

        #[method(extraLineFragmentRect)]
        pub unsafe fn extraLineFragmentRect(&self) -> CGRect;

        #[method(extraLineFragmentUsedRect)]
        pub unsafe fn extraLineFragmentUsedRect(&self) -> CGRect;

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Other extraLineFragmentTextContainer)]
        pub unsafe fn extraLineFragmentTextContainer(&self) -> Option<Id<NSTextContainer>>;

        #[method(locationForGlyphAtIndex:)]
        pub unsafe fn locationForGlyphAtIndex(&self, glyph_index: NSUInteger) -> CGPoint;

        #[method(notShownAttributeForGlyphAtIndex:)]
        pub unsafe fn notShownAttributeForGlyphAtIndex(&self, glyph_index: NSUInteger) -> bool;

        #[method(drawsOutsideLineFragmentForGlyphAtIndex:)]
        pub unsafe fn drawsOutsideLineFragmentForGlyphAtIndex(
            &self,
            glyph_index: NSUInteger,
        ) -> bool;

        #[method(attachmentSizeForGlyphAtIndex:)]
        pub unsafe fn attachmentSizeForGlyphAtIndex(&self, glyph_index: NSUInteger) -> CGSize;

        #[method(truncatedGlyphRangeInLineFragmentForGlyphAtIndex:)]
        pub unsafe fn truncatedGlyphRangeInLineFragmentForGlyphAtIndex(
            &self,
            glyph_index: NSUInteger,
        ) -> NSRange;

        #[method(glyphRangeForCharacterRange:actualCharacterRange:)]
        pub unsafe fn glyphRangeForCharacterRange_actualCharacterRange(
            &self,
            char_range: NSRange,
            actual_char_range: NSRangePointer,
        ) -> NSRange;

        #[method(characterRangeForGlyphRange:actualGlyphRange:)]
        pub unsafe fn characterRangeForGlyphRange_actualGlyphRange(
            &self,
            glyph_range: NSRange,
            actual_glyph_range: NSRangePointer,
        ) -> NSRange;

        #[cfg(feature = "NSTextContainer")]
        #[method(glyphRangeForTextContainer:)]
        pub unsafe fn glyphRangeForTextContainer(&self, container: &NSTextContainer) -> NSRange;

        #[method(rangeOfNominallySpacedGlyphsContainingIndex:)]
        pub unsafe fn rangeOfNominallySpacedGlyphsContainingIndex(
            &self,
            glyph_index: NSUInteger,
        ) -> NSRange;

        #[cfg(feature = "NSTextContainer")]
        #[method(boundingRectForGlyphRange:inTextContainer:)]
        pub unsafe fn boundingRectForGlyphRange_inTextContainer(
            &self,
            glyph_range: NSRange,
            container: &NSTextContainer,
        ) -> CGRect;

        #[cfg(feature = "NSTextContainer")]
        #[method(glyphRangeForBoundingRect:inTextContainer:)]
        pub unsafe fn glyphRangeForBoundingRect_inTextContainer(
            &self,
            bounds: CGRect,
            container: &NSTextContainer,
        ) -> NSRange;

        #[cfg(feature = "NSTextContainer")]
        #[method(glyphRangeForBoundingRectWithoutAdditionalLayout:inTextContainer:)]
        pub unsafe fn glyphRangeForBoundingRectWithoutAdditionalLayout_inTextContainer(
            &self,
            bounds: CGRect,
            container: &NSTextContainer,
        ) -> NSRange;

        #[cfg(feature = "NSTextContainer")]
        #[method(glyphIndexForPoint:inTextContainer:)]
        pub unsafe fn glyphIndexForPoint_inTextContainer(
            &self,
            point: CGPoint,
            container: &NSTextContainer,
        ) -> NSUInteger;

        #[cfg(feature = "NSTextContainer")]
        #[method(fractionOfDistanceThroughGlyphForPoint:inTextContainer:)]
        pub unsafe fn fractionOfDistanceThroughGlyphForPoint_inTextContainer(
            &self,
            point: CGPoint,
            container: &NSTextContainer,
        ) -> CGFloat;

        #[cfg(feature = "NSTextContainer")]
        #[method(characterIndexForPoint:inTextContainer:fractionOfDistanceBetweenInsertionPoints:)]
        pub unsafe fn characterIndexForPoint_inTextContainer_fractionOfDistanceBetweenInsertionPoints(
            &self,
            point: CGPoint,
            container: &NSTextContainer,
            partial_fraction: *mut CGFloat,
        ) -> NSUInteger;

        #[method(getLineFragmentInsertionPointsForCharacterAtIndex:alternatePositions:inDisplayOrder:positions:characterIndexes:)]
        pub unsafe fn getLineFragmentInsertionPointsForCharacterAtIndex_alternatePositions_inDisplayOrder_positions_characterIndexes(
            &self,
            char_index: NSUInteger,
            a_flag: bool,
            d_flag: bool,
            positions: *mut CGFloat,
            char_indexes: *mut NSUInteger,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSTextContainer", feature = "block2"))]
        #[method(enumerateLineFragmentsForGlyphRange:usingBlock:)]
        pub unsafe fn enumerateLineFragmentsForGlyphRange_usingBlock(
            &self,
            glyph_range: NSRange,
            block: &block2::Block<
                dyn Fn(CGRect, CGRect, NonNull<NSTextContainer>, NSRange, NonNull<Bool>),
            >,
        );

        #[cfg(all(feature = "NSTextContainer", feature = "block2"))]
        #[method(enumerateEnclosingRectsForGlyphRange:withinSelectedGlyphRange:inTextContainer:usingBlock:)]
        pub unsafe fn enumerateEnclosingRectsForGlyphRange_withinSelectedGlyphRange_inTextContainer_usingBlock(
            &self,
            glyph_range: NSRange,
            selected_range: NSRange,
            text_container: &NSTextContainer,
            block: &block2::Block<dyn Fn(CGRect, NonNull<Bool>)>,
        );

        #[method(drawBackgroundForGlyphRange:atPoint:)]
        pub unsafe fn drawBackgroundForGlyphRange_atPoint(
            &self,
            glyphs_to_show: NSRange,
            origin: CGPoint,
        );

        #[method(drawGlyphsForGlyphRange:atPoint:)]
        pub unsafe fn drawGlyphsForGlyphRange_atPoint(
            &self,
            glyphs_to_show: NSRange,
            origin: CGPoint,
        );

        #[cfg(feature = "UIColor")]
        #[method(fillBackgroundRectArray:count:forCharacterRange:color:)]
        pub unsafe fn fillBackgroundRectArray_count_forCharacterRange_color(
            &self,
            rect_array: NonNull<CGRect>,
            rect_count: NSUInteger,
            char_range: NSRange,
            color: &UIColor,
        );

        #[cfg(feature = "NSAttributedString")]
        #[method(drawUnderlineForGlyphRange:underlineType:baselineOffset:lineFragmentRect:lineFragmentGlyphRange:containerOrigin:)]
        pub unsafe fn drawUnderlineForGlyphRange_underlineType_baselineOffset_lineFragmentRect_lineFragmentGlyphRange_containerOrigin(
            &self,
            glyph_range: NSRange,
            underline_val: NSUnderlineStyle,
            baseline_offset: CGFloat,
            line_rect: CGRect,
            line_glyph_range: NSRange,
            container_origin: CGPoint,
        );

        #[cfg(feature = "NSAttributedString")]
        #[method(underlineGlyphRange:underlineType:lineFragmentRect:lineFragmentGlyphRange:containerOrigin:)]
        pub unsafe fn underlineGlyphRange_underlineType_lineFragmentRect_lineFragmentGlyphRange_containerOrigin(
            &self,
            glyph_range: NSRange,
            underline_val: NSUnderlineStyle,
            line_rect: CGRect,
            line_glyph_range: NSRange,
            container_origin: CGPoint,
        );

        #[cfg(feature = "NSAttributedString")]
        #[method(drawStrikethroughForGlyphRange:strikethroughType:baselineOffset:lineFragmentRect:lineFragmentGlyphRange:containerOrigin:)]
        pub unsafe fn drawStrikethroughForGlyphRange_strikethroughType_baselineOffset_lineFragmentRect_lineFragmentGlyphRange_containerOrigin(
            &self,
            glyph_range: NSRange,
            strikethrough_val: NSUnderlineStyle,
            baseline_offset: CGFloat,
            line_rect: CGRect,
            line_glyph_range: NSRange,
            container_origin: CGPoint,
        );

        #[cfg(feature = "NSAttributedString")]
        #[method(strikethroughGlyphRange:strikethroughType:lineFragmentRect:lineFragmentGlyphRange:containerOrigin:)]
        pub unsafe fn strikethroughGlyphRange_strikethroughType_lineFragmentRect_lineFragmentGlyphRange_containerOrigin(
            &self,
            glyph_range: NSRange,
            strikethrough_val: NSUnderlineStyle,
            line_rect: CGRect,
            line_glyph_range: NSRange,
            container_origin: CGPoint,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLayoutManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSLayoutManagerDelegate: NSObjectProtocol {
        #[optional]
        #[method(layoutManager:lineSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        unsafe fn layoutManager_lineSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            layout_manager: &NSLayoutManager,
            glyph_index: NSUInteger,
            rect: CGRect,
        ) -> CGFloat;

        #[optional]
        #[method(layoutManager:paragraphSpacingBeforeGlyphAtIndex:withProposedLineFragmentRect:)]
        unsafe fn layoutManager_paragraphSpacingBeforeGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            layout_manager: &NSLayoutManager,
            glyph_index: NSUInteger,
            rect: CGRect,
        ) -> CGFloat;

        #[optional]
        #[method(layoutManager:paragraphSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        unsafe fn layoutManager_paragraphSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            layout_manager: &NSLayoutManager,
            glyph_index: NSUInteger,
            rect: CGRect,
        ) -> CGFloat;

        #[optional]
        #[method(layoutManager:shouldUseAction:forControlCharacterAtIndex:)]
        unsafe fn layoutManager_shouldUseAction_forControlCharacterAtIndex(
            &self,
            layout_manager: &NSLayoutManager,
            action: NSControlCharacterAction,
            char_index: NSUInteger,
        ) -> NSControlCharacterAction;

        #[optional]
        #[method(layoutManager:shouldBreakLineByWordBeforeCharacterAtIndex:)]
        unsafe fn layoutManager_shouldBreakLineByWordBeforeCharacterAtIndex(
            &self,
            layout_manager: &NSLayoutManager,
            char_index: NSUInteger,
        ) -> bool;

        #[optional]
        #[method(layoutManager:shouldBreakLineByHyphenatingBeforeCharacterAtIndex:)]
        unsafe fn layoutManager_shouldBreakLineByHyphenatingBeforeCharacterAtIndex(
            &self,
            layout_manager: &NSLayoutManager,
            char_index: NSUInteger,
        ) -> bool;

        #[cfg(feature = "NSTextContainer")]
        #[optional]
        #[method(layoutManager:boundingBoxForControlGlyphAtIndex:forTextContainer:proposedLineFragment:glyphPosition:characterIndex:)]
        unsafe fn layoutManager_boundingBoxForControlGlyphAtIndex_forTextContainer_proposedLineFragment_glyphPosition_characterIndex(
            &self,
            layout_manager: &NSLayoutManager,
            glyph_index: NSUInteger,
            text_container: &NSTextContainer,
            proposed_rect: CGRect,
            glyph_position: CGPoint,
            char_index: NSUInteger,
        ) -> CGRect;

        #[cfg(feature = "NSTextContainer")]
        #[optional]
        #[method(layoutManager:shouldSetLineFragmentRect:lineFragmentUsedRect:baselineOffset:inTextContainer:forGlyphRange:)]
        unsafe fn layoutManager_shouldSetLineFragmentRect_lineFragmentUsedRect_baselineOffset_inTextContainer_forGlyphRange(
            &self,
            layout_manager: &NSLayoutManager,
            line_fragment_rect: NonNull<CGRect>,
            line_fragment_used_rect: NonNull<CGRect>,
            baseline_offset: NonNull<CGFloat>,
            text_container: &NSTextContainer,
            glyph_range: NSRange,
        ) -> bool;

        #[optional]
        #[method(layoutManagerDidInvalidateLayout:)]
        unsafe fn layoutManagerDidInvalidateLayout(&self, sender: &NSLayoutManager);

        #[cfg(feature = "NSTextContainer")]
        #[optional]
        #[method(layoutManager:didCompleteLayoutForTextContainer:atEnd:)]
        unsafe fn layoutManager_didCompleteLayoutForTextContainer_atEnd(
            &self,
            layout_manager: &NSLayoutManager,
            text_container: Option<&NSTextContainer>,
            layout_finished_flag: bool,
        );

        #[cfg(feature = "NSTextContainer")]
        #[optional]
        #[method(layoutManager:textContainer:didChangeGeometryFromSize:)]
        unsafe fn layoutManager_textContainer_didChangeGeometryFromSize(
            &self,
            layout_manager: &NSLayoutManager,
            text_container: &NSTextContainer,
            old_size: CGSize,
        );
    }

    unsafe impl ProtocolType for dyn NSLayoutManagerDelegate {}
);

extern_methods!(
    /// NSLayoutManagerDeprecated
    unsafe impl NSLayoutManager {
        #[deprecated = "Please use usesDefaultHyphenation or -[NSParagraphStyle hyphenationFactor] instead."]
        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> CGFloat;

        #[deprecated = "Please use usesDefaultHyphenation or -[NSParagraphStyle hyphenationFactor] instead."]
        #[method(setHyphenationFactor:)]
        pub unsafe fn setHyphenationFactor(&self, hyphenation_factor: CGFloat);
    }
);
