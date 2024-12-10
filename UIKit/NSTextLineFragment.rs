//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstextlinefragment?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextLineFragment;
);

unsafe impl NSCoding for NSTextLineFragment {}

unsafe impl NSObjectProtocol for NSTextLineFragment {}

unsafe impl NSSecureCoding for NSTextLineFragment {}

extern_methods!(
    unsafe impl NSTextLineFragment {
        #[method_id(@__retain_semantics Init initWithAttributedString:range:)]
        pub unsafe fn initWithAttributedString_range(
            this: Allocated<Self>,
            attributed_string: &NSAttributedString,
            range: NSRange,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithString:attributes:range:)]
        pub unsafe fn initWithString_attributes_range(
            this: Allocated<Self>,
            string: &NSString,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            range: NSRange,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Retained<NSAttributedString>;

        #[method(characterRange)]
        pub unsafe fn characterRange(&self) -> NSRange;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(typographicBounds)]
        pub unsafe fn typographicBounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(glyphOrigin)]
        pub unsafe fn glyphOrigin(&self) -> CGPoint;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
        #[method(drawAtPoint:inContext:)]
        pub unsafe fn drawAtPoint_inContext(&self, point: CGPoint, context: CGContextRef);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(locationForCharacterAtIndex:)]
        pub unsafe fn locationForCharacterAtIndex(&self, index: NSInteger) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(characterIndexForPoint:)]
        pub unsafe fn characterIndexForPoint(&self, point: CGPoint) -> NSInteger;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(fractionOfDistanceThroughGlyphForPoint:)]
        pub unsafe fn fractionOfDistanceThroughGlyphForPoint(&self, point: CGPoint) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextLineFragment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
