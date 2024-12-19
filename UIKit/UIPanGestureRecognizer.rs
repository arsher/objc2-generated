//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscrolltype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIScrollType(pub NSUInteger);
impl UIScrollType {
    #[doc(alias = "UIScrollTypeDiscrete")]
    pub const Discrete: Self = Self(0);
    #[doc(alias = "UIScrollTypeContinuous")]
    pub const Continuous: Self = Self(1);
}

unsafe impl Encode for UIScrollType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIScrollType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscrolltypemask?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIScrollTypeMask(pub NSInteger);
bitflags::bitflags! {
    impl UIScrollTypeMask: NSInteger {
        #[doc(alias = "UIScrollTypeMaskDiscrete")]
        const Discrete = 1<<UIScrollType::Discrete.0;
        #[doc(alias = "UIScrollTypeMaskContinuous")]
        const Continuous = 1<<UIScrollType::Continuous.0;
        #[doc(alias = "UIScrollTypeMaskAll")]
        const All = UIScrollTypeMask::Discrete.0|UIScrollTypeMask::Continuous.0;
    }
}

unsafe impl Encode for UIScrollTypeMask {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIScrollTypeMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipangesturerecognizer?language=objc)
    #[unsafe(super(UIGestureRecognizer, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGestureRecognizer")]
    pub struct UIPanGestureRecognizer;
);

#[cfg(feature = "UIGestureRecognizer")]
unsafe impl NSObjectProtocol for UIPanGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UIPanGestureRecognizer {
        #[method(minimumNumberOfTouches)]
        pub fn minimumNumberOfTouches(&self) -> NSUInteger;

        #[method(setMinimumNumberOfTouches:)]
        pub fn setMinimumNumberOfTouches(&self, minimum_number_of_touches: NSUInteger);

        #[method(maximumNumberOfTouches)]
        pub fn maximumNumberOfTouches(&self) -> NSUInteger;

        #[method(setMaximumNumberOfTouches:)]
        pub fn setMaximumNumberOfTouches(&self, maximum_number_of_touches: NSUInteger);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method(translationInView:)]
        pub fn translationInView(&self, view: Option<&UIView>) -> CGPoint;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method(setTranslation:inView:)]
        pub fn setTranslation_inView(&self, translation: CGPoint, view: Option<&UIView>);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method(velocityInView:)]
        pub fn velocityInView(&self, view: Option<&UIView>) -> CGPoint;

        #[method(allowedScrollTypesMask)]
        pub fn allowedScrollTypesMask(&self) -> UIScrollTypeMask;

        #[method(setAllowedScrollTypesMask:)]
        pub unsafe fn setAllowedScrollTypesMask(&self, allowed_scroll_types_mask: UIScrollTypeMask);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGestureRecognizer`
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UIPanGestureRecognizer {
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
    unsafe impl UIPanGestureRecognizer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
