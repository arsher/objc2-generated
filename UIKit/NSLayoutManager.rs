//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstextlayoutorientation?language=objc)
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

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nsglyphproperty?language=objc)
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

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nscontrolcharacteraction?language=objc)
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
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstextlayoutorientationprovider?language=objc)
    pub unsafe trait NSTextLayoutOrientationProvider {
        #[method(layoutOrientation)]
        unsafe fn layoutOrientation(&self) -> NSTextLayoutOrientation;
    }

    unsafe impl ProtocolType for dyn NSTextLayoutOrientationProvider {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nslayoutmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLayoutManager;
);

unsafe impl NSCoding for NSLayoutManager {}

unsafe impl NSObjectProtocol for NSLayoutManager {}

unsafe impl NSSecureCoding for NSLayoutManager {}

extern_methods!(
    unsafe impl NSLayoutManager {
        /// ************************** Initialization ***************************
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSTextStorage")]
        /// ************************* Text storage **************************
        #[method_id(@__retain_semantics Other textStorage)]
        pub unsafe fn textStorage(&self) -> Option<Retained<NSTextStorage>>;

        #[cfg(feature = "NSTextStorage")]
        /// Setter for [`textStorage`][Self::textStorage].
        #[method(setTextStorage:)]
        pub unsafe fn setTextStorage(&self, text_storage: Option<&NSTextStorage>);

        #[cfg(feature = "NSTextContainer")]
        /// ************************** Text containers ***************************
        #[method_id(@__retain_semantics Other textContainers)]
        pub unsafe fn textContainers(&self) -> Retained<NSArray<NSTextContainer>>;

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

        /// ************************** Delegate ***************************
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSLayoutManagerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSLayoutManagerDelegate>>,
        );

        /// ********************* Global layout manager options **********************
        #[method(showsInvisibleCharacters)]
        pub unsafe fn showsInvisibleCharacters(&self) -> bool;

        /// Setter for [`showsInvisibleCharacters`][Self::showsInvisibleCharacters].
        #[method(setShowsInvisibleCharacters:)]
        pub unsafe fn setShowsInvisibleCharacters(&self, shows_invisible_characters: bool);

        #[method(showsControlCharacters)]
        pub unsafe fn showsControlCharacters(&self) -> bool;

        /// Setter for [`showsControlCharacters`][Self::showsControlCharacters].
        #[method(setShowsControlCharacters:)]
        pub unsafe fn setShowsControlCharacters(&self, shows_control_characters: bool);

        #[method(usesFontLeading)]
        pub unsafe fn usesFontLeading(&self) -> bool;

        /// Setter for [`usesFontLeading`][Self::usesFontLeading].
        #[method(setUsesFontLeading:)]
        pub unsafe fn setUsesFontLeading(&self, uses_font_leading: bool);

        #[method(allowsNonContiguousLayout)]
        pub unsafe fn allowsNonContiguousLayout(&self) -> bool;

        /// Setter for [`allowsNonContiguousLayout`][Self::allowsNonContiguousLayout].
        #[method(setAllowsNonContiguousLayout:)]
        pub unsafe fn setAllowsNonContiguousLayout(&self, allows_non_contiguous_layout: bool);

        #[method(hasNonContiguousLayout)]
        pub unsafe fn hasNonContiguousLayout(&self) -> bool;

        #[method(limitsLayoutForSuspiciousContents)]
        pub unsafe fn limitsLayoutForSuspiciousContents(&self) -> bool;

        /// Setter for [`limitsLayoutForSuspiciousContents`][Self::limitsLayoutForSuspiciousContents].
        #[method(setLimitsLayoutForSuspiciousContents:)]
        pub unsafe fn setLimitsLayoutForSuspiciousContents(
            &self,
            limits_layout_for_suspicious_contents: bool,
        );

        #[method(usesDefaultHyphenation)]
        pub unsafe fn usesDefaultHyphenation(&self) -> bool;

        /// Setter for [`usesDefaultHyphenation`][Self::usesDefaultHyphenation].
        #[method(setUsesDefaultHyphenation:)]
        pub unsafe fn setUsesDefaultHyphenation(&self, uses_default_hyphenation: bool);

        /// ************************ Invalidation *************************
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

        /// ********************** Causing glyph generation and layout ***********************
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

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
        #[method(ensureLayoutForBoundingRect:inTextContainer:)]
        pub unsafe fn ensureLayoutForBoundingRect_inTextContainer(
            &self,
            bounds: CGRect,
            container: &NSTextContainer,
        );

        #[cfg(all(feature = "UIFont", feature = "objc2-core-graphics"))]
        /// ********************** Set glyphs and glyph properties ***********************
        #[method(setGlyphs:properties:characterIndexes:font:forGlyphRange:)]
        pub unsafe fn setGlyphs_properties_characterIndexes_font_forGlyphRange(
            &self,
            glyphs: NonNull<CGGlyph>,
            props: NonNull<NSGlyphProperty>,
            char_indexes: NonNull<NSUInteger>,
            a_font: &UIFont,
            glyph_range: NSRange,
        );

        /// ********************** Get glyphs and glyph properties ***********************
        #[method(numberOfGlyphs)]
        pub unsafe fn numberOfGlyphs(&self) -> NSUInteger;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(CGGlyphAtIndex:isValidIndex:)]
        pub unsafe fn CGGlyphAtIndex_isValidIndex(
            &self,
            glyph_index: NSUInteger,
            is_valid_index: *mut Bool,
        ) -> CGGlyph;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(CGGlyphAtIndex:)]
        pub unsafe fn CGGlyphAtIndex(&self, glyph_index: NSUInteger) -> CGGlyph;

        #[method(isValidGlyphIndex:)]
        pub unsafe fn isValidGlyphIndex(&self, glyph_index: NSUInteger) -> bool;

        #[method(propertyForGlyphAtIndex:)]
        pub unsafe fn propertyForGlyphAtIndex(&self, glyph_index: NSUInteger) -> NSGlyphProperty;

        #[method(characterIndexForGlyphAtIndex:)]
        pub unsafe fn characterIndexForGlyphAtIndex(&self, glyph_index: NSUInteger) -> NSUInteger;

        #[method(glyphIndexForCharacterAtIndex:)]
        pub unsafe fn glyphIndexForCharacterAtIndex(&self, char_index: NSUInteger) -> NSUInteger;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(getGlyphsInRange:glyphs:properties:characterIndexes:bidiLevels:)]
        pub unsafe fn getGlyphsInRange_glyphs_properties_characterIndexes_bidiLevels(
            &self,
            glyph_range: NSRange,
            glyph_buffer: *mut CGGlyph,
            props: *mut NSGlyphProperty,
            char_index_buffer: *mut NSUInteger,
            bidi_level_buffer: *mut c_uchar,
        ) -> NSUInteger;

        #[cfg(feature = "NSTextContainer")]
        #[method(setTextContainer:forGlyphRange:)]
        pub unsafe fn setTextContainer_forGlyphRange(
            &self,
            container: &NSTextContainer,
            glyph_range: NSRange,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setLineFragmentRect:forGlyphRange:usedRect:)]
        pub unsafe fn setLineFragmentRect_forGlyphRange_usedRect(
            &self,
            fragment_rect: CGRect,
            glyph_range: NSRange,
            used_rect: CGRect,
        );

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
        #[method(setExtraLineFragmentRect:usedRect:textContainer:)]
        pub unsafe fn setExtraLineFragmentRect_usedRect_textContainer(
            &self,
            fragment_rect: CGRect,
            used_rect: CGRect,
            container: &NSTextContainer,
        );

        #[cfg(feature = "objc2-core-foundation")]
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

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setAttachmentSize:forGlyphRange:)]
        pub unsafe fn setAttachmentSize_forGlyphRange(
            &self,
            attachment_size: CGSize,
            glyph_range: NSRange,
        );

        /// ********************** Get layout information ***********************
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
        ) -> Option<Retained<NSTextContainer>>;

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Other textContainerForGlyphAtIndex:effectiveRange:withoutAdditionalLayout:)]
        pub unsafe fn textContainerForGlyphAtIndex_effectiveRange_withoutAdditionalLayout(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
            flag: bool,
        ) -> Option<Retained<NSTextContainer>>;

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
        #[method(usedRectForTextContainer:)]
        pub unsafe fn usedRectForTextContainer(&self, container: &NSTextContainer) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(lineFragmentRectForGlyphAtIndex:effectiveRange:)]
        pub unsafe fn lineFragmentRectForGlyphAtIndex_effectiveRange(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
        ) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(lineFragmentRectForGlyphAtIndex:effectiveRange:withoutAdditionalLayout:)]
        pub unsafe fn lineFragmentRectForGlyphAtIndex_effectiveRange_withoutAdditionalLayout(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
            flag: bool,
        ) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(lineFragmentUsedRectForGlyphAtIndex:effectiveRange:)]
        pub unsafe fn lineFragmentUsedRectForGlyphAtIndex_effectiveRange(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
        ) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(lineFragmentUsedRectForGlyphAtIndex:effectiveRange:withoutAdditionalLayout:)]
        pub unsafe fn lineFragmentUsedRectForGlyphAtIndex_effectiveRange_withoutAdditionalLayout(
            &self,
            glyph_index: NSUInteger,
            effective_glyph_range: NSRangePointer,
            flag: bool,
        ) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(extraLineFragmentRect)]
        pub unsafe fn extraLineFragmentRect(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(extraLineFragmentUsedRect)]
        pub unsafe fn extraLineFragmentUsedRect(&self) -> CGRect;

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Other extraLineFragmentTextContainer)]
        pub unsafe fn extraLineFragmentTextContainer(&self) -> Option<Retained<NSTextContainer>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(locationForGlyphAtIndex:)]
        pub unsafe fn locationForGlyphAtIndex(&self, glyph_index: NSUInteger) -> CGPoint;

        #[method(notShownAttributeForGlyphAtIndex:)]
        pub unsafe fn notShownAttributeForGlyphAtIndex(&self, glyph_index: NSUInteger) -> bool;

        #[method(drawsOutsideLineFragmentForGlyphAtIndex:)]
        pub unsafe fn drawsOutsideLineFragmentForGlyphAtIndex(
            &self,
            glyph_index: NSUInteger,
        ) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(attachmentSizeForGlyphAtIndex:)]
        pub unsafe fn attachmentSizeForGlyphAtIndex(&self, glyph_index: NSUInteger) -> CGSize;

        #[method(truncatedGlyphRangeInLineFragmentForGlyphAtIndex:)]
        pub unsafe fn truncatedGlyphRangeInLineFragmentForGlyphAtIndex(
            &self,
            glyph_index: NSUInteger,
        ) -> NSRange;

        /// ********************** More sophisticated queries ***********************
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

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
        #[method(boundingRectForGlyphRange:inTextContainer:)]
        pub unsafe fn boundingRectForGlyphRange_inTextContainer(
            &self,
            glyph_range: NSRange,
            container: &NSTextContainer,
        ) -> CGRect;

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
        #[method(glyphRangeForBoundingRect:inTextContainer:)]
        pub unsafe fn glyphRangeForBoundingRect_inTextContainer(
            &self,
            bounds: CGRect,
            container: &NSTextContainer,
        ) -> NSRange;

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
        #[method(glyphRangeForBoundingRectWithoutAdditionalLayout:inTextContainer:)]
        pub unsafe fn glyphRangeForBoundingRectWithoutAdditionalLayout_inTextContainer(
            &self,
            bounds: CGRect,
            container: &NSTextContainer,
        ) -> NSRange;

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
        #[method(glyphIndexForPoint:inTextContainer:fractionOfDistanceThroughGlyph:)]
        pub unsafe fn glyphIndexForPoint_inTextContainer_fractionOfDistanceThroughGlyph(
            &self,
            point: CGPoint,
            container: &NSTextContainer,
            partial_fraction: *mut CGFloat,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
        #[method(glyphIndexForPoint:inTextContainer:)]
        pub unsafe fn glyphIndexForPoint_inTextContainer(
            &self,
            point: CGPoint,
            container: &NSTextContainer,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
        #[method(fractionOfDistanceThroughGlyphForPoint:inTextContainer:)]
        pub unsafe fn fractionOfDistanceThroughGlyphForPoint_inTextContainer(
            &self,
            point: CGPoint,
            container: &NSTextContainer,
        ) -> CGFloat;

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
        #[method(characterIndexForPoint:inTextContainer:fractionOfDistanceBetweenInsertionPoints:)]
        pub unsafe fn characterIndexForPoint_inTextContainer_fractionOfDistanceBetweenInsertionPoints(
            &self,
            point: CGPoint,
            container: &NSTextContainer,
            partial_fraction: *mut CGFloat,
        ) -> NSUInteger;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(getLineFragmentInsertionPointsForCharacterAtIndex:alternatePositions:inDisplayOrder:positions:characterIndexes:)]
        pub unsafe fn getLineFragmentInsertionPointsForCharacterAtIndex_alternatePositions_inDisplayOrder_positions_characterIndexes(
            &self,
            char_index: NSUInteger,
            a_flag: bool,
            d_flag: bool,
            positions: *mut CGFloat,
            char_indexes: *mut NSUInteger,
        ) -> NSUInteger;

        #[cfg(all(
            feature = "NSTextContainer",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        #[method(enumerateLineFragmentsForGlyphRange:usingBlock:)]
        pub unsafe fn enumerateLineFragmentsForGlyphRange_usingBlock(
            &self,
            glyph_range: NSRange,
            block: &block2::Block<
                dyn Fn(CGRect, CGRect, NonNull<NSTextContainer>, NSRange, NonNull<Bool>),
            >,
        );

        #[cfg(all(
            feature = "NSTextContainer",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        #[method(enumerateEnclosingRectsForGlyphRange:withinSelectedGlyphRange:inTextContainer:usingBlock:)]
        pub unsafe fn enumerateEnclosingRectsForGlyphRange_withinSelectedGlyphRange_inTextContainer_usingBlock(
            &self,
            glyph_range: NSRange,
            selected_range: NSRange,
            text_container: &NSTextContainer,
            block: &block2::Block<dyn Fn(CGRect, NonNull<Bool>)>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        /// ********************** Drawing support ***********************
        #[method(drawBackgroundForGlyphRange:atPoint:)]
        pub unsafe fn drawBackgroundForGlyphRange_atPoint(
            &self,
            glyphs_to_show: NSRange,
            origin: CGPoint,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawGlyphsForGlyphRange:atPoint:)]
        pub unsafe fn drawGlyphsForGlyphRange_atPoint(
            &self,
            glyphs_to_show: NSRange,
            origin: CGPoint,
        );

        #[cfg(all(
            feature = "UIFont",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics"
        ))]
        #[method(showCGGlyphs:positions:count:font:textMatrix:attributes:inContext:)]
        pub unsafe fn showCGGlyphs_positions_count_font_textMatrix_attributes_inContext(
            &self,
            glyphs: NonNull<CGGlyph>,
            positions: NonNull<CGPoint>,
            glyph_count: NSInteger,
            font: &UIFont,
            text_matrix: CGAffineTransform,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            cg_context: CGContextRef,
        );

        #[cfg(all(feature = "UIColor", feature = "objc2-core-foundation"))]
        #[method(fillBackgroundRectArray:count:forCharacterRange:color:)]
        pub unsafe fn fillBackgroundRectArray_count_forCharacterRange_color(
            &self,
            rect_array: NonNull<CGRect>,
            rect_count: NSUInteger,
            char_range: NSRange,
            color: &UIColor,
        );

        #[cfg(all(feature = "NSAttributedString", feature = "objc2-core-foundation"))]
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

        #[cfg(all(feature = "NSAttributedString", feature = "objc2-core-foundation"))]
        #[method(underlineGlyphRange:underlineType:lineFragmentRect:lineFragmentGlyphRange:containerOrigin:)]
        pub unsafe fn underlineGlyphRange_underlineType_lineFragmentRect_lineFragmentGlyphRange_containerOrigin(
            &self,
            glyph_range: NSRange,
            underline_val: NSUnderlineStyle,
            line_rect: CGRect,
            line_glyph_range: NSRange,
            container_origin: CGPoint,
        );

        #[cfg(all(feature = "NSAttributedString", feature = "objc2-core-foundation"))]
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

        #[cfg(all(feature = "NSAttributedString", feature = "objc2-core-foundation"))]
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
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nslayoutmanagerdelegate?language=objc)
    pub unsafe trait NSLayoutManagerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "UIFont", feature = "objc2-core-graphics"))]
        /// ********************** Glyph generation ***********************
        #[optional]
        #[method(layoutManager:shouldGenerateGlyphs:properties:characterIndexes:font:forGlyphRange:)]
        unsafe fn layoutManager_shouldGenerateGlyphs_properties_characterIndexes_font_forGlyphRange(
            &self,
            layout_manager: &NSLayoutManager,
            glyphs: NonNull<CGGlyph>,
            props: NonNull<NSGlyphProperty>,
            char_indexes: NonNull<NSUInteger>,
            a_font: &UIFont,
            glyph_range: NSRange,
        ) -> NSUInteger;

        #[cfg(feature = "objc2-core-foundation")]
        /// ********************** Line layout ***********************
        #[optional]
        #[method(layoutManager:lineSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        unsafe fn layoutManager_lineSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            layout_manager: &NSLayoutManager,
            glyph_index: NSUInteger,
            rect: CGRect,
        ) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[optional]
        #[method(layoutManager:paragraphSpacingBeforeGlyphAtIndex:withProposedLineFragmentRect:)]
        unsafe fn layoutManager_paragraphSpacingBeforeGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            layout_manager: &NSLayoutManager,
            glyph_index: NSUInteger,
            rect: CGRect,
        ) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
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

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
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

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
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

        /// ********************** Layout processing ***********************
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

        #[cfg(all(feature = "NSTextContainer", feature = "objc2-core-foundation"))]
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
        #[cfg(feature = "objc2-core-graphics")]
        #[method(glyphAtIndex:isValidIndex:)]
        pub unsafe fn glyphAtIndex_isValidIndex(
            &self,
            glyph_index: NSUInteger,
            is_valid_index: *mut Bool,
        ) -> CGGlyph;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(glyphAtIndex:)]
        pub unsafe fn glyphAtIndex(&self, glyph_index: NSUInteger) -> CGGlyph;

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "Please use usesDefaultHyphenation or -[NSParagraphStyle hyphenationFactor] instead."]
        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`hyphenationFactor`][Self::hyphenationFactor].
        #[deprecated = "Please use usesDefaultHyphenation or -[NSParagraphStyle hyphenationFactor] instead."]
        #[method(setHyphenationFactor:)]
        pub unsafe fn setHyphenationFactor(&self, hyphenation_factor: CGFloat);

        #[cfg(all(
            feature = "UIFont",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics"
        ))]
        #[deprecated]
        #[method(showCGGlyphs:positions:count:font:matrix:attributes:inContext:)]
        pub unsafe fn showCGGlyphs_positions_count_font_matrix_attributes_inContext(
            &self,
            glyphs: NonNull<CGGlyph>,
            positions: NonNull<CGPoint>,
            glyph_count: NSUInteger,
            font: &UIFont,
            text_matrix: CGAffineTransform,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            graphics_context: CGContextRef,
        );
    }
);
