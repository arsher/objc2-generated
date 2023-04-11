//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPressGestureRecognizer")]
    pub struct NSPressGestureRecognizer;

    #[cfg(feature = "AppKit_NSPressGestureRecognizer")]
    unsafe impl ClassType for NSPressGestureRecognizer {
        #[inherits(NSObject)]
        type Super = NSGestureRecognizer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSPressGestureRecognizer")]
unsafe impl NSCoding for NSPressGestureRecognizer {}

#[cfg(feature = "AppKit_NSPressGestureRecognizer")]
unsafe impl NSObjectProtocol for NSPressGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPressGestureRecognizer")]
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
    #[cfg(feature = "AppKit_NSPressGestureRecognizer")]
    unsafe impl NSPressGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Option<Allocated<Self>>,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self>;
    }
);
