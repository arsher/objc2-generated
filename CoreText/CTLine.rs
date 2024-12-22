//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctlineref?language=objc)
pub type CTLineRef = *const c_void;

/// Options for CTLineGetBoundsWithOptions.
///
///
/// Passing 0 (no options) returns the typographic bounds,
/// including typographic leading and shifts.
///
///
/// Pass this option to exclude typographic leading.
///
///
/// Pass this option to ignore cross-stream shifts due to
/// positioning (such as kerning or baseline alignment).
///
///
/// Normally line bounds include all glyphs; pass this option to
/// treat standard punctuation hanging off either end of the line
/// as fully hanging.
///
///
/// Pass this option to use glyph path bounds rather than the
/// default typographic bounds.
///
///
/// Pass this option to use optical bounds, as determined by
/// CTFontGetOpticalBoundsForGlyphs. This option overrides
/// kCTLineBoundsUseGlyphPathBounds.
///
///
/// Pass this option to include additional space based on common
/// glyph sequences for various languages. The result is intended
/// to be used when drawing to avoid clipping that may be caused
/// by the typographic bounds. This option does not have any effect
/// when used with kCTLineBoundsUseGlyphPathBounds.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/ctlineboundsoptions?language=objc)
// NS_OPTIONS
#[cfg(feature = "objc2-core-foundation")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTLineBoundsOptions(pub CFOptionFlags);
#[cfg(feature = "objc2-core-foundation")]
bitflags::bitflags! {
    impl CTLineBoundsOptions: CFOptionFlags {
        const kCTLineBoundsExcludeTypographicLeading = 1<<0;
        const kCTLineBoundsExcludeTypographicShifts = 1<<1;
        const kCTLineBoundsUseHangingPunctuation = 1<<2;
        const kCTLineBoundsUseGlyphPathBounds = 1<<3;
        const kCTLineBoundsUseOpticalBounds = 1<<4;
        const kCTLineBoundsIncludeLanguageExtents = 1<<5;
    }
}

#[cfg(all(feature = "objc2", feature = "objc2-core-foundation"))]
unsafe impl Encode for CTLineBoundsOptions {
    const ENCODING: Encoding = CFOptionFlags::ENCODING;
}

#[cfg(all(feature = "objc2", feature = "objc2-core-foundation"))]
unsafe impl RefEncode for CTLineBoundsOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Truncation types required by CTLineCreateTruncatedLine. These
/// will tell truncation engine which type of truncation is being
/// requested.
///
///
/// Truncate at the beginning of the line, leaving the end portion
/// visible.
///
///
/// Truncate at the end of the line, leaving the start portion
/// visible.
///
///
/// Truncate in the middle of the line, leaving both the start
/// and the end portions visible.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/ctlinetruncationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTLineTruncationType(pub u32);
impl CTLineTruncationType {
    pub const kCTLineTruncationStart: Self = Self(0);
    pub const kCTLineTruncationEnd: Self = Self(1);
    pub const kCTLineTruncationMiddle: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CTLineTruncationType {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CTLineTruncationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    /// Returns the CFType of the line object
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    /// Creates a single immutable line object directly from an
    /// attributed string.
    ///
    ///
    /// This will allow clients who need very simple line generation to
    /// create a line without needing to create a typesetter object. The
    /// typesetting will be done under the hood. Without a typesetter
    /// object, the line cannot be properly broken. However, for simple
    /// things like text labels and other things, this is not an issue.
    ///
    ///
    /// Parameter `attrString`: The attributed string which the line will be created for.
    ///
    ///
    /// Returns: This function will return a reference to a CTLine object.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineCreateWithAttributedString(attr_string: CFAttributedStringRef) -> CTLineRef;
}

extern "C-unwind" {
    /// Creates a truncated line from an existing line.
    ///
    ///
    /// Parameter `line`: The line that you want to create a truncated line for.
    ///
    ///
    /// Parameter `width`: The width at which truncation will begin. The line will be
    /// truncated if its width is greater than the width passed in this.
    ///
    ///
    /// Parameter `truncationType`: The type of truncation to perform if needed.
    ///
    ///
    /// Parameter `truncationToken`: This token will be added to the point where truncation took place
    /// to indicate that the line was truncated. Usually, the truncation
    /// token is the ellipsis character (U+2026). If this parameter is
    /// set to NULL, then no truncation token is used, and the line is
    /// simply cut off. The line specified in truncationToken should have
    /// a width less than the width specified by the width parameter. If
    /// the width of the line specified in truncationToken is greater,
    /// this function will return NULL if truncation is needed.
    ///
    ///
    /// Returns: This function will return a reference to a truncated CTLine
    /// object if the call was successful. Otherwise, it will return
    /// NULL.
    pub fn CTLineCreateTruncatedLine(
        line: CTLineRef,
        width: c_double,
        truncation_type: CTLineTruncationType,
        truncation_token: CTLineRef,
    ) -> CTLineRef;
}

extern "C-unwind" {
    /// Creates a justified line from an existing line.
    ///
    ///
    /// Parameter `line`: The line that you want to create a justified line for.
    ///
    ///
    /// Parameter `justificationFactor`: Allows for full or partial justification. When set to 1.0 or
    /// greater indicates, full justification will be performed. If less
    /// than 1.0, varying degrees of partial justification will be
    /// performed. If set to 0 or less, then no justification will be
    /// performed.
    ///
    ///
    /// Parameter `justificationWidth`: The width to which the resultant line will be justified. If
    /// justificationWidth is less than the actual width of the line,
    /// then negative justification will be performed ("text squishing").
    ///
    ///
    /// Returns: This function will return a reference to a justified CTLine
    /// object if the call was successful. Otherwise, it will return
    /// NULL.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineCreateJustifiedLine(
        line: CTLineRef,
        justification_factor: CGFloat,
        justification_width: c_double,
    ) -> CTLineRef;
}

