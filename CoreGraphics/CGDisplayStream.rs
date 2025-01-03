//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-io-surface")]
#[cfg(not(target_os = "watchos"))]
use objc2_io_surface::*;

use crate::*;

/// An opaque reference to a CGDisplayStream object
///
/// A CGDisplayStream provides a streaming API for capturing display updates in a realtime manner.  It can also provide
/// scaling and color space conversion services, as well as allow capturing sub regions of the display.   Callbacks can be targetted
/// at either a traditional CFRunLoop, or at a dispatch queue.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplaystreamref?language=objc)
pub type CGDisplayStreamRef = *mut c_void;

/// An opaque reference to a single frame's extra metadata that describes useful frame delta information
///
/// A CGDisplayStreamUpdate encapsulates information about what portions of a frame have changed relative to
/// a previously delivered frame.   This includes regions that were changed in any way, which ones were actually redrawn, and which
/// regions were merely copied from one place to another.   A routine is provided to merge two update refs together in cases
/// where apps need to coalesce the values because they decided to skip processing for one or more frames.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplaystreamupdateref?language=objc)
pub type CGDisplayStreamUpdateRef = *const c_void;

/// Used to select which array of rectangles to be returned by CGDisplayUpdateGetRects
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplaystreamupdaterecttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGDisplayStreamUpdateRectType(pub i32);
impl CGDisplayStreamUpdateRectType {
    pub const kCGDisplayStreamUpdateRefreshedRects: Self = Self(0);
    pub const kCGDisplayStreamUpdateMovedRects: Self = Self(1);
    pub const kCGDisplayStreamUpdateDirtyRects: Self = Self(2);
    pub const kCGDisplayStreamUpdateReducedDirtyRects: Self = Self(3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGDisplayStreamUpdateRectType {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGDisplayStreamUpdateRectType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Provides information about incoming frame updates
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplaystreamframestatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGDisplayStreamFrameStatus(pub i32);
impl CGDisplayStreamFrameStatus {
    pub const kCGDisplayStreamFrameStatusFrameComplete: Self = Self(0);
    pub const kCGDisplayStreamFrameStatusFrameIdle: Self = Self(1);
    pub const kCGDisplayStreamFrameStatusFrameBlank: Self = Self(2);
    pub const kCGDisplayStreamFrameStatusStopped: Self = Self(3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGDisplayStreamFrameStatus {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGDisplayStreamFrameStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplaystreamframeavailablehandler?language=objc)
#[cfg(all(feature = "block2", feature = "objc2-io-surface"))]
#[cfg(not(target_os = "watchos"))]
pub type CGDisplayStreamFrameAvailableHandler = *mut block2::Block<
    dyn Fn(CGDisplayStreamFrameStatus, u64, IOSurfaceRef, CGDisplayStreamUpdateRef),
>;

extern "C-unwind" {
    /// Returns the CF "class" ID for CGDisplayStreamUpdate
    ///
    /// Returns: The CFTypeID of the CGDisplayStreamUpdate class.
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Please use ScreenCaptureKit instead."]
    pub fn CGDisplayStreamUpdateGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    /// Returns a pointer to an array of CGRect structs that describe what parts of the frame have changed relative
    /// to the previously delivered frame.   This rectangle list encapsulates both the update rectangles and movement rectangles.
    ///
    /// Parameter `updateRef`: The CGDisplayStreamUpdateRef
    ///
    /// Parameter `rectCount`: A pointer to where the count of the number of rectangles in the array is to be returned. Must not be NULL.
    ///
    /// Returns: A pointer to the array of CGRectangles.  This array should not be freed by the caller.
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Please use ScreenCaptureKit instead."]
    pub fn CGDisplayStreamUpdateGetRects(
        update_ref: CGDisplayStreamUpdateRef,
        rect_type: CGDisplayStreamUpdateRectType,
        rect_count: NonNull<usize>,
    ) -> *const CGRect;
}

extern "C-unwind" {
    /// Merge two CGDisplayUpdateRefs into a new one.
    ///
    /// In cases where the client wishes to drop certain frame updates, this function may be used to merge two
    /// CGDisplayUpdateRefs together.  The core bit of functionality here is generating a new set of refresh/move/dirty
    /// rectangle arrays that properly represent the union of the deltas between the two frames.  Note that the ordering of
    /// the two refs is important.
    ///
    ///
    /// Parameter `firstUpdate`: The first update (in a temporal sense)
    ///
    /// Parameter `secondUpdate`: The second update (in a temporal sense)
    ///
    /// Returns: The new CGDisplayStreamUpdateRef
    #[deprecated = "Please use ScreenCaptureKit instead."]
    pub fn CGDisplayStreamUpdateCreateMergedUpdate(
        first_update: CGDisplayStreamUpdateRef,
        second_update: CGDisplayStreamUpdateRef,
    ) -> CGDisplayStreamUpdateRef;
}

extern "C-unwind" {
    /// Return the movement dx and dy values for a single update
    ///
    /// Parameter `updateRef`: The CGDisplayStreamUpdateRef
    ///
    /// Parameter `dx`: A pointer to a CGFloat to store the x component of the movement delta
    ///
    /// Parameter `dy`: A pointer to a CGFloat to store the y component of the movement delta
    ///
    /// The delta values describe the offset from the moved rectangles back to the source location.
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Please use ScreenCaptureKit instead."]
    pub fn CGDisplayStreamUpdateGetMovedRectsDelta(
        update_ref: CGDisplayStreamUpdateRef,
        dx: NonNull<CGFloat>,
        dy: NonNull<CGFloat>,
    );
}

extern "C-unwind" {
    /// Return how many frames (if any) have been dropped since the last call to the handler.
    ///
    /// Parameter `updateRef`: The CGDisplayStreamUpdateRef
    ///
    /// Returns: The number of dropped frames
    ///
    /// This call is primarily useful for performance measurement to determine if the client is keeping up with
    /// all WindowServer updates.
    #[deprecated = "Please use ScreenCaptureKit instead."]
    pub fn CGDisplayStreamUpdateGetDropCount(update_ref: CGDisplayStreamUpdateRef) -> usize;
}

extern "C" {
    /// This may be used to request a subregion of the display to be provided as the source of the display stream.  Use
    /// CGRectCreateDictionaryRepresentation to convert from a CGRect to the value used here.   Note: The coordinate system for the
    /// source rectangle is specified in display logical coordinates and not in pixels, in order to match the normal convention on
    /// HiDPI displays.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreamsourcerect?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamSourceRect: CFStringRef;
}

extern "C" {
    /// This may be used to request where within the destination buffer the display updates should be placed. Use
    /// CGRectCreateDictionaryRepresentation to convert from a CGRect to the value used here.   Note: The coordinate system for
    /// the destination rectangle is always specified in output pixels to match the fact that the output buffer size is also
    /// specified in terms of pixels.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreamdestinationrect?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamDestinationRect: CFStringRef;
}

extern "C" {
    /// Enable/disable the work the Window Server will do to preserve the display aspect ratio.  By default the Window Server will
    /// assume that it should preserve the original aspect ratio of the source display rect.  If the aspect ratio of the source display and
    /// the display stream destination rect are not the same, black borders will be inserted at the top/bottom or right/left sides of the destination
    /// in order to preserve the source aspect ratio.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreampreserveaspectratio?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamPreserveAspectRatio: CFStringRef;
}

extern "C" {
    /// Set the desired CGColorSpace of the output frames.  By default the color space will be that of the display.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreamcolorspace?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamColorSpace: CFStringRef;
}

extern "C" {
    /// Request that the delta between frame updates be at least as much specified by this value.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreamminimumframetime?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamMinimumFrameTime: CFStringRef;
}

extern "C" {
    /// Controls whether the cursor is embedded within the provided buffers or not.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreamshowcursor?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamShowCursor: CFStringRef;
}

extern "C" {
    /// Controls how many frames deep the frame queue will be.  Defaults to N.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreamqueuedepth?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamQueueDepth: CFStringRef;
}

extern "C" {
    /// When outputting frames in 420v or 420f format, this key may be used to control which YCbCr matrix is used
    /// The value should be one of the three kCGDisplayStreamYCbCrMatrix values specified below.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreamycbcrmatrix?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamYCbCrMatrix: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreamycbcrmatrix_itu_r_709_2?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamYCbCrMatrix_ITU_R_709_2: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreamycbcrmatrix_itu_r_601_4?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamYCbCrMatrix_ITU_R_601_4: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgdisplaystreamycbcrmatrix_smpte_240m_1995?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGDisplayStreamYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
}

extern "C-unwind" {
    /// Returns the CF "class" ID for CGDisplayStream
    ///
    /// Returns: The CFTypeID of the CGDisplayStream class.
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Please use ScreenCaptureKit instead."]
    pub fn CGDisplayStreamGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    /// Creates a new CGDisplayStream intended to be used with a CFRunLoop
    ///
    /// This function creates a new CGDisplayStream that is to be used to get a stream of frame updates
    /// from a particular display.
    ///
    /// Parameter `display`: The CGDirectDisplayID to use as the source for generated frames
    ///
    /// Parameter `outputWidth`: The output width (in pixels, not points) of the frames to be generated.  Must not be zero.
    ///
    /// Parameter `outputHeight`: The output height (in pixels, not points) of the frames to be generated.  Must not be zero.
    ///
    /// Parameter `pixelFormat`: The desired CoreVideo/CoreMedia-style pixel format of the output IOSurfaces.  The currently
    /// supported values are:
    ///
    /// 'BGRA' Packed Little Endian ARGB8888
    /// 'l10r' Packed Little Endian ARGB2101010
    /// '420v' 2-plane "video" range YCbCr 4:2:0
    /// '420f' 2-plane "full" range YCbCr 4:2:0
    ///
    ///
    /// Parameter `properties`: Any optional properties of the CGDisplayStream
    ///
    /// Parameter `handler`: A block that will be called for frame deliver.
    ///
    /// Returns: The new CGDisplayStream object.
    #[cfg(all(
        feature = "CGDirectDisplay",
        feature = "block2",
        feature = "objc2-core-foundation",
        feature = "objc2-io-surface"
    ))]
    #[cfg(not(target_os = "watchos"))]
    #[deprecated = "Please use ScreenCaptureKit instead."]
    pub fn CGDisplayStreamCreate(
        display: CGDirectDisplayID,
        output_width: usize,
        output_height: usize,
        pixel_format: i32,
        properties: CFDictionaryRef,
        handler: CGDisplayStreamFrameAvailableHandler,
    ) -> CGDisplayStreamRef;
}

extern "C-unwind" {
    /// Begin delivering frame updates to the handler block.
    ///
    /// Parameter `displayStream`: to be started
    ///
    /// Returns: kCGErrorSuccess If the display stream was started, otherwise an error.
    #[cfg(feature = "CGError")]
    #[deprecated = "Please use ScreenCaptureKit instead."]
    pub fn CGDisplayStreamStart(display_stream: CGDisplayStreamRef) -> CGError;
}

extern "C-unwind" {
    /// End delivery of frame updates to the handler block.
    ///
    /// Parameter `displayStream`: to be stopped
    ///
    /// Returns: kCGErrorSuccess If the display stream was stopped, otherwise an error.
    ///
    /// After this call returns, the CGDisplayStream callback function will eventually be called with a
    /// status of kCGDisplayStreamFrameStatusStopped.  After that point it is safe to release the CGDisplayStream.
    /// It is safe to call this function from within the handler block, but the previous caveat still applies.
    #[cfg(feature = "CGError")]
    #[deprecated = "Please use ScreenCaptureKit instead."]
    pub fn CGDisplayStreamStop(display_stream: CGDisplayStreamRef) -> CGError;
}

extern "C-unwind" {
    /// Return the singleton CFRunLoopSourceRef for a CGDisplayStream.
    ///
    /// Parameter `displayStream`: The CGDisplayStream object
    ///
    /// Returns: The CFRunLoopSource for this displayStream.  Note: This function will return NULL if the
    /// display stream was created via  CGDisplayStreamCreateWithDispatchQueue().
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "Please use ScreenCaptureKit instead."]
    pub fn CGDisplayStreamGetRunLoopSource(
        display_stream: CGDisplayStreamRef,
    ) -> CFRunLoopSourceRef;
}
