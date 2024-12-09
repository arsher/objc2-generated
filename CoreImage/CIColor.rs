//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cicolor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIColor;
);

unsafe impl NSCoding for CIColor {}

unsafe impl NSCopying for CIColor {}

unsafe impl CopyingHelper for CIColor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIColor {}

unsafe impl NSSecureCoding for CIColor {}

extern_methods!(
    unsafe impl CIColor {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithRed:green:blue:alpha:)]
        pub unsafe fn colorWithRed_green_blue_alpha(
            r: CGFloat,
            g: CGFloat,
            b: CGFloat,
            a: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithRed:green:blue:)]
        pub unsafe fn colorWithRed_green_blue(r: CGFloat, g: CGFloat, b: CGFloat)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Other colorWithString:)]
        pub unsafe fn colorWithString(representation: &NSString) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithRed:green:blue:alpha:)]
        pub unsafe fn initWithRed_green_blue_alpha(
            this: Allocated<Self>,
            r: CGFloat,
            g: CGFloat,
            b: CGFloat,
            a: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithRed:green:blue:)]
        pub unsafe fn initWithRed_green_blue(
            this: Allocated<Self>,
            r: CGFloat,
            g: CGFloat,
            b: CGFloat,
        ) -> Retained<Self>;

        #[method(numberOfComponents)]
        pub unsafe fn numberOfComponents(&self) -> usize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(components)]
        pub unsafe fn components(&self) -> NonNull<CGFloat>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(red)]
        pub unsafe fn red(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(green)]
        pub unsafe fn green(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(blue)]
        pub unsafe fn blue(&self) -> CGFloat;

        #[method_id(@__retain_semantics Other stringRepresentation)]
        pub unsafe fn stringRepresentation(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other blackColor)]
        pub unsafe fn blackColor() -> Retained<CIColor>;

        #[method_id(@__retain_semantics Other whiteColor)]
        pub unsafe fn whiteColor() -> Retained<CIColor>;

        #[method_id(@__retain_semantics Other grayColor)]
        pub unsafe fn grayColor() -> Retained<CIColor>;

        #[method_id(@__retain_semantics Other redColor)]
        pub unsafe fn redColor() -> Retained<CIColor>;

        #[method_id(@__retain_semantics Other greenColor)]
        pub unsafe fn greenColor() -> Retained<CIColor>;

        #[method_id(@__retain_semantics Other blueColor)]
        pub unsafe fn blueColor() -> Retained<CIColor>;

        #[method_id(@__retain_semantics Other cyanColor)]
        pub unsafe fn cyanColor() -> Retained<CIColor>;

        #[method_id(@__retain_semantics Other magentaColor)]
        pub unsafe fn magentaColor() -> Retained<CIColor>;

        #[method_id(@__retain_semantics Other yellowColor)]
        pub unsafe fn yellowColor() -> Retained<CIColor>;

        #[method_id(@__retain_semantics Other clearColor)]
        pub unsafe fn clearColor() -> Retained<CIColor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIColor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
