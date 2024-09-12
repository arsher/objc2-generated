//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCompositingOperation(pub NSUInteger);
impl NSCompositingOperation {
    #[doc(alias = "NSCompositingOperationClear")]
    pub const Clear: Self = Self(0);
    #[doc(alias = "NSCompositingOperationCopy")]
    pub const Copy: Self = Self(1);
    #[doc(alias = "NSCompositingOperationSourceOver")]
    pub const SourceOver: Self = Self(2);
    #[doc(alias = "NSCompositingOperationSourceIn")]
    pub const SourceIn: Self = Self(3);
    #[doc(alias = "NSCompositingOperationSourceOut")]
    pub const SourceOut: Self = Self(4);
    #[doc(alias = "NSCompositingOperationSourceAtop")]
    pub const SourceAtop: Self = Self(5);
    #[doc(alias = "NSCompositingOperationDestinationOver")]
    pub const DestinationOver: Self = Self(6);
    #[doc(alias = "NSCompositingOperationDestinationIn")]
    pub const DestinationIn: Self = Self(7);
    #[doc(alias = "NSCompositingOperationDestinationOut")]
    pub const DestinationOut: Self = Self(8);
    #[doc(alias = "NSCompositingOperationDestinationAtop")]
    pub const DestinationAtop: Self = Self(9);
    #[doc(alias = "NSCompositingOperationXOR")]
    pub const XOR: Self = Self(10);
    #[doc(alias = "NSCompositingOperationPlusDarker")]
    pub const PlusDarker: Self = Self(11);
    #[deprecated = "Use NSCompositingOperationSourceOver instead"]
    #[doc(alias = "NSCompositingOperationHighlight")]
    pub const Highlight: Self = Self(12);
    #[doc(alias = "NSCompositingOperationPlusLighter")]
    pub const PlusLighter: Self = Self(13);
    #[doc(alias = "NSCompositingOperationMultiply")]
    pub const Multiply: Self = Self(14);
    #[doc(alias = "NSCompositingOperationScreen")]
    pub const Screen: Self = Self(15);
    #[doc(alias = "NSCompositingOperationOverlay")]
    pub const Overlay: Self = Self(16);
    #[doc(alias = "NSCompositingOperationDarken")]
    pub const Darken: Self = Self(17);
    #[doc(alias = "NSCompositingOperationLighten")]
    pub const Lighten: Self = Self(18);
    #[doc(alias = "NSCompositingOperationColorDodge")]
    pub const ColorDodge: Self = Self(19);
    #[doc(alias = "NSCompositingOperationColorBurn")]
    pub const ColorBurn: Self = Self(20);
    #[doc(alias = "NSCompositingOperationSoftLight")]
    pub const SoftLight: Self = Self(21);
    #[doc(alias = "NSCompositingOperationHardLight")]
    pub const HardLight: Self = Self(22);
    #[doc(alias = "NSCompositingOperationDifference")]
    pub const Difference: Self = Self(23);
    #[doc(alias = "NSCompositingOperationExclusion")]
    pub const Exclusion: Self = Self(24);
    #[doc(alias = "NSCompositingOperationHue")]
    pub const Hue: Self = Self(25);
    #[doc(alias = "NSCompositingOperationSaturation")]
    pub const Saturation: Self = Self(26);
    #[doc(alias = "NSCompositingOperationColor")]
    pub const Color: Self = Self(27);
    #[doc(alias = "NSCompositingOperationLuminosity")]
    pub const Luminosity: Self = Self(28);
}

unsafe impl Encode for NSCompositingOperation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCompositingOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub static NSCompositeClear: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Clear.0);

pub static NSCompositeCopy: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Copy.0);

pub static NSCompositeSourceOver: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::SourceOver.0);

pub static NSCompositeSourceIn: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::SourceIn.0);

pub static NSCompositeSourceOut: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::SourceOut.0);

pub static NSCompositeSourceAtop: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::SourceAtop.0);

pub static NSCompositeDestinationOver: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::DestinationOver.0);

pub static NSCompositeDestinationIn: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::DestinationIn.0);

pub static NSCompositeDestinationOut: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::DestinationOut.0);

pub static NSCompositeDestinationAtop: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::DestinationAtop.0);

