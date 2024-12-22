//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// The position along the perimeter of a rectangular frame (its edges and corners) from which it’s resized.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nscursorframeresizeposition?language=objc)
// NS_CLOSED_ENUM
#[repr(usize)] // NSUInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NSCursorFrameResizePosition {
    /// The top edge of the frame.
    #[doc(alias = "NSCursorFrameResizePositionTop")]
    Top = 1 << 0,
    /// The left edge of the frame.
    #[doc(alias = "NSCursorFrameResizePositionLeft")]
    Left = 1 << 1,
    /// The bottom edge of the frame.
    #[doc(alias = "NSCursorFrameResizePositionBottom")]
    Bottom = 1 << 2,
    /// The right edge of the frame.
    #[doc(alias = "NSCursorFrameResizePositionRight")]
    Right = 1 << 3,
    /// The top left corner of the frame.
    #[doc(alias = "NSCursorFrameResizePositionTopLeft")]
    TopLeft =
        NSCursorFrameResizePosition::Top as usize | NSCursorFrameResizePosition::Left as usize,
    /// The top right corner of the frame.
    #[doc(alias = "NSCursorFrameResizePositionTopRight")]
    TopRight =
        NSCursorFrameResizePosition::Top as usize | NSCursorFrameResizePosition::Right as usize,
    /// The bottom left corner of the frame.
    #[doc(alias = "NSCursorFrameResizePositionBottomLeft")]
    BottomLeft =
        NSCursorFrameResizePosition::Bottom as usize | NSCursorFrameResizePosition::Left as usize,
    /// The bottom right corner of the frame.
    #[doc(alias = "NSCursorFrameResizePositionBottomRight")]
    BottomRight =
        NSCursorFrameResizePosition::Bottom as usize | NSCursorFrameResizePosition::Right as usize,
}

