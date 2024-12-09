//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caanimationcalculationmode?language=objc)
// NS_TYPED_ENUM
pub type CAAnimationCalculationMode = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caanimationrotationmode?language=objc)
// NS_TYPED_ENUM
pub type CAAnimationRotationMode = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/catransitiontype?language=objc)
// NS_TYPED_ENUM
pub type CATransitionType = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/catransitionsubtype?language=objc)
// NS_TYPED_ENUM
pub type CATransitionSubtype = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caanimation?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAAnimation;
);

#[cfg(feature = "CALayer")]
unsafe impl CAAction for CAAnimation {}

#[cfg(feature = "CAMediaTiming")]
unsafe impl CAMediaTiming for CAAnimation {}

unsafe impl NSCoding for CAAnimation {}

unsafe impl NSCopying for CAAnimation {}

unsafe impl CopyingHelper for CAAnimation {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CAAnimation {}

unsafe impl NSSecureCoding for CAAnimation {}

extern_methods!(
    unsafe impl CAAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Retained<Self>;

        #[method_id(@__retain_semantics Other defaultValueForKey:)]
        pub unsafe fn defaultValueForKey(key: &NSString) -> Option<Retained<AnyObject>>;

        #[method(shouldArchiveValueForKey:)]
        pub unsafe fn shouldArchiveValueForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "CAMediaTimingFunction")]
        #[method_id(@__retain_semantics Other timingFunction)]
        pub unsafe fn timingFunction(&self) -> Option<Retained<CAMediaTimingFunction>>;

        #[cfg(feature = "CAMediaTimingFunction")]
        #[method(setTimingFunction:)]
        pub unsafe fn setTimingFunction(&self, timing_function: Option<&CAMediaTimingFunction>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn CAAnimationDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CAAnimationDelegate>>,
        );

        #[method(isRemovedOnCompletion)]
        pub unsafe fn isRemovedOnCompletion(&self) -> bool;

        #[method(setRemovedOnCompletion:)]
        pub unsafe fn setRemovedOnCompletion(&self, removed_on_completion: bool);

        #[cfg(feature = "CAFrameRateRange")]
        #[method(preferredFrameRateRange)]
        pub unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange;

        #[cfg(feature = "CAFrameRateRange")]
        #[method(setPreferredFrameRateRange:)]
        pub unsafe fn setPreferredFrameRateRange(
            &self,
            preferred_frame_rate_range: CAFrameRateRange,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAAnimation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caanimationdelegate?language=objc)
    pub unsafe trait CAAnimationDelegate: NSObjectProtocol {
        #[optional]
        #[method(animationDidStart:)]
        unsafe fn animationDidStart(&self, anim: &CAAnimation);

        #[optional]
        #[method(animationDidStop:finished:)]
        unsafe fn animationDidStop_finished(&self, anim: &CAAnimation, flag: bool);
    }

    unsafe impl ProtocolType for dyn CAAnimationDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/capropertyanimation?language=objc)
    #[unsafe(super(CAAnimation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAPropertyAnimation;
);

#[cfg(feature = "CALayer")]
unsafe impl CAAction for CAPropertyAnimation {}

#[cfg(feature = "CAMediaTiming")]
unsafe impl CAMediaTiming for CAPropertyAnimation {}

unsafe impl NSCoding for CAPropertyAnimation {}

unsafe impl NSCopying for CAPropertyAnimation {}

unsafe impl CopyingHelper for CAPropertyAnimation {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CAPropertyAnimation {}

unsafe impl NSSecureCoding for CAPropertyAnimation {}

extern_methods!(
    unsafe impl CAPropertyAnimation {
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other keyPath)]
        pub unsafe fn keyPath(&self) -> Option<Retained<NSString>>;

        #[method(setKeyPath:)]
        pub unsafe fn setKeyPath(&self, key_path: Option<&NSString>);

