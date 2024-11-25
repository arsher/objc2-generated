//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
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
        #[method(rotation)]
        pub unsafe fn rotation(&self) -> CGFloat;

        #[method(setRotation:)]
        pub unsafe fn setRotation(&self, rotation: CGFloat);

        #[method(rotationInDegrees)]
        pub unsafe fn rotationInDegrees(&self) -> CGFloat;

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