unsafe impl Encode for NSCursorFrameResizePosition {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCursorFrameResizePosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// The directions in which a rectangular frame can be resized.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nscursorframeresizedirections?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCursorFrameResizeDirections(pub NSUInteger);
bitflags::bitflags! {
    impl NSCursorFrameResizeDirections: NSUInteger {
/// Indicates that the shape can be resized inwards to be smaller.
        #[doc(alias = "NSCursorFrameResizeDirectionsInward")]
        const Inward = 1<<0;
/// Indicates that the shape can be resized outwards to be larger.
        #[doc(alias = "NSCursorFrameResizeDirectionsOutward")]
        const Outward = 1<<1;
/// Indicates that the shape can be resized inwards or wards to be either smaller or larger, respectively.
        #[doc(alias = "NSCursorFrameResizeDirectionsAll")]
        const All = NSCursorFrameResizeDirections::Inward.0|NSCursorFrameResizeDirections::Outward.0;
    }
}

unsafe impl Encode for NSCursorFrameResizeDirections {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCursorFrameResizeDirections {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscursor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCursor;
);

unsafe impl NSCoding for NSCursor {}

unsafe impl NSObjectProtocol for NSCursor {}

unsafe impl NSSecureCoding for NSCursor {}

extern_methods!(
    unsafe impl NSCursor {
        #[cfg(all(feature = "NSImage", feature = "objc2-core-foundation"))]
        #[method_id(@__retain_semantics Init initWithImage:hotSpot:)]
        pub fn initWithImage_hotSpot(
            this: Allocated<Self>,
            new_image: &NSImage,
            point: NSPoint,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Retained<NSImage>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(hotSpot)]
        pub unsafe fn hotSpot(&self) -> NSPoint;

        #[method(hide)]
        pub unsafe fn hide();

        #[method(unhide)]
        pub unsafe fn unhide();

        #[method(setHiddenUntilMouseMoves:)]
        pub unsafe fn setHiddenUntilMouseMoves(flag: bool);

        #[method(pop)]
        pub unsafe fn pop_class();

        #[method(pop)]
        pub unsafe fn pop(&self);

        #[method(push)]
        pub unsafe fn push(&self);

        #[method(set)]
        pub unsafe fn set(&self);

        /// Returns the application’s current cursor.
        /// - Note: This isn’t necessarily the cursor that is currently being displayed, as the system may be showing the cursor for another running application.
        #[method_id(@__retain_semantics Other currentCursor)]
        pub unsafe fn currentCursor() -> Retained<NSCursor>;

        /// Returns the default cursor, the arrow cursor.
        /// - Discussion: The default cursor, a slanted arrow with its hot spot at the tip. The arrow cursor is the one you’re used to seeing over buttons, scrollers, and many other objects in the window system.
        #[method_id(@__retain_semantics Other arrowCursor)]
        pub fn arrowCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other crosshairCursor)]
        pub fn crosshairCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other disappearingItemCursor)]
        pub fn disappearingItemCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other operationNotAllowedCursor)]
        pub fn operationNotAllowedCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other dragLinkCursor)]
        pub fn dragLinkCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other dragCopyCursor)]
        pub fn dragCopyCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other contextualMenuCursor)]
        pub fn contextualMenuCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other pointingHandCursor)]
        pub fn pointingHandCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other closedHandCursor)]
        pub fn closedHandCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other openHandCursor)]
        pub fn openHandCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other IBeamCursor)]
        pub fn IBeamCursor() -> Retained<NSCursor>;

        #[method_id(@__retain_semantics Other IBeamCursorForVerticalLayout)]
        pub fn IBeamCursorForVerticalLayout() -> Retained<NSCursor>;

        /// Returns the zoom-in cursor.
        /// - Note: This cursor is used to indicate zooming in on (magnifying) a canvas or object.
        #[method_id(@__retain_semantics Other zoomInCursor)]
        pub unsafe fn zoomInCursor() -> Retained<NSCursor>;

        /// Returns the zoom-out cursor.
        /// - Note: This cursor is used to indicate zooming out of a canvas or object.
        #[method_id(@__retain_semantics Other zoomOutCursor)]
        pub unsafe fn zoomOutCursor() -> Retained<NSCursor>;

        /// Returns the cursor for resizing a column (vertical divider) in either direction.
        #[method_id(@__retain_semantics Other columnResizeCursor)]
        pub unsafe fn columnResizeCursor() -> Retained<NSCursor>;

        #[cfg(feature = "NSDirection")]
        /// Returns the cursor for resizing a column (vertical divider) in the specified directions.
        /// - Parameter directions: The direction in which a column can be resized.
        #[method_id(@__retain_semantics Other columnResizeCursorInDirections:)]
        pub unsafe fn columnResizeCursorInDirections(
            directions: NSHorizontalDirections,
        ) -> Retained<NSCursor>;

        /// Returns the cursor for resizing a row (horizontal divider) in either direction.
        #[method_id(@__retain_semantics Other rowResizeCursor)]
        pub unsafe fn rowResizeCursor() -> Retained<NSCursor>;

        #[cfg(feature = "NSDirection")]
        /// Returns the cursor for resizing a row (horizontal divider) in the specified directions.
        /// - Parameter directions: The direction in which a row can be resized.
        #[method_id(@__retain_semantics Other rowResizeCursorInDirections:)]
        pub unsafe fn rowResizeCursorInDirections(
            directions: NSVerticalDirections,
        ) -> Retained<NSCursor>;

        /// Returns the cursor for resizing a rectangular frame from the specified edge or corner.
        /// - Parameters:
        /// - position: The position along the perimeter of a rectangular frame (its edges and corners) from which it’s resized.
        /// - directions: The directions in which a rectangular frame can be resized.
        #[method_id(@__retain_semantics Other frameResizeCursorFromPosition:inDirections:)]
        pub unsafe fn frameResizeCursorFromPosition_inDirections(
            position: NSCursorFrameResizePosition,
            directions: NSCursorFrameResizeDirections,
        ) -> Retained<NSCursor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCursor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappkitversionnumberwithcursorsizesupport?language=objc)
#[cfg(feature = "NSApplication")]
pub static NSAppKitVersionNumberWithCursorSizeSupport: NSAppKitVersion = 682.0 as _;

