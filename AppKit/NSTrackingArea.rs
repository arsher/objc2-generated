//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstrackingareaoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTrackingAreaOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSTrackingAreaOptions: NSUInteger {
        const NSTrackingMouseEnteredAndExited = 0x01;
        const NSTrackingMouseMoved = 0x02;
        const NSTrackingCursorUpdate = 0x04;
        const NSTrackingActiveWhenFirstResponder = 0x10;
        const NSTrackingActiveInKeyWindow = 0x20;
        const NSTrackingActiveInActiveApp = 0x40;
        const NSTrackingActiveAlways = 0x80;
        const NSTrackingAssumeInside = 0x100;
        const NSTrackingInVisibleRect = 0x200;
        const NSTrackingEnabledDuringMouseDrag = 0x400;
    }
}

unsafe impl Encode for NSTrackingAreaOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTrackingAreaOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstrackingarea?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTrackingArea;
);

unsafe impl NSCoding for NSTrackingArea {}

unsafe impl NSCopying for NSTrackingArea {}

unsafe impl CopyingHelper for NSTrackingArea {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSTrackingArea {}

extern_methods!(
    unsafe impl NSTrackingArea {
        #[method_id(@__retain_semantics Init initWithRect:options:owner:userInfo:)]
        pub unsafe fn initWithRect_options_owner_userInfo(
            this: Allocated<Self>,
            rect: NSRect,
            options: NSTrackingAreaOptions,
            owner: Option<&AnyObject>,
            user_info: Option<&NSDictionary<AnyObject, AnyObject>>,
        ) -> Retained<Self>;

        #[method(rect)]
        pub unsafe fn rect(&self) -> NSRect;

        #[method(options)]
        pub unsafe fn options(&self) -> NSTrackingAreaOptions;

        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<NSDictionary<AnyObject, AnyObject>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTrackingArea {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
