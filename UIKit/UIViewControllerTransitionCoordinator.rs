//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitransitioncontextviewcontrollerkey?language=objc)
// NS_TYPED_ENUM
pub type UITransitionContextViewControllerKey = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitransitioncontextviewkey?language=objc)
// NS_TYPED_ENUM
pub type UITransitionContextViewKey = NSString;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewcontrollertransitioncoordinatorcontext?language=objc)
    pub unsafe trait UIViewControllerTransitionCoordinatorContext:
        NSObjectProtocol + MainThreadOnly
    {
        #[method(isAnimated)]
        unsafe fn isAnimated(&self) -> bool;

        #[cfg(feature = "UIViewController")]
        #[method(presentationStyle)]
        unsafe fn presentationStyle(&self) -> UIModalPresentationStyle;

        #[method(initiallyInteractive)]
        unsafe fn initiallyInteractive(&self) -> bool;

        #[method(isInterruptible)]
        unsafe fn isInterruptible(&self) -> bool;

        #[method(isInteractive)]
        unsafe fn isInteractive(&self) -> bool;

        #[method(isCancelled)]
        unsafe fn isCancelled(&self) -> bool;

        #[method(transitionDuration)]
        unsafe fn transitionDuration(&self) -> NSTimeInterval;

        #[method(percentComplete)]
        unsafe fn percentComplete(&self) -> CGFloat;

        #[method(completionVelocity)]
        unsafe fn completionVelocity(&self) -> CGFloat;

        #[cfg(feature = "UIView")]
        #[method(completionCurve)]
        unsafe fn completionCurve(&self) -> UIViewAnimationCurve;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other viewControllerForKey:)]
        unsafe fn viewControllerForKey(
            &self,
            key: &UITransitionContextViewControllerKey,
        ) -> Option<Retained<UIViewController>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other viewForKey:)]
        unsafe fn viewForKey(&self, key: &UITransitionContextViewKey) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other containerView)]
        unsafe fn containerView(&self) -> Retained<UIView>;

        #[method(targetTransform)]
        unsafe fn targetTransform(&self) -> CGAffineTransform;
    }

    unsafe impl ProtocolType for dyn UIViewControllerTransitionCoordinatorContext {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewcontrollertransitioncoordinator?language=objc)
    pub unsafe trait UIViewControllerTransitionCoordinator:
        UIViewControllerTransitionCoordinatorContext + MainThreadOnly
    {
        #[cfg(feature = "block2")]
        #[method(animateAlongsideTransition:completion:)]
        unsafe fn animateAlongsideTransition_completion(
            &self,
            animation: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>,
                    ),
                >,
            >,
            completion: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>,
                    ),
                >,
            >,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "block2"))]
        #[method(animateAlongsideTransitionInView:animation:completion:)]
        unsafe fn animateAlongsideTransitionInView_animation_completion(
            &self,
            view: Option<&UIView>,
            animation: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>,
                    ),
                >,
            >,
            completion: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>,
                    ),
                >,
            >,
        ) -> bool;

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(notifyWhenInteractionEndsUsingBlock:)]
        unsafe fn notifyWhenInteractionEndsUsingBlock(
            &self,
            handler: &block2::Block<
                dyn Fn(NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>),
            >,
        );

        #[cfg(feature = "block2")]
        #[method(notifyWhenInteractionChangesUsingBlock:)]
        unsafe fn notifyWhenInteractionChangesUsingBlock(
            &self,
            handler: &block2::Block<
                dyn Fn(NonNull<ProtocolObject<dyn UIViewControllerTransitionCoordinatorContext>>),
            >,
        );
    }

    unsafe impl ProtocolType for dyn UIViewControllerTransitionCoordinator {}
);

extern_methods!(
    /// UIViewControllerTransitionCoordinator
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIViewController {
        #[method_id(@__retain_semantics Other transitionCoordinator)]
        pub unsafe fn transitionCoordinator(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIViewControllerTransitionCoordinator>>>;
    }
);
