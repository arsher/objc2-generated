//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionunitstype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionUnitsType(pub NSInteger);
impl AVCaptionUnitsType {
    #[doc(alias = "AVCaptionUnitsTypeUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "AVCaptionUnitsTypeCells")]
    pub const Cells: Self = Self(1);
    #[doc(alias = "AVCaptionUnitsTypePercent")]
    pub const Percent: Self = Self(2);
}

unsafe impl Encode for AVCaptionUnitsType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionUnitsType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptiondimension?language=objc)
#[cfg(feature = "objc2-core-foundation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVCaptionDimension {
    pub value: CGFloat,
    pub units: AVCaptionUnitsType,
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Encode for AVCaptionDimension {
    const ENCODING: Encoding = Encoding::Struct(
        "AVCaptionDimension",
        &[<CGFloat>::ENCODING, <AVCaptionUnitsType>::ENCODING],
    );
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl RefEncode for AVCaptionDimension {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionpoint?language=objc)
#[cfg(feature = "objc2-core-foundation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVCaptionPoint {
    pub x: AVCaptionDimension,
    pub y: AVCaptionDimension,
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Encode for AVCaptionPoint {
    const ENCODING: Encoding = Encoding::Struct(
        "AVCaptionPoint",
        &[
            <AVCaptionDimension>::ENCODING,
            <AVCaptionDimension>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl RefEncode for AVCaptionPoint {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionsize?language=objc)
#[cfg(feature = "objc2-core-foundation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVCaptionSize {
    pub width: AVCaptionDimension,
    pub height: AVCaptionDimension,
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Encode for AVCaptionSize {
    const ENCODING: Encoding = Encoding::Struct(
        "AVCaptionSize",
        &[
            <AVCaptionDimension>::ENCODING,
            <AVCaptionDimension>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl RefEncode for AVCaptionSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn AVCaptionDimensionMake(value: CGFloat, units: AVCaptionUnitsType) -> AVCaptionDimension;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn AVCaptionPointMake(x: AVCaptionDimension, y: AVCaptionDimension) -> AVCaptionPoint;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn AVCaptionSizeMake(
        width: AVCaptionDimension,
        height: AVCaptionDimension,
    ) -> AVCaptionSize;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionregiondisplayalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionRegionDisplayAlignment(pub NSInteger);
impl AVCaptionRegionDisplayAlignment {
    #[doc(alias = "AVCaptionRegionDisplayAlignmentBefore")]
    pub const Before: Self = Self(0);
    #[doc(alias = "AVCaptionRegionDisplayAlignmentCenter")]
    pub const Center: Self = Self(1);
    #[doc(alias = "AVCaptionRegionDisplayAlignmentAfter")]
    pub const After: Self = Self(2);
}

unsafe impl Encode for AVCaptionRegionDisplayAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionRegionDisplayAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionregionwritingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionRegionWritingMode(pub NSInteger);
impl AVCaptionRegionWritingMode {
    #[doc(alias = "AVCaptionRegionWritingModeLeftToRightAndTopToBottom")]
    pub const LeftToRightAndTopToBottom: Self = Self(0);
    #[doc(alias = "AVCaptionRegionWritingModeTopToBottomAndRightToLeft")]
    pub const TopToBottomAndRightToLeft: Self = Self(2);
}

unsafe impl Encode for AVCaptionRegionWritingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionRegionWritingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionregionscroll?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionRegionScroll(pub NSInteger);
impl AVCaptionRegionScroll {
    #[doc(alias = "AVCaptionRegionScrollNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "AVCaptionRegionScrollRollUp")]
    pub const RollUp: Self = Self(1);
}

unsafe impl Encode for AVCaptionRegionScroll {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionRegionScroll {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionregion?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptionRegion;
);

unsafe impl NSCoding for AVCaptionRegion {}

unsafe impl NSCopying for AVCaptionRegion {}

unsafe impl CopyingHelper for AVCaptionRegion {
    type Result = Self;
}

unsafe impl NSMutableCopying for AVCaptionRegion {}

unsafe impl MutableCopyingHelper for AVCaptionRegion {
    type Result = AVMutableCaptionRegion;
}

unsafe impl NSObjectProtocol for AVCaptionRegion {}

unsafe impl NSSecureCoding for AVCaptionRegion {}

extern_methods!(
    unsafe impl AVCaptionRegion {
        #[method_id(@__retain_semantics Other appleITTTopRegion)]
        pub unsafe fn appleITTTopRegion() -> Retained<AVCaptionRegion>;

        #[method_id(@__retain_semantics Other appleITTBottomRegion)]
        pub unsafe fn appleITTBottomRegion() -> Retained<AVCaptionRegion>;

        #[method_id(@__retain_semantics Other appleITTLeftRegion)]
        pub unsafe fn appleITTLeftRegion() -> Retained<AVCaptionRegion>;

        #[method_id(@__retain_semantics Other appleITTRightRegion)]
        pub unsafe fn appleITTRightRegion() -> Retained<AVCaptionRegion>;

        #[method_id(@__retain_semantics Other subRipTextBottomRegion)]
        pub unsafe fn subRipTextBottomRegion() -> Retained<AVCaptionRegion>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(origin)]
        pub unsafe fn origin(&self) -> AVCaptionPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(size)]
        pub unsafe fn size(&self) -> AVCaptionSize;

        #[method(scroll)]
        pub unsafe fn scroll(&self) -> AVCaptionRegionScroll;

        #[method(displayAlignment)]
        pub unsafe fn displayAlignment(&self) -> AVCaptionRegionDisplayAlignment;

        #[method(writingMode)]
        pub unsafe fn writingMode(&self) -> AVCaptionRegionWritingMode;

        #[method(encodeWithCoder:)]
        pub unsafe fn encodeWithCoder(&self, encoder: &NSCoder);

        #[method(isEqual:)]
        pub unsafe fn isEqual(&self, object: &AnyObject) -> bool;

        #[method_id(@__retain_semantics MutableCopy mutableCopyWithZone:)]
        pub unsafe fn mutableCopyWithZone(&self, zone: *mut NSZone) -> Retained<AnyObject>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVCaptionRegion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmutablecaptionregion?language=objc)
    #[unsafe(super(AVCaptionRegion, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMutableCaptionRegion;
);

unsafe impl NSCoding for AVMutableCaptionRegion {}

unsafe impl NSCopying for AVMutableCaptionRegion {}

unsafe impl CopyingHelper for AVMutableCaptionRegion {
    type Result = AVCaptionRegion;
}

unsafe impl NSMutableCopying for AVMutableCaptionRegion {}

unsafe impl MutableCopyingHelper for AVMutableCaptionRegion {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVMutableCaptionRegion {}

unsafe impl NSSecureCoding for AVMutableCaptionRegion {}

extern_methods!(
    unsafe impl AVMutableCaptionRegion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(origin)]
        pub unsafe fn origin(&self) -> AVCaptionPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setOrigin:)]
        pub unsafe fn setOrigin(&self, origin: AVCaptionPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(size)]
        pub unsafe fn size(&self) -> AVCaptionSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: AVCaptionSize);

        #[method(scroll)]
        pub unsafe fn scroll(&self) -> AVCaptionRegionScroll;

        #[method(setScroll:)]
        pub unsafe fn setScroll(&self, scroll: AVCaptionRegionScroll);

        #[method(displayAlignment)]
        pub unsafe fn displayAlignment(&self) -> AVCaptionRegionDisplayAlignment;

        #[method(setDisplayAlignment:)]
        pub unsafe fn setDisplayAlignment(
            &self,
            display_alignment: AVCaptionRegionDisplayAlignment,
        );

        #[method(writingMode)]
        pub unsafe fn writingMode(&self) -> AVCaptionRegionWritingMode;

        #[method(setWritingMode:)]
        pub unsafe fn setWritingMode(&self, writing_mode: AVCaptionRegionWritingMode);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMutableCaptionRegion {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionanimation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionAnimation(pub NSInteger);
impl AVCaptionAnimation {
    #[doc(alias = "AVCaptionAnimationNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "AVCaptionAnimationCharacterReveal")]
    pub const CharacterReveal: Self = Self(1);
}

unsafe impl Encode for AVCaptionAnimation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionAnimation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaption?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaption;
);

unsafe impl NSCoding for AVCaption {}

unsafe impl NSCopying for AVCaption {}

unsafe impl CopyingHelper for AVCaption {
    type Result = Self;
}

unsafe impl NSMutableCopying for AVCaption {}

unsafe impl MutableCopyingHelper for AVCaption {
    type Result = AVMutableCaption;
}

unsafe impl NSObjectProtocol for AVCaption {}

unsafe impl NSSecureCoding for AVCaption {}

extern_methods!(
    unsafe impl AVCaption {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method_id(@__retain_semantics Init initWithText:timeRange:)]
        pub unsafe fn initWithText_timeRange(
            this: Allocated<Self>,
            text: &NSString,
            time_range: CMTimeRange,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-core-media")]
        #[method(timeRange)]
        pub unsafe fn timeRange(&self) -> CMTimeRange;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmutablecaption?language=objc)
    #[unsafe(super(AVCaption, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMutableCaption;
);

unsafe impl NSCoding for AVMutableCaption {}

unsafe impl NSCopying for AVMutableCaption {}

unsafe impl CopyingHelper for AVMutableCaption {
    type Result = AVCaption;
}

unsafe impl NSMutableCopying for AVMutableCaption {}

unsafe impl MutableCopyingHelper for AVMutableCaption {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVMutableCaption {}

unsafe impl NSSecureCoding for AVMutableCaption {}

extern_methods!(
    unsafe impl AVMutableCaption {
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Retained<NSString>;

        #[method(setText:)]
        pub unsafe fn setText(&self, text: &NSString);

        #[cfg(feature = "objc2-core-media")]
        #[method(timeRange)]
        pub unsafe fn timeRange(&self) -> CMTimeRange;

        #[cfg(feature = "objc2-core-media")]
        #[method(setTimeRange:)]
        pub unsafe fn setTimeRange(&self, time_range: CMTimeRange);
    }
);

extern_methods!(
    /// Methods declared on superclass `AVCaption`
    unsafe impl AVMutableCaption {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method_id(@__retain_semantics Init initWithText:timeRange:)]
        pub unsafe fn initWithText_timeRange(
            this: Allocated<Self>,
            text: &NSString,
            time_range: CMTimeRange,
        ) -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionfontweight?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionFontWeight(pub NSInteger);
impl AVCaptionFontWeight {
    #[doc(alias = "AVCaptionFontWeightUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "AVCaptionFontWeightNormal")]
    pub const Normal: Self = Self(1);
    #[doc(alias = "AVCaptionFontWeightBold")]
    pub const Bold: Self = Self(2);
}

unsafe impl Encode for AVCaptionFontWeight {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionFontWeight {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionfontstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionFontStyle(pub NSInteger);
impl AVCaptionFontStyle {
    #[doc(alias = "AVCaptionFontStyleUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "AVCaptionFontStyleNormal")]
    pub const Normal: Self = Self(1);
    #[doc(alias = "AVCaptionFontStyleItalic")]
    pub const Italic: Self = Self(2);
}

unsafe impl Encode for AVCaptionFontStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionFontStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptiondecoration?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionDecoration(pub NSUInteger);
bitflags::bitflags! {
    impl AVCaptionDecoration: NSUInteger {
        #[doc(alias = "AVCaptionDecorationNone")]
        const None = 0;
        #[doc(alias = "AVCaptionDecorationUnderline")]
        const Underline = 1<<0;
        #[doc(alias = "AVCaptionDecorationLineThrough")]
        const LineThrough = 1<<1;
        #[doc(alias = "AVCaptionDecorationOverline")]
        const Overline = 1<<2;
    }
}

unsafe impl Encode for AVCaptionDecoration {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionDecoration {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptiontextcombine?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionTextCombine(pub NSInteger);
impl AVCaptionTextCombine {
    #[doc(alias = "AVCaptionTextCombineAll")]
    pub const All: Self = Self(-1);
    #[doc(alias = "AVCaptionTextCombineNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "AVCaptionTextCombineOneDigit")]
    pub const OneDigit: Self = Self(1);
    #[doc(alias = "AVCaptionTextCombineTwoDigits")]
    pub const TwoDigits: Self = Self(2);
    #[doc(alias = "AVCaptionTextCombineThreeDigits")]
    pub const ThreeDigits: Self = Self(3);
    #[doc(alias = "AVCaptionTextCombineFourDigits")]
    pub const FourDigits: Self = Self(4);
}

unsafe impl Encode for AVCaptionTextCombine {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionTextCombine {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptiontextalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionTextAlignment(pub NSInteger);
impl AVCaptionTextAlignment {
    #[doc(alias = "AVCaptionTextAlignmentStart")]
    pub const Start: Self = Self(0);
    #[doc(alias = "AVCaptionTextAlignmentEnd")]
    pub const End: Self = Self(1);
    #[doc(alias = "AVCaptionTextAlignmentCenter")]
    pub const Center: Self = Self(2);
    #[doc(alias = "AVCaptionTextAlignmentLeft")]
    pub const Left: Self = Self(3);
    #[doc(alias = "AVCaptionTextAlignmentRight")]
    pub const Right: Self = Self(4);
}

unsafe impl Encode for AVCaptionTextAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionTextAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// Styling
    unsafe impl AVCaption {
        #[cfg(feature = "objc2-core-graphics")]
        #[method(textColorAtIndex:range:)]
        pub unsafe fn textColorAtIndex_range(
            &self,
            index: NSInteger,
            out_range: *mut NSRange,
        ) -> CGColorRef;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(backgroundColorAtIndex:range:)]
        pub unsafe fn backgroundColorAtIndex_range(
            &self,
            index: NSInteger,
            out_range: *mut NSRange,
        ) -> CGColorRef;

        #[method(fontWeightAtIndex:range:)]
        pub unsafe fn fontWeightAtIndex_range(
            &self,
            index: NSInteger,
            out_range: *mut NSRange,
        ) -> AVCaptionFontWeight;

        #[method(fontStyleAtIndex:range:)]
        pub unsafe fn fontStyleAtIndex_range(
            &self,
            index: NSInteger,
            out_range: *mut NSRange,
        ) -> AVCaptionFontStyle;

        #[method(decorationAtIndex:range:)]
        pub unsafe fn decorationAtIndex_range(
            &self,
            index: NSInteger,
            out_range: *mut NSRange,
        ) -> AVCaptionDecoration;

        #[method(textCombineAtIndex:range:)]
        pub unsafe fn textCombineAtIndex_range(
            &self,
            index: NSInteger,
            out_range: *mut NSRange,
        ) -> AVCaptionTextCombine;

        #[method_id(@__retain_semantics Other rubyAtIndex:range:)]
        pub unsafe fn rubyAtIndex_range(
            &self,
            index: NSInteger,
            out_range: *mut NSRange,
        ) -> Option<Retained<AVCaptionRuby>>;
    }
);

extern_methods!(
    /// Region
    unsafe impl AVCaption {
        #[method_id(@__retain_semantics Other region)]
        pub unsafe fn region(&self) -> Option<Retained<AVCaptionRegion>>;

        #[method(textAlignment)]
        pub unsafe fn textAlignment(&self) -> AVCaptionTextAlignment;
    }
);

extern_methods!(
    /// Animation
    unsafe impl AVCaption {
        #[method(animation)]
        pub unsafe fn animation(&self) -> AVCaptionAnimation;
    }
);

extern_methods!(
    /// Styling
    unsafe impl AVMutableCaption {
        #[cfg(feature = "objc2-core-graphics")]
        #[method(setTextColor:inRange:)]
        pub unsafe fn setTextColor_inRange(&self, color: CGColorRef, range: NSRange);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(setBackgroundColor:inRange:)]
        pub unsafe fn setBackgroundColor_inRange(&self, color: CGColorRef, range: NSRange);

        #[method(setFontWeight:inRange:)]
        pub unsafe fn setFontWeight_inRange(
            &self,
            font_weight: AVCaptionFontWeight,
            range: NSRange,
        );

        #[method(setFontStyle:inRange:)]
        pub unsafe fn setFontStyle_inRange(&self, font_style: AVCaptionFontStyle, range: NSRange);

        #[method(setDecoration:inRange:)]
        pub unsafe fn setDecoration_inRange(&self, decoration: AVCaptionDecoration, range: NSRange);

        #[method(setTextCombine:inRange:)]
        pub unsafe fn setTextCombine_inRange(
            &self,
            text_combine: AVCaptionTextCombine,
            range: NSRange,
        );

        #[method(setRuby:inRange:)]
        pub unsafe fn setRuby_inRange(&self, ruby: &AVCaptionRuby, range: NSRange);

        #[method(removeTextColorInRange:)]
        pub unsafe fn removeTextColorInRange(&self, range: NSRange);

        #[method(removeBackgroundColorInRange:)]
        pub unsafe fn removeBackgroundColorInRange(&self, range: NSRange);

        #[method(removeFontWeightInRange:)]
        pub unsafe fn removeFontWeightInRange(&self, range: NSRange);

        #[method(removeFontStyleInRange:)]
        pub unsafe fn removeFontStyleInRange(&self, range: NSRange);

        #[method(removeDecorationInRange:)]
        pub unsafe fn removeDecorationInRange(&self, range: NSRange);

        #[method(removeTextCombineInRange:)]
        pub unsafe fn removeTextCombineInRange(&self, range: NSRange);

        #[method(removeRubyInRange:)]
        pub unsafe fn removeRubyInRange(&self, range: NSRange);
    }
);

extern_methods!(
    /// Region
    unsafe impl AVMutableCaption {
        #[method_id(@__retain_semantics Other region)]
        pub unsafe fn region(&self) -> Retained<AVCaptionRegion>;

        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: &AVCaptionRegion);

        #[method(textAlignment)]
        pub unsafe fn textAlignment(&self) -> AVCaptionTextAlignment;

        #[method(setTextAlignment:)]
        pub unsafe fn setTextAlignment(&self, text_alignment: AVCaptionTextAlignment);
    }
);

extern_methods!(
    /// Animation
    unsafe impl AVMutableCaption {
        #[method(animation)]
        pub unsafe fn animation(&self) -> AVCaptionAnimation;

        #[method(setAnimation:)]
        pub unsafe fn setAnimation(&self, animation: AVCaptionAnimation);
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionrubyposition?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionRubyPosition(pub NSInteger);
impl AVCaptionRubyPosition {
    #[doc(alias = "AVCaptionRubyPositionBefore")]
    pub const Before: Self = Self(0);
    #[doc(alias = "AVCaptionRubyPositionAfter")]
    pub const After: Self = Self(1);
}

unsafe impl Encode for AVCaptionRubyPosition {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionRubyPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionrubyalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionRubyAlignment(pub NSInteger);
impl AVCaptionRubyAlignment {
    #[doc(alias = "AVCaptionRubyAlignmentStart")]
    pub const Start: Self = Self(0);
    #[doc(alias = "AVCaptionRubyAlignmentCenter")]
    pub const Center: Self = Self(1);
    #[doc(alias = "AVCaptionRubyAlignmentDistributeSpaceBetween")]
    pub const DistributeSpaceBetween: Self = Self(2);
    #[doc(alias = "AVCaptionRubyAlignmentDistributeSpaceAround")]
    pub const DistributeSpaceAround: Self = Self(3);
}

unsafe impl Encode for AVCaptionRubyAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionRubyAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionruby?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptionRuby;
);

unsafe impl Send for AVCaptionRuby {}

unsafe impl Sync for AVCaptionRuby {}

unsafe impl NSCoding for AVCaptionRuby {}

unsafe impl NSCopying for AVCaptionRuby {}

unsafe impl CopyingHelper for AVCaptionRuby {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVCaptionRuby {}

unsafe impl NSSecureCoding for AVCaptionRuby {}

extern_methods!(
    unsafe impl AVCaptionRuby {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithText:)]
        pub unsafe fn initWithText(this: Allocated<Self>, text: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithText:position:alignment:)]
        pub unsafe fn initWithText_position_alignment(
            this: Allocated<Self>,
            text: &NSString,
            position: AVCaptionRubyPosition,
            alignment: AVCaptionRubyAlignment,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Retained<NSString>;

        #[method(position)]
        pub unsafe fn position(&self) -> AVCaptionRubyPosition;

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> AVCaptionRubyAlignment;
    }
);
