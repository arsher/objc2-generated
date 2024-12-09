//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewpropertyanimator?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIViewPropertyAnimator;
);

unsafe impl NSCopying for UIViewPropertyAnimator {}

unsafe impl CopyingHelper for UIViewPropertyAnimator {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIViewPropertyAnimator {}

#[cfg(feature = "UIViewAnimating")]
unsafe impl UIViewAnimating for UIViewPropertyAnimator {}

#[cfg(feature = "UIViewAnimating")]
unsafe impl UIViewImplicitlyAnimating for UIViewPropertyAnimator {}

extern_methods!(
    unsafe impl UIViewPropertyAnimator {
        #[cfg(feature = "UITimingCurveProvider")]
        #[method_id(@__retain_semantics Other timingParameters)]
        pub unsafe fn timingParameters(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UITimingCurveProvider>>>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(delay)]
        pub unsafe fn delay(&self) -> NSTimeInterval;

        #[method(isUserInteractionEnabled)]
        pub unsafe fn isUserInteractionEnabled(&self) -> bool;

        #[method(setUserInteractionEnabled:)]
        pub unsafe fn setUserInteractionEnabled(&self, user_interaction_enabled: bool);

        #[method(isManualHitTestingEnabled)]
        pub unsafe fn isManualHitTestingEnabled(&self) -> bool;

        #[method(setManualHitTestingEnabled:)]
        pub unsafe fn setManualHitTestingEnabled(&self, manual_hit_testing_enabled: bool);

        #[method(isInterruptible)]
        pub unsafe fn isInterruptible(&self) -> bool;

        #[method(setInterruptible:)]
        pub unsafe fn setInterruptible(&self, interruptible: bool);

        #[method(scrubsLinearly)]
        pub unsafe fn scrubsLinearly(&self) -> bool;

        #[method(setScrubsLinearly:)]
        pub unsafe fn setScrubsLinearly(&self, scrubs_linearly: bool);

        #[method(pausesOnCompletion)]
        pub unsafe fn pausesOnCompletion(&self) -> bool;

        #[method(setPausesOnCompletion:)]
        pub unsafe fn setPausesOnCompletion(&self, pauses_on_completion: bool);

        #[cfg(feature = "UITimingCurveProvider")]
        #[method_id(@__retain_semantics Init initWithDuration:timingParameters:)]
        pub unsafe fn initWithDuration_timingParameters(
            this: Allocated<Self>,
            duration: NSTimeInterval,
            parameters: &ProtocolObject<dyn UITimingCurveProvider>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIView", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithDuration:curve:animations:)]
        pub unsafe fn initWithDuration_curve_animations(
            this: Allocated<Self>,
            duration: NSTimeInterval,
            curve: UIViewAnimationCurve,
            animations: Option<&block2::Block<dyn Fn()>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
        #[method_id(@__retain_semantics Init initWithDuration:controlPoint1:controlPoint2:animations:)]
        pub unsafe fn initWithDuration_controlPoint1_controlPoint2_animations(
            this: Allocated<Self>,
            duration: NSTimeInterval,
            point1: CGPoint,
            point2: CGPoint,
            animations: Option<&block2::Block<dyn Fn()>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
        #[method_id(@__retain_semantics Init initWithDuration:dampingRatio:animations:)]
        pub unsafe fn initWithDuration_dampingRatio_animations(
            this: Allocated<Self>,
            duration: NSTimeInterval,
            ratio: CGFloat,
            animations: Option<&block2::Block<dyn Fn()>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIView", feature = "UIViewAnimating", feature = "block2"))]
        #[method_id(@__retain_semantics Other runningPropertyAnimatorWithDuration:delay:options:animations:completion:)]
        pub unsafe fn runningPropertyAnimatorWithDuration_delay_options_animations_completion(
            duration: NSTimeInterval,
            delay: NSTimeInterval,
            options: UIViewAnimationOptions,
            animations: &block2::Block<dyn Fn()>,
            completion: Option<&block2::Block<dyn Fn(UIViewAnimatingPosition)>>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
        #[method(addAnimations:delayFactor:)]
        pub unsafe fn addAnimations_delayFactor(
            &self,
            animation: &block2::Block<dyn Fn()>,
            delay_factor: CGFloat,
        );

        #[cfg(feature = "block2")]
        #[method(addAnimations:)]
        pub unsafe fn addAnimations(&self, animation: &block2::Block<dyn Fn()>);

        #[cfg(all(feature = "UIViewAnimating", feature = "block2"))]
        #[method(addCompletion:)]
        pub unsafe fn addCompletion(
            &self,
            completion: &block2::Block<dyn Fn(UIViewAnimatingPosition)>,
        );

        #[cfg(all(feature = "UITimingCurveProvider", feature = "objc2-core-foundation"))]
        #[method(continueAnimationWithTimingParameters:durationFactor:)]
        pub unsafe fn continueAnimationWithTimingParameters_durationFactor(
            &self,
            parameters: Option<&ProtocolObject<dyn UITimingCurveProvider>>,
            duration_factor: CGFloat,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIViewPropertyAnimator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
