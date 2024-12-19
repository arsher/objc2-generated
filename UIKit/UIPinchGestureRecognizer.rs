//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipinchgesturerecognizer?language=objc)
    #[unsafe(super(UIGestureRecognizer, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGestureRecognizer")]
    pub struct UIPinchGestureRecognizer;
);

#[cfg(feature = "UIGestureRecognizer")]
unsafe impl NSObjectProtocol for UIPinchGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UIPinchGestureRecognizer {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(scale)]
        pub fn scale(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setScale:)]
        pub unsafe fn setScale(&self, scale: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(velocity)]
        pub fn velocity(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGestureRecognizer`
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UIPinchGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Allocated<Self>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UIPinchGestureRecognizer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