extern "C-unwind" {
    /// Returns the total glyph count for the line object.
    ///
    ///
    /// The total glyph count is equal to the sum of all of the glyphs in
    /// the glyph runs forming the line.
    ///
    ///
    /// Parameter `line`: The line that you want to obtain the glyph count for.
    ///
    ///
    /// Returns: The total glyph count for the line passed in.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetGlyphCount(line: CTLineRef) -> CFIndex;
}

extern "C-unwind" {
    /// Returns the array of glyph runs that make up the line object.
    ///
    ///
    /// Parameter `line`: The line that you want to obtain the glyph run array for.
    ///
    ///
    /// Returns: A CFArrayRef containing the CTRun objects that make up the line.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetGlyphRuns(line: CTLineRef) -> CFArrayRef;
}

extern "C-unwind" {
    /// Gets the range of characters that originally spawned the glyphs
    /// in the line.
    ///
    ///
    /// Parameter `line`: The line that you want to obtain the string range from.
    ///
    ///
    /// Returns: A CFRange that contains the range over the backing store string
    /// that spawned the glyphs. If the function fails for any reason, an
    /// empty range will be returned.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetStringRange(line: CTLineRef) -> CFRange;
}

extern "C-unwind" {
    /// Gets the pen offset required to draw flush text.
    ///
    ///
    /// Parameter `line`: The line that you want to obtain a flush position from.
    ///
    ///
    /// Parameter `flushFactor`: Specifies what kind of flushness you want. A flushFactor of 0 or
    /// less indicates left flush. A flushFactor of 1.0 or more indicates
    /// right flush. Flush factors between 0 and 1.0 indicate varying
    /// degrees of center flush, with a value of 0.5 being totally center
    /// flush.
    ///
    ///
    /// Parameter `flushWidth`: Specifies the width that the flushness operation should apply to.
    ///
    ///
    /// Returns: A value which can be used to offset the current pen position for
    /// the flush operation.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetPenOffsetForFlush(
        line: CTLineRef,
        flush_factor: CGFloat,
        flush_width: c_double,
    ) -> c_double;
}

extern "C-unwind" {
    /// Draws a line.
    ///
    ///
    /// This is a convenience call, since the line could be drawn
    /// run-by-run by getting the glyph runs and accessing the glyphs out
    /// of them. This call may leave the graphics context in any state and
    /// does not flush the context after drawing. This call also expects
    /// a text matrix with `y` values increasing from bottom to top; a
    /// flipped text matrix may result in misplaced diacritics.
    ///
    ///
    /// Parameter `line`: The line that you want to draw.
    ///
    ///
    /// Parameter `context`: The context to which the line will be drawn.
    #[cfg(feature = "objc2-core-graphics")]
    pub fn CTLineDraw(line: CTLineRef, context: CGContextRef);
}