        #[method(isAdditive)]
        pub unsafe fn isAdditive(&self) -> bool;

        #[method(setAdditive:)]
        pub unsafe fn setAdditive(&self, additive: bool);

        #[method(isCumulative)]
        pub unsafe fn isCumulative(&self) -> bool;

        #[method(setCumulative:)]
        pub unsafe fn setCumulative(&self, cumulative: bool);

        #[cfg(feature = "CAValueFunction")]
        #[method_id(@__retain_semantics Other valueFunction)]
        pub unsafe fn valueFunction(&self) -> Option<Retained<CAValueFunction>>;

        #[cfg(feature = "CAValueFunction")]
        #[method(setValueFunction:)]
        pub unsafe fn setValueFunction(&self, value_function: Option<&CAValueFunction>);
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CAPropertyAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAPropertyAnimation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cabasicanimation?language=objc)
    #[unsafe(super(CAPropertyAnimation, CAAnimation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CABasicAnimation;
);

#[cfg(feature = "CALayer")]
unsafe impl CAAction for CABasicAnimation {}

#[cfg(feature = "CAMediaTiming")]
unsafe impl CAMediaTiming for CABasicAnimation {}

unsafe impl NSCoding for CABasicAnimation {}

unsafe impl NSCopying for CABasicAnimation {}

unsafe impl CopyingHelper for CABasicAnimation {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CABasicAnimation {}

unsafe impl NSSecureCoding for CABasicAnimation {}

extern_methods!(
    unsafe impl CABasicAnimation {
        #[method_id(@__retain_semantics Other fromValue)]
        pub unsafe fn fromValue(&self) -> Option<Retained<AnyObject>>;

        #[method(setFromValue:)]
        pub unsafe fn setFromValue(&self, from_value: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other toValue)]
        pub unsafe fn toValue(&self) -> Option<Retained<AnyObject>>;

        #[method(setToValue:)]
        pub unsafe fn setToValue(&self, to_value: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other byValue)]
        pub unsafe fn byValue(&self) -> Option<Retained<AnyObject>>;