pub static NSCompositeXOR: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::XOR.0);

pub static NSCompositePlusDarker: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::PlusDarker.0);

pub static NSCompositeHighlight: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Highlight.0);

pub static NSCompositePlusLighter: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::PlusLighter.0);

pub static NSCompositeMultiply: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Multiply.0);

pub static NSCompositeScreen: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Screen.0);

pub static NSCompositeOverlay: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Overlay.0);

pub static NSCompositeDarken: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Darken.0);

pub static NSCompositeLighten: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Lighten.0);

pub static NSCompositeColorDodge: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::ColorDodge.0);

pub static NSCompositeColorBurn: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::ColorBurn.0);

pub static NSCompositeSoftLight: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::SoftLight.0);

pub static NSCompositeHardLight: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::HardLight.0);

pub static NSCompositeDifference: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Difference.0);

pub static NSCompositeExclusion: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Exclusion.0);

pub static NSCompositeHue: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Hue.0);

pub static NSCompositeSaturation: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Saturation.0);

pub static NSCompositeColor: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Color.0);

pub static NSCompositeLuminosity: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Luminosity.0);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBackingStoreType(pub NSUInteger);
impl NSBackingStoreType {
    #[deprecated]
    pub const NSBackingStoreRetained: Self = Self(0);
    #[deprecated]
    pub const NSBackingStoreNonretained: Self = Self(1);
    pub const NSBackingStoreBuffered: Self = Self(2);
}

unsafe impl Encode for NSBackingStoreType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSBackingStoreType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWindowOrderingMode(pub NSInteger);
impl NSWindowOrderingMode {
    pub const NSWindowAbove: Self = Self(1);
    pub const NSWindowBelow: Self = Self(-1);
    pub const NSWindowOut: Self = Self(0);
}

unsafe impl Encode for NSWindowOrderingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSWindowOrderingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFocusRingPlacement(pub NSUInteger);
impl NSFocusRingPlacement {
    pub const NSFocusRingOnly: Self = Self(0);
    pub const NSFocusRingBelow: Self = Self(1);
    pub const NSFocusRingAbove: Self = Self(2);
}

