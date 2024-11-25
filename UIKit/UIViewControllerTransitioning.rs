//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitransitioncontextfromviewcontrollerkey?language=objc)
    #[cfg(feature = "UIViewControllerTransitionCoordinator")]
    pub static UITransitionContextFromViewControllerKey:
        &'static UITransitionContextViewControllerKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitransitioncontexttoviewcontrollerkey?language=objc)
    #[cfg(feature = "UIViewControllerTransitionCoordinator")]
    pub static UITransitionContextToViewControllerKey:
        &'static UITransitionContextViewControllerKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitransitioncontextfromviewkey?language=objc)
    #[cfg(feature = "UIViewControllerTransitionCoordinator")]
    pub static UITransitionContextFromViewKey: &'static UITransitionContextViewKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitransitioncontexttoviewkey?language=objc)
    #[cfg(feature = "UIViewControllerTransitionCoordinator")]
    pub static UITransitionContextToViewKey: &'static UITransitionContextViewKey;
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewcontrollercontexttransitioning?language=objc)
    pub unsafe trait UIViewControllerContextTransitioning:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other containerView)]
        unsafe fn containerView(&self) -> Retained<UIView>;

        #[method(isAnimated)]
        unsafe fn isAnimated(&self) -> bool;

        #[method(isInteractive)]
        unsafe fn isInteractive(&self) -> bool;

        #[method(transitionWasCancelled)]
        unsafe fn transitionWasCancelled(&self) -> bool;

        #[cfg(feature = "UIViewController")]
        #[method(presentationStyle)]
        unsafe fn presentationStyle(&self) -> UIModalPresentationStyle;

        #[method(updateInteractiveTransition:)]
        unsafe fn updateInteractiveTransition(&self, percent_complete: CGFloat);

        #[method(finishInteractiveTransition)]
        unsafe fn finishInteractiveTransition(&self);

        #[method(cancelInteractiveTransition)]
        unsafe fn cancelInteractiveTransition(&self);

        #[method(pauseInteractiveTransition)]
        unsafe fn pauseInteractiveTransition(&self);

        #[method(completeTransition:)]
        unsafe fn completeTransition(&self, did_complete: bool);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "UIViewControllerTransitionCoordinator"
        ))]
        #[method_id(@__retain_semantics Other viewControllerForKey:)]
        unsafe fn viewControllerForKey(
            &self,
            key: &UITransitionContextViewControllerKey,
        ) -> Option<Retained<UIViewController>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "UIViewControllerTransitionCoordinator"
        ))]
        #[method_id(@__retain_semantics Other viewForKey:)]
        unsafe fn viewForKey(&self, key: &UITransitionContextViewKey) -> Option<Retained<UIView>>;

        #[method(targetTransform)]
        unsafe fn targetTransform(&self) -> CGAffineTransform;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method(initialFrameForViewController:)]
        unsafe fn initialFrameForViewController(&self, vc: &UIViewController) -> CGRect;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method(finalFrameForViewController:)]
        unsafe fn finalFrameForViewController(&self, vc: &UIViewController) -> CGRect;
    }

    unsafe impl ProtocolType for dyn UIViewControllerContextTransitioning {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewcontrolleranimatedtransitioning?language=objc)
    pub unsafe trait UIViewControllerAnimatedTransitioning:
        NSObjectProtocol + MainThreadOnly
    {
        #[method(transitionDuration:)]
        unsafe fn transitionDuration(
            &self,
            transition_context: Option<&ProtocolObject<dyn UIViewControllerContextTransitioning>>,
        ) -> NSTimeInterval;

        #[method(animateTransition:)]
        unsafe fn animateTransition(
            &self,
            transition_context: &ProtocolObject<dyn UIViewControllerContextTransitioning>,
        );

        #[cfg(feature = "UIViewAnimating")]
        #[optional]
        #[method_id(@__retain_semantics Other interruptibleAnimatorForTransition:)]
        unsafe fn interruptibleAnimatorForTransition(
            &self,
            transition_context: &ProtocolObject<dyn UIViewControllerContextTransitioning>,
        ) -> Retained<ProtocolObject<dyn UIViewImplicitlyAnimating>>;

        #[optional]
        #[method(animationEnded:)]
        unsafe fn animationEnded(&self, transition_completed: bool);
    }

    unsafe impl ProtocolType for dyn UIViewControllerAnimatedTransitioning {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewcontrollerinteractivetransitioning?language=objc)
    pub unsafe trait UIViewControllerInteractiveTransitioning:
        NSObjectProtocol + MainThreadOnly
    {
        #[method(startInteractiveTransition:)]
        unsafe fn startInteractiveTransition(
            &self,
            transition_context: &ProtocolObject<dyn UIViewControllerContextTransitioning>,
        );

        #[optional]
        #[method(completionSpeed)]
        unsafe fn completionSpeed(&self) -> CGFloat;

        #[cfg(feature = "UIView")]
        #[optional]
        #[method(completionCurve)]
        unsafe fn completionCurve(&self) -> UIViewAnimationCurve;

        #[optional]
        #[method(wantsInteractiveStart)]
        unsafe fn wantsInteractiveStart(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn UIViewControllerInteractiveTransitioning {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewcontrollertransitioningdelegate?language=objc)
    pub unsafe trait UIViewControllerTransitioningDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other animationControllerForPresentedController:presentingController:sourceController:)]
        unsafe fn animationControllerForPresentedController_presentingController_sourceController(
            &self,
            presented: &UIViewController,
            presenting: &UIViewController,
            source: &UIViewController,
        ) -> Option<Retained<ProtocolObject<dyn UIViewControllerAnimatedTransitioning>>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other animationControllerForDismissedController:)]
        unsafe fn animationControllerForDismissedController(
            &self,
            dismissed: &UIViewController,
        ) -> Option<Retained<ProtocolObject<dyn UIViewControllerAnimatedTransitioning>>>;

        #[optional]
        #[method_id(@__retain_semantics Other interactionControllerForPresentation:)]
        unsafe fn interactionControllerForPresentation(
            &self,
            animator: &ProtocolObject<dyn UIViewControllerAnimatedTransitioning>,
        ) -> Option<Retained<ProtocolObject<dyn UIViewControllerInteractiveTransitioning>>>;

        #[optional]
        #[method_id(@__retain_semantics Other interactionControllerForDismissal:)]
        unsafe fn interactionControllerForDismissal(
            &self,
            animator: &ProtocolObject<dyn UIViewControllerAnimatedTransitioning>,
        ) -> Option<Retained<ProtocolObject<dyn UIViewControllerInteractiveTransitioning>>>;

        #[cfg(all(
            feature = "UIPresentationController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other presentationControllerForPresentedViewController:presentingViewController:sourceViewController:)]
        unsafe fn presentationControllerForPresentedViewController_presentingViewController_sourceViewController(
            &self,
            presented: &UIViewController,
            presenting: Option<&UIViewController>,
            source: &UIViewController,
        ) -> Option<Retained<UIPresentationController>>;
    }

    unsafe impl ProtocolType for dyn UIViewControllerTransitioningDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipercentdriveninteractivetransition?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPercentDrivenInteractiveTransition;
);

