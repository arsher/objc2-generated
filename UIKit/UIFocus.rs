//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIFocusHeading(pub NSUInteger);
bitflags::bitflags! {
    impl UIFocusHeading: NSUInteger {
        #[doc(alias = "UIFocusHeadingNone")]
        const None = 0;
        #[doc(alias = "UIFocusHeadingUp")]
        const Up = 1<<0;
        #[doc(alias = "UIFocusHeadingDown")]
        const Down = 1<<1;
        #[doc(alias = "UIFocusHeadingLeft")]
        const Left = 1<<2;
        #[doc(alias = "UIFocusHeadingRight")]
        const Right = 1<<3;
        #[doc(alias = "UIFocusHeadingNext")]
        const Next = 1<<4;
        #[doc(alias = "UIFocusHeadingPrevious")]
        const Previous = 1<<5;
        #[doc(alias = "UIFocusHeadingFirst")]
        const First = 1<<8;
        #[doc(alias = "UIFocusHeadingLast")]
        const Last = 1<<9;
    }
}

unsafe impl Encode for UIFocusHeading {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIFocusHeading {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type UIFocusSoundIdentifier = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
pub type UIFocusGroupPriority = NSInteger;

pub static UIFocusGroupPriorityIgnored: UIFocusGroupPriority = 0;

pub static UIFocusGroupPriorityPreviouslyFocused: UIFocusGroupPriority = 1000;

pub static UIFocusGroupPriorityPrioritized: UIFocusGroupPriority = 2000;

pub static UIFocusGroupPriorityCurrentlyFocused: UIFocusGroupPriority = NSIntegerMax as _;

extern_protocol!(
    pub unsafe trait UIFocusEnvironment: NSObjectProtocol + IsMainThreadOnly {
        #[method_id(@__retain_semantics Other preferredFocusEnvironments)]
        unsafe fn preferredFocusEnvironments(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn UIFocusEnvironment>>>;

        #[method_id(@__retain_semantics Other parentFocusEnvironment)]
        unsafe fn parentFocusEnvironment(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIFocusEnvironment>>>;

        #[method_id(@__retain_semantics Other focusItemContainer)]
        unsafe fn focusItemContainer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIFocusItemContainer>>>;

        #[method(setNeedsFocusUpdate)]
        unsafe fn setNeedsFocusUpdate(&self);

        #[method(updateFocusIfNeeded)]
        unsafe fn updateFocusIfNeeded(&self);

        #[method(shouldUpdateFocusInContext:)]
        unsafe fn shouldUpdateFocusInContext(&self, context: &UIFocusUpdateContext) -> bool;

        #[cfg(feature = "UIFocusAnimationCoordinator")]
        #[method(didUpdateFocusInContext:withAnimationCoordinator:)]
        unsafe fn didUpdateFocusInContext_withAnimationCoordinator(
            &self,
            context: &UIFocusUpdateContext,
            coordinator: &UIFocusAnimationCoordinator,
        );

        #[optional]
        #[method_id(@__retain_semantics Other soundIdentifierForFocusUpdateInContext:)]
        unsafe fn soundIdentifierForFocusUpdateInContext(
            &self,
            context: &UIFocusUpdateContext,
        ) -> Option<Retained<UIFocusSoundIdentifier>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other preferredFocusedView)]
        unsafe fn preferredFocusedView(&self) -> Option<Retained<UIView>>;

        #[optional]
        #[method_id(@__retain_semantics Other focusGroupIdentifier)]
        unsafe fn focusGroupIdentifier(&self) -> Option<Retained<NSString>>;
    }

    unsafe impl ProtocolType for dyn UIFocusEnvironment {}
);

extern_protocol!(
    pub unsafe trait UIFocusItem: UIFocusEnvironment + IsMainThreadOnly {
        #[method(canBecomeFocused)]
        unsafe fn canBecomeFocused(&self) -> bool;

        #[method(frame)]
        unsafe fn frame(&self) -> CGRect;

        #[cfg(feature = "UIFocusEffect")]
        #[optional]
        #[method_id(@__retain_semantics Other focusEffect)]
        unsafe fn focusEffect(&self) -> Option<Retained<UIFocusEffect>>;

        #[optional]
        #[method(focusGroupPriority)]
        unsafe fn focusGroupPriority(&self) -> UIFocusGroupPriority;

        #[optional]
        #[method(isTransparentFocusItem)]
        unsafe fn isTransparentFocusItem(&self) -> bool;

        #[cfg(feature = "UIFocusMovementHint")]
        #[optional]
        #[method(didHintFocusMovement:)]
        unsafe fn didHintFocusMovement(&self, hint: &UIFocusMovementHint);
    }

    unsafe impl ProtocolType for dyn UIFocusItem {}
);

extern_protocol!(
    pub unsafe trait UIFocusItemContainer: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "UIView")]
        #[method_id(@__retain_semantics Other coordinateSpace)]
        unsafe fn coordinateSpace(&self) -> Retained<ProtocolObject<dyn UICoordinateSpace>>;

        #[method_id(@__retain_semantics Other focusItemsInRect:)]
        unsafe fn focusItemsInRect(
            &self,
            rect: CGRect,
        ) -> Retained<NSArray<ProtocolObject<dyn UIFocusItem>>>;
    }

    unsafe impl ProtocolType for dyn UIFocusItemContainer {}
);

extern_protocol!(
    pub unsafe trait UIFocusItemScrollableContainer:
        UIFocusItemContainer + IsMainThreadOnly
    {
        #[method(contentOffset)]
        unsafe fn contentOffset(&self) -> CGPoint;

        #[method(setContentOffset:)]
        unsafe fn setContentOffset(&self, content_offset: CGPoint);

        #[method(contentSize)]
        unsafe fn contentSize(&self) -> CGSize;

        #[method(visibleSize)]
        unsafe fn visibleSize(&self) -> CGSize;
    }

    unsafe impl ProtocolType for dyn UIFocusItemScrollableContainer {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFocusUpdateContext;

    unsafe impl ClassType for UIFocusUpdateContext {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIFocusUpdateContext {}

extern_methods!(
    unsafe impl UIFocusUpdateContext {
        #[method_id(@__retain_semantics Other previouslyFocusedItem)]
        pub unsafe fn previouslyFocusedItem(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIFocusItem>>>;

        #[method_id(@__retain_semantics Other nextFocusedItem)]
        pub unsafe fn nextFocusedItem(&self) -> Option<Retained<ProtocolObject<dyn UIFocusItem>>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other previouslyFocusedView)]
        pub unsafe fn previouslyFocusedView(&self) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other nextFocusedView)]
        pub unsafe fn nextFocusedView(&self) -> Option<Retained<UIView>>;

        #[method(focusHeading)]
        pub unsafe fn focusHeading(&self) -> UIFocusHeading;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIFocusUpdateContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    pub static UIFocusDidUpdateNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIFocusMovementDidFailNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UIFocusUpdateContextKey: &'static NSString;
}

extern "C" {
    pub static UIFocusUpdateAnimationCoordinatorKey: &'static NSString;
}

extern "C" {
    pub static UIFocusSoundIdentifierNone: &'static UIFocusSoundIdentifier;
}

extern "C" {
    pub static UIFocusSoundIdentifierDefault: &'static UIFocusSoundIdentifier;
}
