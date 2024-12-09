//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrotationgesturerecognizer?language=objc)
    #[unsafe(super(NSGestureRecognizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSGestureRecognizer")]
    pub struct NSRotationGestureRecognizer;
);

#[cfg(feature = "NSGestureRecognizer")]
unsafe impl NSCoding for NSRotationGestureRecognizer {}

#[cfg(feature = "NSGestureRecognizer")]
unsafe impl NSObjectProtocol for NSRotationGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "NSGestureRecognizer")]
    unsafe impl NSRotationGestureRecognizer {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(rotation)]
        pub unsafe fn rotation(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setRotation:)]
        pub unsafe fn setRotation(&self, rotation: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rotationInDegrees)]
        pub unsafe fn rotationInDegrees(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setRotationInDegrees:)]
        pub unsafe fn setRotationInDegrees(&self, rotation_in_degrees: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSGestureRecognizer`
    #[cfg(feature = "NSGestureRecognizer")]
    unsafe impl NSRotationGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Allocated<Self>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSGestureRecognizer")]
    unsafe impl NSRotationGestureRecognizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
