//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilongpressgesturerecognizer?language=objc)
    #[unsafe(super(UIGestureRecognizer, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGestureRecognizer")]
    pub struct UILongPressGestureRecognizer;
);

#[cfg(feature = "UIGestureRecognizer")]
unsafe impl NSObjectProtocol for UILongPressGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UILongPressGestureRecognizer {
        #[method(numberOfTapsRequired)]
        pub unsafe fn numberOfTapsRequired(&self) -> NSUInteger;

        #[method(setNumberOfTapsRequired:)]
        pub unsafe fn setNumberOfTapsRequired(&self, number_of_taps_required: NSUInteger);

        #[method(numberOfTouchesRequired)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSUInteger;

        #[method(setNumberOfTouchesRequired:)]
        pub unsafe fn setNumberOfTouchesRequired(&self, number_of_touches_required: NSUInteger);

        #[method(minimumPressDuration)]
        pub unsafe fn minimumPressDuration(&self) -> NSTimeInterval;

        #[method(setMinimumPressDuration:)]
        pub unsafe fn setMinimumPressDuration(&self, minimum_press_duration: NSTimeInterval);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(allowableMovement)]
        pub unsafe fn allowableMovement(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setAllowableMovement:)]
        pub unsafe fn setAllowableMovement(&self, allowable_movement: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGestureRecognizer`
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UILongPressGestureRecognizer {
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
    unsafe impl UILongPressGestureRecognizer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