unsafe impl NSObjectProtocol for UIPercentDrivenInteractiveTransition {}

unsafe impl UIViewControllerInteractiveTransitioning for UIPercentDrivenInteractiveTransition {}

extern_methods!(
    unsafe impl UIPercentDrivenInteractiveTransition {
        #[method(duration)]
        pub unsafe fn duration(&self) -> CGFloat;

        #[method(percentComplete)]
        pub unsafe fn percentComplete(&self) -> CGFloat;

        #[method(completionSpeed)]
        pub unsafe fn completionSpeed(&self) -> CGFloat;

        #[method(setCompletionSpeed:)]
        pub unsafe fn setCompletionSpeed(&self, completion_speed: CGFloat);

        #[cfg(feature = "UIView")]
        #[method(completionCurve)]
        pub unsafe fn completionCurve(&self) -> UIViewAnimationCurve;

        #[cfg(feature = "UIView")]
        #[method(setCompletionCurve:)]
        pub unsafe fn setCompletionCurve(&self, completion_curve: UIViewAnimationCurve);

        #[cfg(feature = "UITimingCurveProvider")]
        #[method_id(@__retain_semantics Other timingCurve)]
        pub unsafe fn timingCurve(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UITimingCurveProvider>>>;

        #[cfg(feature = "UITimingCurveProvider")]
        #[method(setTimingCurve:)]
        pub unsafe fn setTimingCurve(
            &self,
            timing_curve: Option<&ProtocolObject<dyn UITimingCurveProvider>>,
        );

        #[method(wantsInteractiveStart)]
        pub unsafe fn wantsInteractiveStart(&self) -> bool;

        #[method(setWantsInteractiveStart:)]
        pub unsafe fn setWantsInteractiveStart(&self, wants_interactive_start: bool);

        #[method(pauseInteractiveTransition)]
        pub unsafe fn pauseInteractiveTransition(&self);

        #[method(updateInteractiveTransition:)]
        pub unsafe fn updateInteractiveTransition(&self, percent_complete: CGFloat);

        #[method(cancelInteractiveTransition)]
        pub unsafe fn cancelInteractiveTransition(&self);

        #[method(finishInteractiveTransition)]
        pub unsafe fn finishInteractiveTransition(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPercentDrivenInteractiveTransition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
