//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextListOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSTextListOptions: NSUInteger {
        const NSTextListPrependEnclosingMarker = 1<<0;
    }
}

unsafe impl Encode for NSTextListOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextListOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkerformat?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSTextListMarkerFormat = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkerbox?language=objc)
    pub static NSTextListMarkerBox: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkercheck?language=objc)
    pub static NSTextListMarkerCheck: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkercircle?language=objc)
    pub static NSTextListMarkerCircle: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkerdiamond?language=objc)
    pub static NSTextListMarkerDiamond: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkerdisc?language=objc)
    pub static NSTextListMarkerDisc: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkerhyphen?language=objc)
    pub static NSTextListMarkerHyphen: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkersquare?language=objc)
    pub static NSTextListMarkerSquare: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkerlowercasehexadecimal?language=objc)
    pub static NSTextListMarkerLowercaseHexadecimal: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkeruppercasehexadecimal?language=objc)
    pub static NSTextListMarkerUppercaseHexadecimal: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkeroctal?language=objc)
    pub static NSTextListMarkerOctal: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkerlowercasealpha?language=objc)
    pub static NSTextListMarkerLowercaseAlpha: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkeruppercasealpha?language=objc)
    pub static NSTextListMarkerUppercaseAlpha: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkerlowercaselatin?language=objc)
    pub static NSTextListMarkerLowercaseLatin: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkeruppercaselatin?language=objc)
    pub static NSTextListMarkerUppercaseLatin: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkerlowercaseroman?language=objc)
    pub static NSTextListMarkerLowercaseRoman: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkeruppercaseroman?language=objc)
    pub static NSTextListMarkerUppercaseRoman: &'static NSTextListMarkerFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlistmarkerdecimal?language=objc)
    pub static NSTextListMarkerDecimal: &'static NSTextListMarkerFormat;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlist?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextList;
);

unsafe impl NSCoding for NSTextList {}

unsafe impl NSCopying for NSTextList {}

unsafe impl CopyingHelper for NSTextList {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSTextList {}

unsafe impl NSSecureCoding for NSTextList {}

extern_methods!(
    unsafe impl NSTextList {
        #[method_id(@__retain_semantics Init initWithMarkerFormat:options:startingItemNumber:)]
        pub unsafe fn initWithMarkerFormat_options_startingItemNumber(
            this: Allocated<Self>,
            marker_format: &NSTextListMarkerFormat,
            options: NSTextListOptions,
            starting_item_number: NSInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithMarkerFormat:options:)]
        pub unsafe fn initWithMarkerFormat_options(
            this: Allocated<Self>,
            marker_format: &NSTextListMarkerFormat,
            options: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other markerFormat)]
        pub unsafe fn markerFormat(&self) -> Retained<NSTextListMarkerFormat>;

        #[method(listOptions)]
        pub unsafe fn listOptions(&self) -> NSTextListOptions;

        #[method(startingItemNumber)]
        pub unsafe fn startingItemNumber(&self) -> NSInteger;

        /// Setter for [`startingItemNumber`][Self::startingItemNumber].
        #[method(setStartingItemNumber:)]
        pub unsafe fn setStartingItemNumber(&self, starting_item_number: NSInteger);

        #[method(isOrdered)]
        pub unsafe fn isOrdered(&self) -> bool;

        #[method_id(@__retain_semantics Other markerForItemNumber:)]
        pub unsafe fn markerForItemNumber(&self, item_number: NSInteger) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextList {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
