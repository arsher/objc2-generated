//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSGestureRecognizer")]
    pub struct NSPressGestureRecognizer;

    #[cfg(feature = "NSGestureRecognizer")]
    unsafe impl ClassType for NSPressGestureRecognizer {
        #[inherits(NSObject)]
        type Super = NSGestureRecognizer;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "NSGestureRecognizer")]
unsafe impl NSCoding for NSPressGestureRecognizer {}

#[cfg(feature = "NSGestureRecognizer")]
unsafe impl NSObjectProtocol for NSPressGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "NSGestureRecognizer")]
    unsafe impl NSPressGestureRecognizer {
        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSUInteger;

        #[method(setButtonMask:)]
        pub unsafe fn setButtonMask(&self, button_mask: NSUInteger);

        #[method(minimumPressDuration)]
        pub unsafe fn minimumPressDuration(&self) -> NSTimeInterval;

        #[method(setMinimumPressDuration:)]
        pub unsafe fn setMinimumPressDuration(&self, minimum_press_duration: NSTimeInterval);

        #[method(allowableMovement)]
        pub unsafe fn allowableMovement(&self) -> CGFloat;

        #[method(setAllowableMovement:)]
        pub unsafe fn setAllowableMovement(&self, allowable_movement: CGFloat);

        #[method(numberOfTouchesRequired)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSInteger;

        #[method(setNumberOfTouchesRequired:)]
        pub unsafe fn setNumberOfTouchesRequired(&self, number_of_touches_required: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSGestureRecognizer`
    #[cfg(feature = "NSGestureRecognizer")]
    unsafe impl NSPressGestureRecognizer {
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
    unsafe impl NSPressGestureRecognizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