extern "C-unwind" {
    /// Calculates the typographic bounds for a line.
    ///
    ///
    /// A line's typographic width is the distance to the rightmost
    /// glyph advance width edge. Note that this distance includes
    /// trailing whitespace glyphs.
    ///
    ///
    /// Parameter `line`: The line that you want to calculate the typographic bounds for.
    ///
    ///
    /// Parameter `ascent`: Upon return, this parameter will contain the ascent of the line.
    /// This may be set to NULL if not needed.
    ///
    ///
    /// Parameter `descent`: Upon return, this parameter will contain the descent of the line.
    /// This may be set to NULL if not needed.
    ///
    ///
    /// Parameter `leading`: Upon return, this parameter will contain the leading of the line.
    /// This may be set to NULL if not needed.
    ///
    ///
    /// Returns: The typographic width of the line. If line is invalid, this
    /// function will always return zero.
    ///
    ///
    /// See also: CTLineGetTrailingWhitespaceWidth
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetTypographicBounds(
        line: CTLineRef,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> c_double;
}

extern "C-unwind" {
    /// Calculates the bounds for a line.
    ///
    ///
    /// Parameter `line`: The line that you want to calculate the bounds for.
    ///
    ///
    /// Parameter `options`: Desired options or 0 if none.
    ///
    ///
    /// Returns: The bounds of the line as specified by the type and options,
    /// such that the coordinate origin is coincident with the line
    /// origin and the rect origin is at the bottom left. If the line
    /// is invalid this function will return CGRectNull.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetBoundsWithOptions(line: CTLineRef, options: CTLineBoundsOptions) -> CGRect;
}

extern "C-unwind" {
    /// Calculates the trailing whitespace width for a line.
    ///
    ///
    /// Parameter `line`: The line that you want to calculate the trailing whitespace width
    /// for. Creating a line for a width can result in a line that is
    /// actually longer than the desired width due to trailing
    /// whitespace. Normally this is not an issue due to whitespace being
    /// invisible, but this function may be used to determine what amount
    /// of a line's width is due to trailing whitespace.
    ///
    ///
    /// Returns: The width of the line's trailing whitespace. If line is invalid,
    /// this function will always return zero.
    pub fn CTLineGetTrailingWhitespaceWidth(line: CTLineRef) -> c_double;
}

extern "C-unwind" {
    /// Calculates the image bounds for a line.
    ///
    ///
    /// The image bounds for a line is the union of all non-empty glyph
    /// bounding rects, each positioned as it would be if drawn using
    /// CTLineDraw using the current context. Note that the result is
    /// ideal and does not account for raster coverage due to rendering.
    /// This function is purely a convenience for using glyphs as an
    /// image and should not be used for typographic purposes.
    ///
    ///
    /// Parameter `line`: The line that you want to calculate the image bounds for.
    ///
    ///
    /// Parameter `context`: The context which the image bounds will be calculated for or NULL,
    /// in which case the bounds are relative to CGPointZero.
    ///
    ///
    /// Returns: A rectangle that tightly encloses the paths of the line's glyphs,
    /// which will be translated by the supplied context's text position.
    /// If the line is invalid, CGRectNull will be returned.
    ///
    ///
    /// See also: CTLineGetTypographicBounds
    ///
    /// See also: CTLineGetBoundsWithOptions
    ///
    /// See also: CTLineGetPenOffsetForFlush
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
    pub fn CTLineGetImageBounds(line: CTLineRef, context: CGContextRef) -> CGRect;
}

extern "C-unwind" {
    /// Performs hit testing.
    ///
    ///
    /// This function can be used to determine the string index for a
    /// mouse click or other event. This string index corresponds to the
    /// character before which the next character should be inserted.
    /// This determination is made by analyzing the string from which a
    /// typesetter was created and the corresponding glyphs as embodied
    /// by a particular line.
    ///
    ///
    /// Parameter `line`: The line being examined.
    ///
    ///
    /// Parameter `position`: The location of the mouse click relative to the line's origin.
    ///
    ///
    /// Returns: The string index for the position. Relative to the line's string
    /// range, this value will be no less than the first string index and
    /// no greater than one plus the last string index. In the event of
    /// failure, this function will return kCFNotFound.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetStringIndexForPosition(line: CTLineRef, position: CGPoint) -> CFIndex;
}

extern "C-unwind" {
    /// Determines the graphical offset(s) for a string index.
    ///
    ///
    /// This function returns the graphical offset(s) corresponding to
    /// a string index, suitable for movement between adjacent lines or
    /// for drawing a custom caret. For the former, the primary offset
    /// may be adjusted for any relative indentation of the two lines;
    /// a CGPoint constructed with the adjusted offset for its x value
    /// and 0.0 for its y value is suitable for passing to
    /// CTLineGetStringIndexForPosition. In either case, the primary
    /// offset corresponds to the portion of the caret that represents
    /// the visual insertion location for a character whose direction
    /// matches the line's writing direction.
    ///
    ///
    /// Parameter `line`: The line from which the offset is requested.
    ///
    ///
    /// Parameter `charIndex`: The string index corresponding to the desired position.
    ///
    ///
    /// Parameter `secondaryOffset`: An output parameter that will be set to the secondary offset
    /// along the baseline for charIndex. When a single caret is
    /// sufficient for a string index, this value will be the same as
    /// the primary offset, which is the return value of this function.
    /// This parameter may be NULL.
    ///
    ///
    /// Returns: The primary offset along the baseline for charIndex, or 0.0 in
    /// the event of failure.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTLineGetOffsetForStringIndex(
        line: CTLineRef,
        char_index: CFIndex,
        secondary_offset: *mut CGFloat,
    ) -> CGFloat;
}

extern "C-unwind" {
    /// Enumerates caret offsets for characters in a line.
    ///
    ///
    /// The provided block is invoked once for each logical caret edge in the line, in left-to-right visual order.
    ///
    ///
    /// Parameter `block`: The offset parameter is relative to the line origin. The leadingEdge parameter of this block refers to logical order.
    #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
    pub fn CTLineEnumerateCaretOffsets(
        line: CTLineRef,
        block: &block2::Block<dyn Fn(c_double, CFIndex, bool, NonNull<bool>)>,
    );
}
