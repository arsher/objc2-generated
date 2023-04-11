//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSAnimationCurve {
        NSAnimationEaseInOut = 0,
        NSAnimationEaseIn = 1,
        NSAnimationEaseOut = 2,
        NSAnimationLinear = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSAnimationBlockingMode {
        NSAnimationBlocking = 0,
        NSAnimationNonblocking = 1,
        NSAnimationNonblockingThreaded = 2,
    }
);

pub type NSAnimationProgress = c_float;

extern_static!(NSAnimationProgressMarkNotification: &'static NSNotificationName);

extern_static!(NSAnimationProgressMark: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSAnimation")]
    pub struct NSAnimation;

    #[cfg(feature = "AppKit_NSAnimation")]
    unsafe impl ClassType for NSAnimation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSAnimation")]
unsafe impl NSCoding for NSAnimation {}

#[cfg(feature = "AppKit_NSAnimation")]
unsafe impl NSCopying for NSAnimation {}

#[cfg(feature = "AppKit_NSAnimation")]
unsafe impl NSObjectProtocol for NSAnimation {}

extern_methods!(
    #[cfg(feature = "AppKit_NSAnimation")]
    unsafe impl NSAnimation {
        #[method_id(@__retain_semantics Init initWithDuration:animationCurve:)]
        pub unsafe fn initWithDuration_animationCurve(
            this: Option<Allocated<Self>>,
            duration: NSTimeInterval,
            animation_curve: NSAnimationCurve,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method(startAnimation)]
        pub unsafe fn startAnimation(&self);

        #[method(stopAnimation)]
        pub unsafe fn stopAnimation(&self);

        #[method(isAnimating)]
        pub unsafe fn isAnimating(&self) -> bool;

        #[method(currentProgress)]
        pub unsafe fn currentProgress(&self) -> NSAnimationProgress;

        #[method(setCurrentProgress:)]
        pub unsafe fn setCurrentProgress(&self, current_progress: NSAnimationProgress);

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(setDuration:)]
        pub unsafe fn setDuration(&self, duration: NSTimeInterval);

        #[method(animationBlockingMode)]
        pub unsafe fn animationBlockingMode(&self) -> NSAnimationBlockingMode;

        #[method(setAnimationBlockingMode:)]
        pub unsafe fn setAnimationBlockingMode(
            &self,
            animation_blocking_mode: NSAnimationBlockingMode,
        );

        #[method(frameRate)]
        pub unsafe fn frameRate(&self) -> c_float;

        #[method(setFrameRate:)]
        pub unsafe fn setFrameRate(&self, frame_rate: c_float);

        #[method(animationCurve)]
        pub unsafe fn animationCurve(&self) -> NSAnimationCurve;

        #[method(setAnimationCurve:)]
        pub unsafe fn setAnimationCurve(&self, animation_curve: NSAnimationCurve);

        #[method(currentValue)]
        pub unsafe fn currentValue(&self) -> c_float;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSAnimationDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSAnimationDelegate>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other progressMarks)]
        pub unsafe fn progressMarks(&self) -> Id<NSArray<NSNumber>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method(setProgressMarks:)]
        pub unsafe fn setProgressMarks(&self, progress_marks: &NSArray<NSNumber>);

        #[method(addProgressMark:)]
        pub unsafe fn addProgressMark(&self, progress_mark: NSAnimationProgress);

        #[method(removeProgressMark:)]
        pub unsafe fn removeProgressMark(&self, progress_mark: NSAnimationProgress);

        #[method(startWhenAnimation:reachesProgress:)]
        pub unsafe fn startWhenAnimation_reachesProgress(
            &self,
            animation: &NSAnimation,
            start_progress: NSAnimationProgress,
        );

        #[method(stopWhenAnimation:reachesProgress:)]
        pub unsafe fn stopWhenAnimation_reachesProgress(
            &self,
            animation: &NSAnimation,
            stop_progress: NSAnimationProgress,
        );

        #[method(clearStartAnimation)]
        pub unsafe fn clearStartAnimation(&self);

        #[method(clearStopAnimation)]
        pub unsafe fn clearStopAnimation(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other runLoopModesForAnimating)]
        pub unsafe fn runLoopModesForAnimating(&self) -> Option<Id<NSArray<NSRunLoopMode>>>;
    }
);

