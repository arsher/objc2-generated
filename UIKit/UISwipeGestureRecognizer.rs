//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiswipegesturerecognizerdirection?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISwipeGestureRecognizerDirection(pub NSUInteger);
bitflags::bitflags! {
    impl UISwipeGestureRecognizerDirection: NSUInteger {
        #[doc(alias = "UISwipeGestureRecognizerDirectionRight")]
        const Right = 1<<0;
        #[doc(alias = "UISwipeGestureRecognizerDirectionLeft")]
        const Left = 1<<1;
        #[doc(alias = "UISwipeGestureRecognizerDirectionUp")]
        const Up = 1<<2;
        #[doc(alias = "UISwipeGestureRecognizerDirectionDown")]
        const Down = 1<<3;
    }
}

unsafe impl Encode for UISwipeGestureRecognizerDirection {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UISwipeGestureRecognizerDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiswipegesturerecognizer?language=objc)
    #[unsafe(super(UIGestureRecognizer, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGestureRecognizer")]
    pub struct UISwipeGestureRecognizer;
);

#[cfg(feature = "UIGestureRecognizer")]
unsafe impl NSObjectProtocol for UISwipeGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UISwipeGestureRecognizer {
        #[method(numberOfTouchesRequired)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSUInteger;

        #[method(setNumberOfTouchesRequired:)]
        pub unsafe fn setNumberOfTouchesRequired(&self, number_of_touches_required: NSUInteger);

        #[method(direction)]
        pub unsafe fn direction(&self) -> UISwipeGestureRecognizerDirection;

        #[method(setDirection:)]
        pub unsafe fn setDirection(&self, direction: UISwipeGestureRecognizerDirection);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGestureRecognizer`
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UISwipeGestureRecognizer {
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
    unsafe impl UISwipeGestureRecognizer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