unsafe impl Encode for NSFocusRingPlacement {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFocusRingPlacement {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFocusRingType(pub NSUInteger);
impl NSFocusRingType {
    #[doc(alias = "NSFocusRingTypeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSFocusRingTypeNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "NSFocusRingTypeExterior")]
    pub const Exterior: Self = Self(2);
}

unsafe impl Encode for NSFocusRingType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFocusRingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSColorRenderingIntent(pub NSInteger);
impl NSColorRenderingIntent {
    #[doc(alias = "NSColorRenderingIntentDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSColorRenderingIntentAbsoluteColorimetric")]
    pub const AbsoluteColorimetric: Self = Self(1);
    #[doc(alias = "NSColorRenderingIntentRelativeColorimetric")]
    pub const RelativeColorimetric: Self = Self(2);
    #[doc(alias = "NSColorRenderingIntentPerceptual")]
    pub const Perceptual: Self = Self(3);
    #[doc(alias = "NSColorRenderingIntentSaturation")]
    pub const Saturation: Self = Self(4);
}

unsafe impl Encode for NSColorRenderingIntent {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSColorRenderingIntent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_ENUM
pub type NSColorSpaceName = NSString;

extern "C" {
    pub static NSCalibratedWhiteColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    pub static NSCalibratedRGBColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    pub static NSDeviceWhiteColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    pub static NSDeviceRGBColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    pub static NSDeviceCMYKColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    pub static NSNamedColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    pub static NSPatternColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    pub static NSCustomColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    pub static NSCalibratedBlackColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    pub static NSDeviceBlackColorSpace: &'static NSColorSpaceName;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWindowDepth(pub i32);
impl NSWindowDepth {
    #[doc(alias = "NSWindowDepthTwentyfourBitRGB")]
    pub const TwentyfourBitRGB: Self = Self(0x208);
    #[doc(alias = "NSWindowDepthSixtyfourBitRGB")]
    pub const SixtyfourBitRGB: Self = Self(0x210);
    #[doc(alias = "NSWindowDepthOnehundredtwentyeightBitRGB")]
    pub const OnehundredtwentyeightBitRGB: Self = Self(0x220);
}

unsafe impl Encode for NSWindowDepth {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for NSWindowDepth {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    pub fn NSBestDepth(
        color_space: &NSColorSpaceName,
        bps: NSInteger,
        bpp: NSInteger,
        planar: Bool,
        exact_match: *mut Bool,
    ) -> NSWindowDepth;
}

extern "C-unwind" {
    pub fn NSPlanarFromDepth(depth: NSWindowDepth) -> Bool;
}

extern "C-unwind" {
    pub fn NSColorSpaceFromDepth(depth: NSWindowDepth) -> *mut NSColorSpaceName;
}

extern "C-unwind" {
    pub fn NSBitsPerSampleFromDepth(depth: NSWindowDepth) -> NSInteger;
}

extern "C-unwind" {
    pub fn NSBitsPerPixelFromDepth(depth: NSWindowDepth) -> NSInteger;
}

extern "C-unwind" {
    pub fn NSNumberOfColorComponents(color_space_name: &NSColorSpaceName) -> NSInteger;
}

extern "C-unwind" {
    pub fn NSAvailableWindowDepths() -> NonNull<NSWindowDepth>;
}

extern "C" {
    pub static NSWhite: CGFloat;
}

extern "C" {
    pub static NSLightGray: CGFloat;
}

extern "C" {
    pub static NSDarkGray: CGFloat;
}

extern "C" {
    pub static NSBlack: CGFloat;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDisplayGamut(pub NSInteger);
impl NSDisplayGamut {
    #[doc(alias = "NSDisplayGamutSRGB")]
    pub const SRGB: Self = Self(1);
    #[doc(alias = "NSDisplayGamutP3")]
    pub const P3: Self = Self(2);
}

unsafe impl Encode for NSDisplayGamut {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSDisplayGamut {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSDeviceDescriptionKey = NSString;

extern "C" {
    pub static NSDeviceResolution: &'static NSDeviceDescriptionKey;
}

extern "C" {
    pub static NSDeviceColorSpaceName: &'static NSDeviceDescriptionKey;
}

extern "C" {
    pub static NSDeviceBitsPerSample: &'static NSDeviceDescriptionKey;
}

extern "C" {
    pub static NSDeviceIsScreen: &'static NSDeviceDescriptionKey;
}

extern "C" {
    pub static NSDeviceIsPrinter: &'static NSDeviceDescriptionKey;
}

extern "C" {
    pub static NSDeviceSize: &'static NSDeviceDescriptionKey;
}

extern "C-unwind" {
    pub fn NSRectFill(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSRectFillList(rects: NonNull<NSRect>, count: NSInteger);
}

extern "C-unwind" {
    pub fn NSRectFillListWithGrays(rects: NonNull<NSRect>, grays: NonNull<CGFloat>, num: NSInteger);
}

extern "C-unwind" {
    #[cfg(feature = "NSColor")]
    pub fn NSRectFillListWithColors(
        rects: NonNull<NSRect>,
        colors: NonNull<NonNull<NSColor>>,
        num: NSInteger,
    );
}

extern "C-unwind" {
    pub fn NSRectFillUsingOperation(rect: NSRect, op: NSCompositingOperation);
}

extern "C-unwind" {
    pub fn NSRectFillListUsingOperation(
        rects: NonNull<NSRect>,
        count: NSInteger,
        op: NSCompositingOperation,
    );
}

extern "C-unwind" {
    #[cfg(feature = "NSColor")]
    pub fn NSRectFillListWithColorsUsingOperation(
        rects: NonNull<NSRect>,
        colors: NonNull<NonNull<NSColor>>,
        num: NSInteger,
        op: NSCompositingOperation,
    );
}

extern "C-unwind" {
    pub fn NSFrameRect(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSFrameRectWithWidth(rect: NSRect, frame_width: CGFloat);
}

extern "C-unwind" {
    pub fn NSFrameRectWithWidthUsingOperation(
        rect: NSRect,
        frame_width: CGFloat,
        op: NSCompositingOperation,
    );
}

extern "C-unwind" {
    pub fn NSRectClip(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSRectClipList(rects: NonNull<NSRect>, count: NSInteger);
}

extern "C-unwind" {
    pub fn NSDrawTiledRects(
        bounds_rect: NSRect,
        clip_rect: NSRect,
        sides: NonNull<NSRectEdge>,
        grays: NonNull<CGFloat>,
        count: NSInteger,
    ) -> NSRect;
}

extern "C-unwind" {
    pub fn NSDrawGrayBezel(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDrawGroove(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDrawWhiteBezel(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDrawButton(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSEraseRect(rect: NSRect);
}

extern "C-unwind" {
    #[cfg(feature = "NSColor")]
    #[deprecated = "Use -[NSBitmapImageRep colorAtX:y:] to interrogate pixel values.  If necessary, use -[NSView cacheDisplayInRect:toBitmapImageRep:] to snapshot a view hierarchy into an NSBitmapImageRep."]
    pub fn NSReadPixel(passed_point: NSPoint) -> *mut NSColor;
}

extern "C-unwind" {
    #[deprecated]
    pub fn NSHighlightRect(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSBeep();
}

extern "C-unwind" {
    #[deprecated = "Doesn't return anything useful since 10.0"]
    pub fn NSGetWindowServerMemory(
        context: NSInteger,
        virtual_memory: NonNull<NSInteger>,
        window_backing_memory: NonNull<NSInteger>,
        window_dump_string: NonNull<NonNull<NSString>>,
    ) -> NSInteger;
}

extern "C-unwind" {
    #[cfg(feature = "NSColor")]
    pub fn NSDrawColorTiledRects(
        bounds_rect: NSRect,
        clip_rect: NSRect,
        sides: NonNull<NSRectEdge>,
        colors: NonNull<NonNull<NSColor>>,
        count: NSInteger,
    ) -> NSRect;
}

extern "C-unwind" {
    pub fn NSDrawDarkBezel(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDrawLightBezel(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDottedFrameRect(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDrawWindowBackground(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSSetFocusRingStyle(placement: NSFocusRingPlacement);
}

extern "C-unwind" {
    #[deprecated = "As of 10.11 it is not generally necessary to take explicit action to achieve visual atomicity. +[NSAnimationContext runAnimationGroup:] and other similar methods can be used when a stronger than normal need for visual atomicity is required. The NSAnimationContext methods do not suffer from the same performance problems as NSDisableScreenUpdates."]
    pub fn NSDisableScreenUpdates();
}

extern "C-unwind" {
    #[deprecated = "As of 10.11 it is not generally necessary to take explicit action to achieve visual atomicity. +[NSAnimationContext runAnimationGroup:] and other similar methods can be used when a stronger than normal need for visual atomicity is required. The NSAnimationContext methods do not suffer from the same performance problems as NSEnableScreenUpdates."]
    pub fn NSEnableScreenUpdates();
}

// NS_ENUM
#[deprecated = "Use +[NSCursor disappearingItemCursor] instead"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAnimationEffect(pub NSUInteger);
impl NSAnimationEffect {
    #[doc(alias = "NSAnimationEffectDisappearingItemDefault")]
    pub const DisappearingItemDefault: Self = Self(0);
    #[doc(alias = "NSAnimationEffectPoof")]
    pub const Poof: Self = Self(10);
}

unsafe impl Encode for NSAnimationEffect {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSAnimationEffect {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[deprecated = "Use +[NSCursor disappearingItemCursor] instead"]
    pub fn NSShowAnimationEffect(
        animation_effect: NSAnimationEffect,
        center_location: NSPoint,
        size: NSSize,
        animation_delegate: Option<&AnyObject>,
        did_end_selector: Option<Sel>,
        context_info: *mut c_void,
    );
}

extern "C-unwind" {
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub fn NSCountWindows(count: NonNull<NSInteger>);
}

extern "C-unwind" {
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub fn NSWindowList(size: NSInteger, list: NonNull<NSInteger>);
}

extern "C-unwind" {
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub fn NSCountWindowsForContext(context: NSInteger, count: NonNull<NSInteger>);
}

extern "C-unwind" {
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub fn NSWindowListForContext(context: NSInteger, size: NSInteger, list: NonNull<NSInteger>);
}

extern "C-unwind" {
    #[deprecated]
    pub fn NSCopyBits(src_g_state: NSInteger, src_rect: NSRect, dest_point: NSPoint);
}