extern_methods!(
    /// Deprecated
    unsafe impl NSCursor {
        /// This property will always be `nil` in a future version of macOS.
        #[deprecated = "No longer recommended. Use ScreenCaptureKit to capture the screen. Use the `showsCursor` property on `SCStreamConfiguration` to control whether or not to include the cursor in the capture. Or, use `NSCursor.currentCursor` if needing to just get the current cursor for this application."]
        #[method_id(@__retain_semantics Other currentSystemCursor)]
        pub unsafe fn currentSystemCursor() -> Option<Retained<NSCursor>>;

        #[deprecated = "Use either `+[NSCursor columnResizeCursorInDirections:]` or `+[NSCursor frameResizeCursorFromPosition:inDirections:]` instead, depending on whether a divider is being re-positioned or a rectangular frame is being resized."]
        #[method_id(@__retain_semantics Other resizeLeftCursor)]
        pub fn resizeLeftCursor() -> Retained<NSCursor>;

        #[deprecated = "Use either `+[NSCursor columnResizeCursorInDirections:]` or `+[NSCursor frameResizeCursorFromPosition:inDirections:]` instead, depending on whether a divider is being re-positioned or a rectangular frame is being resized."]
        #[method_id(@__retain_semantics Other resizeRightCursor)]
        pub fn resizeRightCursor() -> Retained<NSCursor>;

        #[deprecated = "Use either `+[NSCursor columnResizeCursorInDirections:]` or `+[NSCursor frameResizeCursorFromPosition:inDirections:]` instead, depending on whether a divider is being re-positioned or a rectangular frame is being resized."]
        #[method_id(@__retain_semantics Other resizeLeftRightCursor)]
        pub fn resizeLeftRightCursor() -> Retained<NSCursor>;

        #[deprecated = "Use either `+[NSCursor rowResizeCursorInDirections:]` or `+[NSCursor frameResizeCursorFromPosition:inDirections:]` instead, depending on whether a divider is being re-positioned or a rectangular frame is being resized."]
        #[method_id(@__retain_semantics Other resizeUpCursor)]
        pub fn resizeUpCursor() -> Retained<NSCursor>;

        #[deprecated = "Use either `+[NSCursor rowResizeCursorInDirections:]` or `+[NSCursor frameResizeCursorFromPosition:inDirections:]` instead, depending on whether a divider is being re-positioned or a rectangular frame is being resized."]
        #[method_id(@__retain_semantics Other resizeDownCursor)]
        pub fn resizeDownCursor() -> Retained<NSCursor>;

        #[deprecated = "Use either `+[NSCursor rowResizeCursorInDirections:]` or `+[NSCursor frameResizeCursorFromPosition:inDirections:]` instead, depending on whether a divider is being re-positioned or a rectangular frame is being resized."]
        #[method_id(@__retain_semantics Other resizeUpDownCursor)]
        pub fn resizeUpDownCursor() -> Retained<NSCursor>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSCursor {
        #[cfg(all(
            feature = "NSColor",
            feature = "NSImage",
            feature = "objc2-core-foundation"
        ))]
        #[deprecated = "Color hints are ignored. Use -initWithImage:hotSpot: instead"]
        #[method_id(@__retain_semantics Init initWithImage:foregroundColorHint:backgroundColorHint:hotSpot:)]
        pub unsafe fn initWithImage_foregroundColorHint_backgroundColorHint_hotSpot(
            this: Allocated<Self>,
            new_image: &NSImage,
            fg: Option<&NSColor>,
            bg: Option<&NSColor>,
            hot_spot: NSPoint,
        ) -> Retained<Self>;

        #[deprecated = "setOnMouseExited is unused and should not be called"]
        #[method(setOnMouseExited:)]
        pub unsafe fn setOnMouseExited(&self, flag: bool);

        #[deprecated = "setOnMouseEntered is unused and should not be called"]
        #[method(setOnMouseEntered:)]
        pub unsafe fn setOnMouseEntered(&self, flag: bool);

        #[deprecated = "isSetOnMouseExited is unused"]
        #[method(isSetOnMouseExited)]
        pub unsafe fn isSetOnMouseExited(&self) -> bool;

        #[deprecated = "isSetOnMouseEntered is unused"]
        #[method(isSetOnMouseEntered)]
        pub unsafe fn isSetOnMouseEntered(&self) -> bool;

        #[cfg(feature = "NSEvent")]
        #[deprecated = "mouseEntered: is unused and should not be called"]
        #[method(mouseEntered:)]
        pub unsafe fn mouseEntered(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[deprecated = "mouseExited: is unused and should not be called"]
        #[method(mouseExited:)]
        pub unsafe fn mouseExited(&self, event: &NSEvent);
    }
);
