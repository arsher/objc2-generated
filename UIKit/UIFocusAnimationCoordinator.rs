//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifocusanimationcontext?language=objc)
    pub unsafe trait UIFocusAnimationContext: NSObjectProtocol + MainThreadOnly {
        #[method(duration)]
        unsafe fn duration(&self) -> NSTimeInterval;
    }

    unsafe impl ProtocolType for dyn UIFocusAnimationContext {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifocusanimationcoordinator?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFocusAnimationCoordinator;
);

unsafe impl NSObjectProtocol for UIFocusAnimationCoordinator {}

extern_methods!(
    unsafe impl UIFocusAnimationCoordinator {
        #[cfg(feature = "block2")]
        #[method(addCoordinatedAnimations:completion:)]
        pub unsafe fn addCoordinatedAnimations_completion(
            &self,
            animations: Option<&block2::Block<dyn Fn()>>,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "block2")]
        #[method(addCoordinatedFocusingAnimations:completion:)]
        pub unsafe fn addCoordinatedFocusingAnimations_completion(
            &self,
            animations: Option<
                &block2::Block<dyn Fn(NonNull<ProtocolObject<dyn UIFocusAnimationContext>>)>,
            >,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "block2")]
        #[method(addCoordinatedUnfocusingAnimations:completion:)]
        pub unsafe fn addCoordinatedUnfocusingAnimations_completion(
            &self,
            animations: Option<
                &block2::Block<dyn Fn(NonNull<ProtocolObject<dyn UIFocusAnimationContext>>)>,
            >,
            completion: Option<&block2::Block<dyn Fn()>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIFocusAnimationCoordinator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
