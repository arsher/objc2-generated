//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpointzero?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static CGPointZero: CGPoint;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgsizezero?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static CGSizeZero: CGSize;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgrectzero?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static CGRectZero: CGRect;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgrectnull?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static CGRectNull: CGRect;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgrectinfinite?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static CGRectInfinite: CGRect;
}

// TODO: pub fn CGPointMake(x: CGFloat,y: CGFloat,) -> CGPoint;

// TODO: pub fn CGSizeMake(width: CGFloat,height: CGFloat,) -> CGSize;

// TODO: pub fn CGVectorMake(dx: CGFloat,dy: CGFloat,) -> CGVector;

// TODO: pub fn CGRectMake(x: CGFloat,y: CGFloat,width: CGFloat,height: CGFloat,) -> CGRect;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectGetMinX(rect: CGRect) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectGetMidX(rect: CGRect) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectGetMaxX(rect: CGRect) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectGetMinY(rect: CGRect) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectGetMidY(rect: CGRect) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectGetMaxY(rect: CGRect) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectGetWidth(rect: CGRect) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectGetHeight(rect: CGRect) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPointEqualToPoint(point1: CGPoint, point2: CGPoint) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGSizeEqualToSize(size1: CGSize, size2: CGSize) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectEqualToRect(rect1: CGRect, rect2: CGRect) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[must_use]
    pub fn CGRectStandardize(rect: CGRect) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectIsEmpty(rect: CGRect) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectIsNull(rect: CGRect) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectIsInfinite(rect: CGRect) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[must_use]
    pub fn CGRectInset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[must_use]
    pub fn CGRectIntegral(rect: CGRect) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[must_use]
    pub fn CGRectUnion(r1: CGRect, r2: CGRect) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[must_use]
    pub fn CGRectIntersection(r1: CGRect, r2: CGRect) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[must_use]
    pub fn CGRectOffset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectDivide(
        rect: CGRect,
        slice: NonNull<CGRect>,
        remainder: NonNull<CGRect>,
        amount: CGFloat,
        edge: CGRectEdge,
    );
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectContainsPoint(rect: CGRect, point: CGPoint) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectContainsRect(rect1: CGRect, rect2: CGRect) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectIntersectsRect(rect1: CGRect, rect2: CGRect) -> bool;
}

extern "C-unwind" {
    /// * Persistent representations. **
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPointCreateDictionaryRepresentation(point: CGPoint) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPointMakeWithDictionaryRepresentation(
        dict: CFDictionaryRef,
        point: *mut CGPoint,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGSizeCreateDictionaryRepresentation(size: CGSize) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGSizeMakeWithDictionaryRepresentation(dict: CFDictionaryRef, size: *mut CGSize)
        -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectCreateDictionaryRepresentation(_: CGRect) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGRectMakeWithDictionaryRepresentation(dict: CFDictionaryRef, rect: *mut CGRect)
        -> bool;
}

// TODO: pub fn CGPointMake(x: CGFloat,y: CGFloat,) -> CGPoint;

// TODO: pub fn CGSizeMake(width: CGFloat,height: CGFloat,) -> CGSize;

// TODO: pub fn CGVectorMake(dx: CGFloat,dy: CGFloat,) -> CGVector;

// TODO: pub fn CGRectMake(x: CGFloat,y: CGFloat,width: CGFloat,height: CGFloat,) -> CGRect;

// TODO: pub fn __CGPointEqualToPoint(point1: CGPoint,point2: CGPoint,) -> bool;

// TODO: pub fn __CGSizeEqualToSize(size1: CGSize,size2: CGSize,) -> bool;
