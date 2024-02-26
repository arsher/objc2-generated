//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_category!(
    /// Category "NSAppKitAdditions" on [`NSAffineTransform`].
    #[doc(alias = "NSAppKitAdditions")]
    pub unsafe trait NSAffineTransformNSAppKitAdditions {
        #[cfg(feature = "AppKit_NSBezierPath")]
        #[method_id(@__retain_semantics Other transformBezierPath:)]
        unsafe fn transformBezierPath(&self, path: &NSBezierPath) -> Id<NSBezierPath>;

        #[method(set)]
        unsafe fn set(&self);

        #[method(concat)]
        unsafe fn concat(&self);
    }

    #[cfg(feature = "Foundation_NSAffineTransform")]
    unsafe impl NSAffineTransformNSAppKitAdditions for NSAffineTransform {}
);