        #[method(setByValue:)]
        pub unsafe fn setByValue(&self, by_value: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `CAPropertyAnimation`
    unsafe impl CABasicAnimation {
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CABasicAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CABasicAnimation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cakeyframeanimation?language=objc)
    #[unsafe(super(CAPropertyAnimation, CAAnimation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAKeyframeAnimation;
);

#[cfg(feature = "CALayer")]
unsafe impl CAAction for CAKeyframeAnimation {}

#[cfg(feature = "CAMediaTiming")]
unsafe impl CAMediaTiming for CAKeyframeAnimation {}

unsafe impl NSCoding for CAKeyframeAnimation {}

unsafe impl NSCopying for CAKeyframeAnimation {}

unsafe impl CopyingHelper for CAKeyframeAnimation {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CAKeyframeAnimation {}

unsafe impl NSSecureCoding for CAKeyframeAnimation {}

extern_methods!(
    unsafe impl CAKeyframeAnimation {
        #[method_id(@__retain_semantics Other values)]
        pub unsafe fn values(&self) -> Option<Retained<NSArray>>;

        #[method(setValues:)]
        pub unsafe fn setValues(&self, values: Option<&NSArray>);

        #[method_id(@__retain_semantics Other keyTimes)]
        pub unsafe fn keyTimes(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[method(setKeyTimes:)]
        pub unsafe fn setKeyTimes(&self, key_times: Option<&NSArray<NSNumber>>);

        #[cfg(feature = "CAMediaTimingFunction")]
        #[method_id(@__retain_semantics Other timingFunctions)]
        pub unsafe fn timingFunctions(&self) -> Option<Retained<NSArray<CAMediaTimingFunction>>>;

        #[cfg(feature = "CAMediaTimingFunction")]
        #[method(setTimingFunctions:)]
        pub unsafe fn setTimingFunctions(
            &self,
            timing_functions: Option<&NSArray<CAMediaTimingFunction>>,
        );

        #[method_id(@__retain_semantics Other calculationMode)]
        pub unsafe fn calculationMode(&self) -> Retained<CAAnimationCalculationMode>;

        #[method(setCalculationMode:)]
        pub unsafe fn setCalculationMode(&self, calculation_mode: &CAAnimationCalculationMode);

        #[method_id(@__retain_semantics Other tensionValues)]
        pub unsafe fn tensionValues(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[method(setTensionValues:)]
        pub unsafe fn setTensionValues(&self, tension_values: Option<&NSArray<NSNumber>>);

        #[method_id(@__retain_semantics Other continuityValues)]
        pub unsafe fn continuityValues(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[method(setContinuityValues:)]
        pub unsafe fn setContinuityValues(&self, continuity_values: Option<&NSArray<NSNumber>>);

        #[method_id(@__retain_semantics Other biasValues)]
        pub unsafe fn biasValues(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[method(setBiasValues:)]
        pub unsafe fn setBiasValues(&self, bias_values: Option<&NSArray<NSNumber>>);

        #[method_id(@__retain_semantics Other rotationMode)]
        pub unsafe fn rotationMode(&self) -> Option<Retained<CAAnimationRotationMode>>;

        #[method(setRotationMode:)]
        pub unsafe fn setRotationMode(&self, rotation_mode: Option<&CAAnimationRotationMode>);
    }
);

extern_methods!(
    /// Methods declared on superclass `CAPropertyAnimation`
    unsafe impl CAKeyframeAnimation {
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CAKeyframeAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAKeyframeAnimation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaanimationlinear?language=objc)
    pub static kCAAnimationLinear: &'static CAAnimationCalculationMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaanimationdiscrete?language=objc)
    pub static kCAAnimationDiscrete: &'static CAAnimationCalculationMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaanimationpaced?language=objc)
    pub static kCAAnimationPaced: &'static CAAnimationCalculationMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaanimationcubic?language=objc)
    pub static kCAAnimationCubic: &'static CAAnimationCalculationMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaanimationcubicpaced?language=objc)
    pub static kCAAnimationCubicPaced: &'static CAAnimationCalculationMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaanimationrotateauto?language=objc)
    pub static kCAAnimationRotateAuto: &'static CAAnimationRotationMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaanimationrotateautoreverse?language=objc)
    pub static kCAAnimationRotateAutoReverse: &'static CAAnimationRotationMode;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caspringanimation?language=objc)
    #[unsafe(super(CABasicAnimation, CAPropertyAnimation, CAAnimation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CASpringAnimation;
);

#[cfg(feature = "CALayer")]
unsafe impl CAAction for CASpringAnimation {}

#[cfg(feature = "CAMediaTiming")]
unsafe impl CAMediaTiming for CASpringAnimation {}

unsafe impl NSCoding for CASpringAnimation {}

unsafe impl NSCopying for CASpringAnimation {}

unsafe impl CopyingHelper for CASpringAnimation {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CASpringAnimation {}

unsafe impl NSSecureCoding for CASpringAnimation {}

extern_methods!(
    unsafe impl CASpringAnimation {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(mass)]
        pub unsafe fn mass(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMass:)]
        pub unsafe fn setMass(&self, mass: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(stiffness)]
        pub unsafe fn stiffness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setStiffness:)]
        pub unsafe fn setStiffness(&self, stiffness: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(damping)]
        pub unsafe fn damping(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setDamping:)]
        pub unsafe fn setDamping(&self, damping: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(initialVelocity)]
        pub unsafe fn initialVelocity(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setInitialVelocity:)]
        pub unsafe fn setInitialVelocity(&self, initial_velocity: CGFloat);

        #[method(allowsOverdamping)]
        pub unsafe fn allowsOverdamping(&self) -> bool;

        #[method(setAllowsOverdamping:)]
        pub unsafe fn setAllowsOverdamping(&self, allows_overdamping: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(settlingDuration)]
        pub unsafe fn settlingDuration(&self) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithPerceptualDuration:bounce:)]
        pub unsafe fn initWithPerceptualDuration_bounce(
            this: Allocated<Self>,
            perceptual_duration: CFTimeInterval,
            bounce: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(perceptualDuration)]
        pub unsafe fn perceptualDuration(&self) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bounce)]
        pub unsafe fn bounce(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAPropertyAnimation`
    unsafe impl CASpringAnimation {
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CASpringAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CASpringAnimation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/catransition?language=objc)
    #[unsafe(super(CAAnimation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CATransition;
);

#[cfg(feature = "CALayer")]
unsafe impl CAAction for CATransition {}

#[cfg(feature = "CAMediaTiming")]
unsafe impl CAMediaTiming for CATransition {}

unsafe impl NSCoding for CATransition {}

unsafe impl NSCopying for CATransition {}

unsafe impl CopyingHelper for CATransition {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CATransition {}

unsafe impl NSSecureCoding for CATransition {}

extern_methods!(
    unsafe impl CATransition {
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<CATransitionType>;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: &CATransitionType);

        #[method_id(@__retain_semantics Other subtype)]
        pub unsafe fn subtype(&self) -> Option<Retained<CATransitionSubtype>>;

        #[method(setSubtype:)]
        pub unsafe fn setSubtype(&self, subtype: Option<&CATransitionSubtype>);

        #[method(startProgress)]
        pub unsafe fn startProgress(&self) -> c_float;

        #[method(setStartProgress:)]
        pub unsafe fn setStartProgress(&self, start_progress: c_float);

        #[method(endProgress)]
        pub unsafe fn endProgress(&self) -> c_float;

        #[method(setEndProgress:)]
        pub unsafe fn setEndProgress(&self, end_progress: c_float);

        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Option<Retained<AnyObject>>;

        #[method(setFilter:)]
        pub unsafe fn setFilter(&self, filter: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CATransition {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CATransition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatransitionfade?language=objc)
    pub static kCATransitionFade: &'static CATransitionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatransitionmovein?language=objc)
    pub static kCATransitionMoveIn: &'static CATransitionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatransitionpush?language=objc)
    pub static kCATransitionPush: &'static CATransitionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatransitionreveal?language=objc)
    pub static kCATransitionReveal: &'static CATransitionType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatransitionfromright?language=objc)
    pub static kCATransitionFromRight: &'static CATransitionSubtype;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatransitionfromleft?language=objc)
    pub static kCATransitionFromLeft: &'static CATransitionSubtype;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatransitionfromtop?language=objc)
    pub static kCATransitionFromTop: &'static CATransitionSubtype;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatransitionfrombottom?language=objc)
    pub static kCATransitionFromBottom: &'static CATransitionSubtype;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caanimationgroup?language=objc)
    #[unsafe(super(CAAnimation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAAnimationGroup;
);

#[cfg(feature = "CALayer")]
unsafe impl CAAction for CAAnimationGroup {}

#[cfg(feature = "CAMediaTiming")]
unsafe impl CAMediaTiming for CAAnimationGroup {}

unsafe impl NSCoding for CAAnimationGroup {}

unsafe impl NSCopying for CAAnimationGroup {}

unsafe impl CopyingHelper for CAAnimationGroup {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CAAnimationGroup {}

unsafe impl NSSecureCoding for CAAnimationGroup {}

extern_methods!(
    unsafe impl CAAnimationGroup {
        #[method_id(@__retain_semantics Other animations)]
        pub unsafe fn animations(&self) -> Option<Retained<NSArray<CAAnimation>>>;

        #[method(setAnimations:)]
        pub unsafe fn setAnimations(&self, animations: Option<&NSArray<CAAnimation>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CAAnimationGroup {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAAnimationGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
