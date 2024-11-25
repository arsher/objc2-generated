//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/civector?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIVector;
);

unsafe impl NSCoding for CIVector {}

unsafe impl NSCopying for CIVector {}

unsafe impl CopyingHelper for CIVector {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIVector {}

unsafe impl NSSecureCoding for CIVector {}

extern_methods!(
    unsafe impl CIVector {
        #[method_id(@__retain_semantics Other vectorWithValues:count:)]
        pub unsafe fn vectorWithValues_count(
            values: NonNull<CGFloat>,
            count: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other vectorWithX:)]
        pub unsafe fn vectorWithX(x: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Other vectorWithX:Y:)]
        pub unsafe fn vectorWithX_Y(x: CGFloat, y: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Other vectorWithX:Y:Z:)]
        pub unsafe fn vectorWithX_Y_Z(x: CGFloat, y: CGFloat, z: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Other vectorWithX:Y:Z:W:)]
        pub unsafe fn vectorWithX_Y_Z_W(
            x: CGFloat,
            y: CGFloat,
            z: CGFloat,
            w: CGFloat,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other vectorWithCGPoint:)]
        pub unsafe fn vectorWithCGPoint(p: CGPoint) -> Retained<Self>;

        #[method_id(@__retain_semantics Other vectorWithCGRect:)]
        pub unsafe fn vectorWithCGRect(r: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Other vectorWithString:)]
        pub unsafe fn vectorWithString(representation: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithValues:count:)]
        pub unsafe fn initWithValues_count(
            this: Allocated<Self>,
            values: NonNull<CGFloat>,
            count: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithX:)]
        pub unsafe fn initWithX(this: Allocated<Self>, x: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithX:Y:)]
        pub unsafe fn initWithX_Y(this: Allocated<Self>, x: CGFloat, y: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithX:Y:Z:)]
        pub unsafe fn initWithX_Y_Z(
            this: Allocated<Self>,
            x: CGFloat,
            y: CGFloat,
            z: CGFloat,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithX:Y:Z:W:)]
        pub unsafe fn initWithX_Y_Z_W(
            this: Allocated<Self>,
            x: CGFloat,
            y: CGFloat,
            z: CGFloat,
            w: CGFloat,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCGPoint:)]
        pub unsafe fn initWithCGPoint(this: Allocated<Self>, p: CGPoint) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCGRect:)]
        pub unsafe fn initWithCGRect(this: Allocated<Self>, r: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Allocated<Self>,
            representation: &NSString,
        ) -> Retained<Self>;

        #[method(valueAtIndex:)]
        pub unsafe fn valueAtIndex(&self, index: usize) -> CGFloat;

        #[method(count)]
        pub unsafe fn count(&self) -> usize;

        #[method(X)]
        pub unsafe fn X(&self) -> CGFloat;

        #[method(Y)]
        pub unsafe fn Y(&self) -> CGFloat;

        #[method(Z)]
        pub unsafe fn Z(&self) -> CGFloat;

        #[method(W)]
        pub unsafe fn W(&self) -> CGFloat;

        #[method(CGPointValue)]
        pub unsafe fn CGPointValue(&self) -> CGPoint;

        #[method(CGRectValue)]
        pub unsafe fn CGRectValue(&self) -> CGRect;

        #[method_id(@__retain_semantics Other stringRepresentation)]
        pub unsafe fn stringRepresentation(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIVector {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