extern_protocol!(
    pub unsafe trait NSAnimationDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSAnimation")]
        #[optional]
        #[method(animationShouldStart:)]
        unsafe fn animationShouldStart(&self, animation: &NSAnimation) -> bool;

        #[cfg(feature = "AppKit_NSAnimation")]
        #[optional]
        #[method(animationDidStop:)]
        unsafe fn animationDidStop(&self, animation: &NSAnimation);

        #[cfg(feature = "AppKit_NSAnimation")]
        #[optional]
        #[method(animationDidEnd:)]
        unsafe fn animationDidEnd(&self, animation: &NSAnimation);

        #[cfg(feature = "AppKit_NSAnimation")]
        #[optional]
        #[method(animation:valueForProgress:)]
        unsafe fn animation_valueForProgress(
            &self,
            animation: &NSAnimation,
            progress: NSAnimationProgress,
        ) -> c_float;

        #[cfg(feature = "AppKit_NSAnimation")]
        #[optional]
        #[method(animation:didReachProgressMark:)]
        unsafe fn animation_didReachProgressMark(
            &self,
            animation: &NSAnimation,
            progress: NSAnimationProgress,
        );
    }

    unsafe impl ProtocolType for dyn NSAnimationDelegate {}
);

typed_enum!(
    pub type NSViewAnimationKey = NSString;
);

extern_static!(NSViewAnimationTargetKey: &'static NSViewAnimationKey);

extern_static!(NSViewAnimationStartFrameKey: &'static NSViewAnimationKey);

extern_static!(NSViewAnimationEndFrameKey: &'static NSViewAnimationKey);

extern_static!(NSViewAnimationEffectKey: &'static NSViewAnimationKey);

typed_enum!(
    pub type NSViewAnimationEffectName = NSString;
);

extern_static!(NSViewAnimationFadeInEffect: &'static NSViewAnimationEffectName);

extern_static!(NSViewAnimationFadeOutEffect: &'static NSViewAnimationEffectName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSViewAnimation")]
    pub struct NSViewAnimation;

    #[cfg(feature = "AppKit_NSViewAnimation")]
    unsafe impl ClassType for NSViewAnimation {
        #[inherits(NSObject)]
        type Super = NSAnimation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSViewAnimation")]
unsafe impl NSCoding for NSViewAnimation {}

#[cfg(feature = "AppKit_NSViewAnimation")]
unsafe impl NSCopying for NSViewAnimation {}

#[cfg(feature = "AppKit_NSViewAnimation")]
unsafe impl NSObjectProtocol for NSViewAnimation {}

extern_methods!(
    #[cfg(feature = "AppKit_NSViewAnimation")]
    unsafe impl NSViewAnimation {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Init initWithViewAnimations:)]
        pub unsafe fn initWithViewAnimations(
            this: Option<Allocated<Self>>,
            view_animations: &NSArray<NSDictionary<NSViewAnimationKey, Object>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Other viewAnimations)]
        pub unsafe fn viewAnimations(
            &self,
        ) -> Id<NSArray<NSDictionary<NSViewAnimationKey, Object>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method(setViewAnimations:)]
        pub unsafe fn setViewAnimations(
            &self,
            view_animations: &NSArray<NSDictionary<NSViewAnimationKey, Object>>,
        );
    }
);

pub type NSAnimatablePropertyKey = NSString;

extern_protocol!(
    pub unsafe trait NSAnimatablePropertyContainer {
        #[method_id(@__retain_semantics Other animator)]
        unsafe fn animator(&self) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other animations)]
        unsafe fn animations(&self) -> Id<NSDictionary<NSAnimatablePropertyKey, Object>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setAnimations:)]
        unsafe fn setAnimations(&self, animations: &NSDictionary<NSAnimatablePropertyKey, Object>);

        #[method_id(@__retain_semantics Other animationForKey:)]
        unsafe fn animationForKey(&self, key: &NSAnimatablePropertyKey) -> Option<Id<Object>>;

        #[method_id(@__retain_semantics Other defaultAnimationForKey:)]
        unsafe fn defaultAnimationForKey(key: &NSAnimatablePropertyKey) -> Option<Id<Object>>;
    }

    unsafe impl ProtocolType for dyn NSAnimatablePropertyContainer {}
);

extern_static!(NSAnimationTriggerOrderIn: &'static NSAnimatablePropertyKey);

extern_static!(NSAnimationTriggerOrderOut: &'static NSAnimatablePropertyKey);

extern_methods!(
    /// Methods declared on superclass `NSAnimation`
    #[cfg(feature = "AppKit_NSViewAnimation")]
    unsafe impl NSViewAnimation {
        #[method_id(@__retain_semantics Init initWithDuration:animationCurve:)]
        pub unsafe fn initWithDuration_animationCurve(
            this: Option<Allocated<Self>>,
            duration: NSTimeInterval,
            animation_curve: NSAnimationCurve,
        ) -> Id<Self>;
    }
);
